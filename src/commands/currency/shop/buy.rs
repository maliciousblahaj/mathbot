use mathbot::{command::CommandParams, error::ClientError, model::item::{ItemController, ItemQueryKey}, send_embed, send_help, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};
use serenity::all::CreateMessage;

pub async fn buy(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let Some(name_id) = params.args.get(0) else {return send_help(params).await;};
    let count = match params.args.get(1) {
        Some(c) => match c.parse::<u64>() {
            Ok(c) => c,
            Err(_) => 1,
        },
        None => 1,
    };
    let ic = ItemController::new(params.state.get_model_controller(),
     ItemQueryKey::name_id_incomplete(name_id.to_string()));

    let (item, price) = match ic.fetch_item().await.ok() {
        Some(item) => {
            let Some(price) = item.price else {return Err(Error::Client(ClientError::ShopBuyItemNotFound(name_id.to_string())));};
            (item, price)
        },
        None => {return Err(Error::Client(ClientError::ShopBuyItemNotFound(name_id.to_string())));},
    };

    let cost = price*(count as i64);
    if cost > (account.balance as i64) {
        return Err(Error::Client(ClientError::ShopBuyInsufficientFunds));
    }

    let message = CreateMessage::new().embed(
        base_embed(&EmbedCtx::from_account(&account), ColorType::Currency)
            .title("Confirm purchase")
            .description(format!("Are you sure you want to buy `{count}` {} for `{cost}MTC$`?", item.display_name))
        );
    if !params.await_confirmation(message).await? {
        return Ok(());
    }

    let count = count as i64;
    sqlx::query!("UPDATE Accounts SET balance = balance - ? WHERE id=?; 
    INSERT OR IGNORE INTO Inventory (account_id, item_id, count) VALUES (?, ?, 0);
    UPDATE Inventory SET count = count + ? WHERE account_id=? AND item_id=?", 
        cost, account.id, account.id, item.id, count, account.id, item.id)
        .execute(params.state.get_model_controller().get_database())
        .await
        .map_err(|e| Error::FailedToBuyItems(e))?;

    send_embed(
        base_embed(&EmbedCtx::from_account(account), ColorType::Success)
            .title("Success")
            .description(format!("Successfully bought `{count}` {} for `{cost}MTC$`", item.display_name)),
        &SendCtx::from_params(&params)
    ).await?;
    Ok(())
}
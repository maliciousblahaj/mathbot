use mathbot::{command::CommandParams, error::ClientError, send_embed, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};
use serenity::{all::CreateMessage, futures::sink::Send};

pub async fn buy(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    
    let price = super::get_slot_price(account.mine_slots as usize);
    if &(account.balance as i64) < &price {
        return Err(Error::Client(ClientError::SlotBuyInsufficientFunds));
    }

    let message = CreateMessage::new().embed(
        base_embed(&EmbedCtx::from_account(account), ColorType::Currency)
            .title("Confirmation")
            .description(format!("Are you sure you want to purchase slot {} for `{price}MTC$`?", account.mine_slots+1))
    );
    if !params.await_confirmation(message).await? {
        return Ok(());
    }

    sqlx::query!("
        UPDATE Accounts SET balance = balance - ?, mine_slots = mine_slots + 1 WHERE id=?; 
        INSERT INTO Slots(account_id) VALUES(?);
    ", price, account.id, account.id)
        .execute(params.state.get_model_controller().get_database())
        .await
        .map_err(Error::FailedToBuySlot)?;

    send_embed(
        base_embed(&EmbedCtx::from_account(account), ColorType::Success)
            .title("Success")
            .description(format!("Successfully purchased slot {} for `{}MTC$`", account.mine_slots+1, price)),
    &SendCtx::from_params(&params)
    ).await?;

    Ok(())
}
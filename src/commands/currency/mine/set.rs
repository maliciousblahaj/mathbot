use mathbot::{command::CommandParams, error::ClientError, model::{account::{AccountController, AccountQueryKey, MineClaimType}, item::{ItemController, ItemQueryKey}}, send_embed, send_help, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};
use serenity::all::Embed;

 
//TODO: tidy up this awfully written spaghetti command
pub async fn set(params: CommandParams) -> Result<()> { 
    let account = params.require_account()?; 
    let (Some(Some(slot)), Some(itemid)) =  
        ( 
            params.args.get(0).map(|s| s.parse::<usize>().ok()), 
            params.args.get(1)
        )
        else {return send_help(params).await;};
    let mut slot = if slot == 0 {0} else {slot-1};
    if slot >= account.mine_slots as usize {return Err(Error::Client(ClientError::MineSlotNotOwned));}
    
    let mc = params.state.get_model_controller();

    let item = ItemController::new(
        mc, 
        ItemQueryKey::name_id_incomplete(itemid.clone())
    ).fetch_item().await.map_err(|_| Error::Client(ClientError::MineSetInvalidItemId))?;
    let mut ac = AccountController::new(
        mc, AccountQueryKey::id(account.id)
    );
    if ac.get_item_count(item.id).await? < 1 {
        return Err(Error::Client(ClientError::MineSetItemNotOwned));
    }
    let slots = ac.fetch_mine().await?;
    let slotid = slots[slot].id;
    let prev_item_id = sqlx::query!(
        "SELECT item_id FROM Slots WHERE id=?", slotid
    ).fetch_one(mc.get_database())
        .await
        .map(|a| a.item_id)
        .map_err(|e| Error::FailedToGetPreviousSlotItem(e))?;

    ac.claim_mine(MineClaimType::AwaitingClaim, 0.0).await?;

    sqlx::query!("
        UPDATE Slots SET item_id=? WHERE id=?;
        UPDATE Inventory SET count = count - 1 WHERE account_id=? AND item_id=?;

        INSERT OR IGNORE INTO Inventory (account_id, item_id, count) VALUES (?, ?, 0);
        UPDATE Inventory SET count = count + 1 WHERE account_id=? 
            AND (item_id = ? AND ? IS NOT NULL);",
        item.id, slotid, account.id, item.id, account.id, prev_item_id, account.id, prev_item_id, prev_item_id
    ).execute(mc.get_database())
        .await
        .map_err(|e| Error::FailedToSetSlotItem(e))?;

    send_embed(
        base_embed(&EmbedCtx::from_account(account), ColorType::Success)
            .title("Success")
            .description(format!("Successfully set {}**{}** in mine slot {}", 
                if let Some(emoji_id) = item.emoji_id {emoji_id} else {String::new()} , item.display_name, slot+1)),
        &SendCtx::from_params(&params)
    ).await?;

    Ok(())
}
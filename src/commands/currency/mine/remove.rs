use mathbot::{command::CommandParams, error::ClientError, model::account::{AccountController, AccountQueryKey, MineClaimType}, send_embed, send_help, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};

pub async fn remove(params: CommandParams) -> Result<()> {
    let account = params.require_account()?; 
    let Some(Some(slot)) = params.args.get(0).map(|s| s.parse::<usize>().ok())
        else {return send_help(params).await;};
    let mut slot = if slot == 0 {0} else {slot-1};
    if slot >= account.mine_slots as usize {return Err(Error::Client(ClientError::MineSlotNotOwned));}


    let mc = params.state.get_model_controller();

    let mut ac = AccountController::new(
        mc, AccountQueryKey::id(account.id)
    );

    let slots = ac.fetch_mine().await?;
    let slotid = slots[slot].id;

    let prev_item = sqlx::query!(
        "SELECT 
            Slots.item_id,
            Items.emoji_id, 
            Items.display_name
        FROM Slots 
            LEFT JOIN Items ON Items.id = Slots.item_id
        WHERE Slots.id=?", slotid
    ).fetch_one(mc.get_database())
        .await
        .map_err(|e| Error::FailedToGetPreviousSlotItem(e))?;

    let (Some(id), emoji_id, Some(display_name)) = (prev_item.item_id, prev_item.emoji_id, prev_item.display_name)
        else {return Err(Error::Client(ClientError::MineRemoveNothingToRemove));};

    ac.claim_mine(MineClaimType::AwaitingClaim, 0.0).await?;

    sqlx::query!("
    UPDATE Slots SET item_id=NULL WHERE id=?;
    INSERT OR IGNORE INTO Inventory (account_id, item_id, count) VALUES (?, ?, 0);
    UPDATE Inventory SET count = count + 1 WHERE account_id=? 
        AND item_id = ?;",
    slotid, account.id, id, account.id, id
    ).execute(mc.get_database())
        .await
        .map_err(|e| Error::FailedToRemoveSlotItem(e))?;




    send_embed(
        base_embed(&EmbedCtx::from_account(account), ColorType::Success)
            .title("Success")
            .description(format!("Successfully removed {}**{}** from mine slot {}", 
                if let Some(em_id) = emoji_id {em_id} else {String::new()} , display_name, slot+1)),
        &SendCtx::from_params(&params)
    ).await?;


    Ok(())
}
/* case "buy":
                if len(args) < 2:
                    await help(ctx,"shop","buy")
                    return
                if len(args) < 3:
                    count = 1
                else:
                    if args[2].isdigit() == False:
                        count = 1
                    else:
                        count = int(args[2])
                try:
                    item = Item(Globals.devdb,Globals.devdb.getItemId(Globals.devdb.getFullItemName(args[1])))
                    if item.forsale == False:
                        raise Exception()
                    if count < 1:
                        await ctx.send(embed=Embed.ErrorEmbed(ctx.author.id,"shopinvalidcount",str(count)))
                        return
                    if authorAccount.balance < item.price*count:
                        await ctx.send(embed=Embed.ErrorEmbed(ctx.author.id,"shopnotenoughmoney",str(item.price*count),str(count),item.emojiid,item.name))
                        return
                    if ctx.author.id in Globals.buttondict.keys():
                        await Globals.buttondict[ctx.author.id].KillButtons()
                    Globals.buttondict[ctx.author.id] = ConfirmShopBuyPage(ctx.author.id,item,count)
                    msg = await ctx.send(embed=Embed.BaseEmbed(ctx.author.id,title="Confirmation",description=f"Are you sure you want to buy `{count}` {item.emojiid} **{item.name}** for `{item.price*count}MTC$`?",colorid="userinfo"), view=Embed.ConfirmationView())
                    Globals.buttondict[ctx.author.id].message = msg
                    return
                    
                except Exception as e:
                    print(e)
                    await ctx.send(embed=Embed.ErrorEmbed(ctx.author.id,"shopitemdoesntexist", args[1]))
                    return
*/

use mathbot::{command::CommandParams, error::ClientError, model::item::{ItemController, ItemQueryKey}, send_embed, send_help, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};

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

    //TODO: add confirmation

    let count = count as i64;
    sqlx::query!("UPDATE Accounts SET balance = balance - ? WHERE id=?; 
    INSERT OR IGNORE INTO Inventory (account_id, item_id, count) VALUES (?, ?, 0);
    UPDATE Inventory SET count = count + ? WHERE account_id=?", cost, account.id, account.id, item.id, count, account.id)
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
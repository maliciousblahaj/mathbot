use std::thread::current;

use mathbot::{command::CommandParams, model::{account::Account, item::Item, ShopItem}, ui::{embed::{base_embed, ButtonEmoji, ColorType, EmbedCtx}, ButtonInfo, ButtonMessage, PageMessage}, Result};
use serenity::all::{ButtonStyle, CreateButton, CreateEmbed, CreateMessage, EmbedField, MessageBuilder};
/*
def ShopEmbed(userid: int, page: int):
    from Item import Item
    embed = Embed.BaseEmbed(userid,"The MathBot Shop","Use `!shop buy {itemid} /{count}` to buy items")
    items = Globals.devdb.getShopItems()
    for item in items[page*6-6:page*6]:
        itemob = Item(Globals.devdb,item)
        value = f"Price: `{itemob.price}MTC$`\nItem id: `{itemob.nameid}`"
        if itemob.itemtype == "graphicscard":
            value += f"\nMTC$/h: `{itemob.mps*3600}`"
        embed.add_field(name=itemob.emojiid + " **" + itemob.name+ "**", value=value, inline=True)
    return embed
*/

pub async fn shop(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let shop = params.state.get_model_controller().get_shop().await?;
    let shoppages = (shop.len()-1)/6 + 1;
    let mut currentpage = 1;

    //i wish let chains existed in stable rust ;-;
    if let Some(input) = params.args.get(0) {
        if let Ok(page) = input.parse::<usize>() {
            if page <= shoppages {
                currentpage = page;
            }
        }
    }


    let mut message = PageMessage::new(
        &params, shop, shoppages, currentpage, shop_embed, account.clone(), 
    );
    while let Some(_) = message.send().await?.run(30).await? {}

    Ok(())
}

fn get_disabled(button: &str, pages: &usize, page: &usize) -> bool {
    match button {
        "first" => false,
        "previous" => false,
        "next" => false,
        "last" => false,
        _ => false,
    }
}

fn shop_embed(params: &CommandParams, account: &Account, shop: &Vec<ShopItem>, page: &usize) -> CreateEmbed {
    let mut fields = Vec::new();
    for item in &shop[page*6-6..page*6] {
        let mut fieldvalue = format!("Price: `{}MTC$`\nItem id: `{}`", item.price.unwrap_or(0), item.name_id);
        if let Some(mps) = item.mps {
            fieldvalue.push_str(format!("\nMTC$/h: `{:.1}`", mps*3600.0).as_str());
        }
        fields.push(
            (format!("{}**{}**", match &item.emoji_id { Some(e) => format!("{e} "), None => String::new()}, item.display_name),
            fieldvalue,
            true)
        );
    }
    base_embed(&EmbedCtx::from_account(account), ColorType::Currency)
        .title("The MathBot Shop")
        .description(format!("Use `{}shop buy {{itemid}} /{{count}}` to buy items", params.bot_prefix))
        .fields(fields)
}
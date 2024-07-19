use std::thread::current;

use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandParams}, model::{account::Account, item::Item, ShopItem}, ui::{embed::{base_embed, ButtonEmoji, ColorType, EmbedCtx}, ButtonInfo, ButtonMessage, PageMessage}, vec_of_strings, Result};
use serenity::all::{ButtonStyle, CreateButton, CreateEmbed, CreateMessage, EmbedField, MessageBuilder};

mod buy;

async fn shop(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let shop = params.state.get_model_controller().get_shop().await?;
    let shoppages = match shop.len() {
        0 => 0,
        len => (len-1)/6 + 1
    };
    let mut currentpage = 1;

    //i wish let chains existed in stable rust ;-;
    if let Some(input) = params.args.get(0) {
        if let Ok(page) = input.parse::<usize>() {
            if page >= 1 && page <= shoppages {
                currentpage = page;
            }
        }
    }


    let mut message = PageMessage::new(
        &params, shop, shoppages, currentpage, shop_embed, account.clone(), 
    );
    let message = message.send().await?;
    while let Some(_) = message.run(30).await? {}

    Ok(())
}

fn shop_embed(params: &CommandParams, account: &Account, shop: &Vec<ShopItem>, page: &usize) -> CreateEmbed {
    let mut fields = Vec::new();
    let mut page = page.clone();
    if page == 0 || page*6-6 >= shop.len() {
        page = 1;
    }
    let mut iter = shop.iter().skip(page*6-6).take(6);
    while let Some(item) = iter.next() {
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
        .title(format!("The MathBot Shop, page {page}"))
        .description(format!("Use `{}shop buy {{itemid}} {{count?}}` to buy items", params.bot_prefix))
        .fields(fields)
}

pub fn command() -> Result<Command> {
    let category = CommandCategory::Currency;
    Ok(
    Command::new(
        shop,
        vec_of_strings!("shop", "store"),
        category.clone(),
        CommandHelp::new("View the MathBot shop (please buy something)", ""),
    ).register_single(
        Command::new(
            buy::buy,
            vec_of_strings!("buy"),
            category.clone(),
            CommandHelp::new("Buy an item from the shop", " {id} {amount?}"),
        )
    )?
    )
}
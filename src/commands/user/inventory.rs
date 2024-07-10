use mathbot::{command::CommandParams, model::account::{Account, AccountController, AccountQueryKey, InventoryItem}, ui::{embed::{base_embed, ColorType, EmbedCtx}, PageMessage}};
use mathbot::Result;
use serenity::all::CreateEmbed;

pub async fn inventory(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let inventory = AccountController::new(
        params.state.get_model_controller(), AccountQueryKey::id(account.id)).fetch_inventory().await?;
    let inventorypages = match inventory.len() {
        0 => 0,
        len => (len-1)/10 + 1
    };
    let mut currentpage = 1;

    if let Some(input) = params.args.get(0) {
        if let Ok(page) = input.parse::<usize>() {
            if page >= 1 && page <= inventorypages {
                currentpage = page;
            }
        }
    }

    let mut message = PageMessage::new(
        &params, inventory, inventorypages, currentpage, inventory_embed, account.clone(), 
    );
    let message = message.send().await?;
    while let Some(_) = message.run(30).await? {}

    Ok(())
}

fn inventory_embed(params: &CommandParams, account: &Account, inventory: &Vec<InventoryItem>, page: &usize) -> CreateEmbed {
    let mut page = page.clone();
    if page == 0 || page*10-10 >= inventory.len() {
        page = 1;
    }
    let mut description = match inventory.len() {
        0 => String::from("This inventory is empty"),
        _ => String::new(),
    };
    let mut iter = inventory.iter().skip(page*10-10).take(10);
    while let Some(item) = iter.next() {
        let field = format!("\n{} **{}** `x{}`", 
            item.emoji_id.clone().unwrap_or(String::new()),
            item.display_name,
            item.count,
        );
        description.push_str(field.as_str());
    }
    description.strip_prefix("\n");
    base_embed(&EmbedCtx::from_account(account), ColorType::UserInfo)
        .title(format!("@{}'s inventory, page {page}", account.username))
        .description(description)
}
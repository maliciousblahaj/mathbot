use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandParams}, model::account::{Account, AccountController, AccountQueryKey, MineItem}, ui::{embed::{base_embed, ColorType, EmbedCtx}, PageMessage}, vec_of_strings, Result};
use serenity::all::CreateEmbed;

async fn mine(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let mine = AccountController::new(
        params.state.get_model_controller(), AccountQueryKey::id(account.id)).fetch_mine().await?;
    let minepages = match mine.len() {
        0 => 0,
        len => (len-1)/6 + 1
    };
    let mut currentpage = 1;

    if let Some(input) = params.args.get(0) {
        if let Ok(page) = input.parse::<usize>() {
            if page >= 1 && page <= minepages {
                currentpage = page;
            }
        }
    }

    let mut message = PageMessage::new(
        &params, mine, minepages, currentpage, mine_embed, account.clone(), 
    );
    let message = message.send().await?;
    while let Some(_) = message.run(30).await? {}

    Ok(())
}

fn mine_embed(params: &CommandParams, account: &Account, mine: &Vec<MineItem>, page: &usize) -> CreateEmbed {
    let mut fields = Vec::new();
    let mut page = page.clone();
    if page == 0 || page*6-6 >= mine.len() {
        page = 1;
    }
    let totalmps: f64 = mine.iter().filter_map(|item| item.mps?).sum();
    
    /*{
        let mut mps = 0.0;
        for item in mine {
            if let Some(itemmps) = item.mps {
                mps += itemmps;
            }
        }
    };*/

    for i in page*6-6..page*6 {
        let fieldvalue = match mine.get(i) {
            Some(mineitem) => {
                if let (Some(item_id), Some(emoji_id), Some(display_name), Some(Some(mps))) = (&mineitem.item_id, &mineitem.emoji_id, &mineitem.display_name, &mineitem.mps) {
                    format!("{}**{}**\n`{} MTC$/h`", 
                        if let Some(emoji_str) = emoji_id {format!("{emoji_str} ")} else {String::new()},
                        display_name,
                        mps*3600.0,
                    )
                } else {
                    "Empty\n`0 MTC$/h`".to_string()
                }
            },
            None => format!("Locked{}", //TODO: implement slot prices
                if (account.mine_slots as usize) == i {format!("\nCost: `{}MTC$`", "TODO")} else {String::new()}
            )
        };

        fields.push(
            (format!("Slot {}", i+1),
            fieldvalue,
            true)
        );
    }
    base_embed(&EmbedCtx::from_account(account), ColorType::Currency)
        .title(format!("@{}'s mining facility, page {page}", account.username))
        .description(format!("Total production: `{:.1}MTC$/h`", totalmps * 3600.0))
        .fields(fields)
}

pub fn command() -> Result<Command> {
    let category = CommandCategory::Currency;
    Ok(
    Command::new(
        mine,
        vec_of_strings!("mine", "min"),
        category.clone(),
        CommandHelp::new("Look what you have in your mining facility", "")
    )
    )
}
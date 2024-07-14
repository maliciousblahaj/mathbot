use mathbot::{command::{Command, CommandCategory, CommandHelp, CommandParams}, send_embed, send_text, ui::embed::{base_embed, ColorType, EmbedCtx}, vec_of_strings, Result, SendCtx};

mod ban;
mod unban;

pub async fn admin(params: CommandParams) -> Result<()> {
    let Some(account) = params.require_admin() else {return Ok(());};

    send_embed(base_embed(&EmbedCtx::from_account(account), ColorType::Admin).title("Admin panel").description("unfinished"), &SendCtx::from_params(&params)).await?;
    Ok(())
}

pub fn command() -> Result<Command> {
    let category = CommandCategory::Admin;
    Ok(
    Command::new(
        admin,
        vec_of_strings!("admin"),
        category.clone(),
        CommandHelp::new("Bully MathBot users by modifying their data", ""),
    ).register(vec![
        Command::new(
            ban::ban,
            vec_of_strings!("ban"),
            category.clone(),
            CommandHelp::new("Ban a MathBot account for a certain amount of time", " {account} {time (default seconds)} {units}")
        ),
        Command::new(
            unban::unban,
            vec_of_strings!("unban"),
            category.clone(),
            CommandHelp::new("Unban a MathBot account", " {account}")
        ),
    ])?
    )
}
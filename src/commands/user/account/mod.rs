use mathbot::error::ClientError;
use mathbot::model::account::Account;
use mathbot::ui::embed::{base_embed, base_embed_no_author, EmbedCtx};
use mathbot::ui::{ButtonInfo, ButtonMessage};
use mathbot::command::{Command, CommandCategory, CommandHelp, CommandParams};
use mathbot::{send_embed, vec_of_strings, Error, Result, SendCtx};
use mathbot::ui::embed::ColorType;
use serenity::all::{ButtonStyle, Color, CreateButton, CreateEmbed, CreateMessage};
use serenity::futures::TryFutureExt;

mod create;

async fn account(params: CommandParams) -> Result<()> {
    let Some(account) = &params.account else {return Err(Error::Client(ClientError::AccountRequired(params.bot_prefix)));};

    send_embed(profile_embed(&EmbedCtx::from_params(&params), account), &SendCtx::from_params(&params)).await?;
    Ok(())
}

fn profile_embed(ctx: &EmbedCtx, account: &Account) -> CreateEmbed {
    let mut embed = base_embed(ctx, ColorType::UserInfo)
        .title(format!("@{}'s account", account.username))
        .thumbnail(account.avatar_url.clone());
    let mut embedfields = Vec::new();

    if let Some(pronouns) = account.pronouns.clone() {
        embedfields.push(("Pronouns", format!("`{pronouns}`"), false));
    }
    if let Some(user_bio) = account.user_bio.clone() {
        embedfields.push(("Bio", format!("```{user_bio}```"), false));
    }
    /*
            embed.add_field(name="Info", value=f"Total balance: `{self.balance} MTC$`", inline=True)
        embed.add_field(name="Stats", value=f"Total SMP's solved: `{self.smps}`", inline=True)
        embed.add_field(name="Account info", value=f"Account created: <t:{self.created}:D>", inline=True)*/
    embedfields.append(vec![
        ("Info", format!("Total balance: `{} MTC$`", account.balance), true),
        ("Stats", format!("Total SMP's solved: `{}`", account.smps_solved), true),
        ("Account info", format!("Account created: <t:{}:D>", account.created), true),
    ].as_mut());
    embed.fields(embedfields)
}


pub fn command() -> Result<Command> {
    let category = CommandCategory::User;
    Ok(
    Command::new(
        account,
        vec_of_strings!("account", "a", "p", "profile"),
        category.clone(),
        CommandHelp::new("Look up info about your or someone else's account", " /{account}"),
    )
        .register(
            vec![
                Command::new(
                    create::create,
                    vec_of_strings!("create"),
                    category.clone(),
                    CommandHelp::new("Create your own account if you don't already have one", ""),
                ),
            ]
        )?
    )
}

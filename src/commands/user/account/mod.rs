use mathbot::error::ClientError;
use mathbot::model::account::Account;
use mathbot::ui::embed::{base_embed, base_embed_no_author, EmbedCtx};
use mathbot::ui::{ButtonInfo, ButtonMessage};
use mathbot::command::{Command, CommandCategory, CommandHelp, CommandParams};
use mathbot::{format_f64, send_embed, vec_of_strings, Error, Result, SendCtx};
use mathbot::ui::embed::ColorType;
use serenity::all::{ButtonStyle, Color, CreateButton, CreateEmbed, CreateMessage};
use serenity::futures::TryFutureExt;

mod create;
mod delete;
mod update_bio;
mod update_username;
mod update_pronouns;
mod update_avatar;

async fn account(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let mut viewaccount = account.clone();

    if let Some(user) = params.args.get(0) {
        if let Some(account) = params.get_account_by_user_input(user).await {
            viewaccount = account;
        }
    }

    send_embed(profile_embed(&EmbedCtx::from_account(account), &viewaccount), &SendCtx::from_params(&params)).await?;
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

    embedfields.append(vec![
        ("Info", format!("Total balance: `{} MTC$`", format_f64(&account.balance)), true),
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
        CommandHelp::new("Look up info about your or someone else's account", " /{username}"),
    )
        .register(
            vec![
                Command::new(
                    create::create,
                    vec_of_strings!("create"),
                    category.clone(),
                    CommandHelp::new("Create your own account if you don't already have one", ""),
                ),
                Command::new(
                    delete::delete,
                    vec_of_strings!("delete"),
                    category.clone(),
                    CommandHelp::new("Delete your MathBot©™ account (NOT RECOMMENDED)", ""),
                ),
                Command::new(
                    update_username::update_username,
                    vec_of_strings!("update_username"),
                    category.clone(),
                    CommandHelp::new("Update your MathBot username. Usernames can be 2-20 characters long, and no whitespaces are allowed", " {new username}"),
                ),
                Command::new(
                    update_bio::update_bio,
                    vec_of_strings!("update_bio"),
                    category.clone(),
                    CommandHelp::new("Update your account bio, or specify remove as an argument to remove your current one", " {new bio/remove}"),
                ),
                Command::new(
                    update_avatar::update_avatar,
                    vec_of_strings!("update_avatar"),
                    category.clone(),
                    CommandHelp::new("Update your avatar shown on your profile. You can either attach an image to your message or paste an image URL", " /{new avatar url}"),
                ),
                Command::new(
                    update_pronouns::update_pronouns,
                    vec_of_strings!("update_pronouns"),
                    category.clone(),
                    CommandHelp::new("Update your pronouns shown on your profile. Pronouns must be 3-20 characters long, can only contain letters, and must be in the correct format. To remove your pronouns, specify 'remove' as an argument", " {new pronouns}"),
                ),
            ]
        )?
    )
}

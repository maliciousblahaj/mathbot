
/*
case "update_avatar":
    if len(args) == 1:
        if len(ctx.message.attachments) == 0:
            await help(ctx,"account","update_avatar")
            return
        url = ctx.message.attachments[0].url
    else:
        url = args[1]
    if validAvatarurl(url) == False:
        await ctx.send(embed=Embed.ErrorEmbed(ctx.author.id,"invalidavatarurl"))
        return
    authoracc.avatarurl = url
    await ctx.send(embed=Embed.BaseEmbed(ctx.author.id,title="Successfully updated your avatar",colorid="success"))
    return
*/

use mathbot::{command::CommandParams, error::ClientError, send_embed, send_help, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};

pub async fn update_avatar(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let url = match params.msg.attachments.get(0) {
        Some(attachment) => attachment.url.to_string(),
        None => {
            let Some(url) = params.args.get(0) else {return send_help(params).await;};
            url.to_string()
        }
    };
    if !valid_avatar_url(url.as_str()).await? {
        return Err(Error::Client(ClientError::UpdateAvatarInvalidAvatarUrl(url)));
    }

    sqlx::query!("UPDATE Accounts SET avatar_url=? WHERE id=?", url, account.id)
        .execute(&params.state.get_model_controller().get_database().clone())
        .await
        .map_err(|e| Error::FailedToUpdateAccountAvatar(e))?;

    let embed = base_embed(&EmbedCtx::from_account(account), ColorType::Success)
        .title("Success")
        .description("Successfully updated your avatar");
    send_embed(embed, &SendCtx::from_params(&params)).await?;
    Ok(())
}

async fn valid_avatar_url(url: &str) -> Result<bool> {
    let get = reqwest::get(url).await
        .map_err(|e| Error::FailedToGetAvatarUrl(e))?;
    let Some(content_type) = get
        .headers()
        .get("content-type")
        else {return Ok(false);};
    match content_type.to_str().map_err(|e| Error::FailedToConvertAvatarContentType(e))? {
        "image/png" | "image/jpeg" | "image/jpg" | "image/webp" | "image/gif" => Ok(true),
        _ => Ok(false),
    }
}
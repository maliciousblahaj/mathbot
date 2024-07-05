use mathbot::command::CommandParams;
use mathbot::error::ClientError;
use mathbot::ui::embed::{base_embed, ColorType, EmbedCtx};
use mathbot::{send_embed, send_help, Error, Result, SendCtx};

pub async fn rockpaperscissors(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let Some(userinput) = params.args.get(0) else {return send_help(params).await};

    let response = match userinput.to_lowercase().as_str() {
        "r" | "rock" => "Paper",
        "p" | "paper" => "Scissors",
        "s" | "scissors" => "Rock",
        "rick" => "Astley",
        _ => {return Err(Error::Client(ClientError::RockPaperScissorsInvalidInput(userinput.clone())));}
    };

    send_embed(
        base_embed(&EmbedCtx::from_account(account), ColorType::Fun)
            .title(response)
            .description("The AI won. You earned `0MTC$`"), 
        &SendCtx::from_params(&params)).await?;
    Ok(())
}
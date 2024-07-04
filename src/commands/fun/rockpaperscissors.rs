use mathbot::command::CommandParams;
use mathbot::error::ClientError;
use mathbot::ui::embed::{base_embed, ColorType, EmbedCtx};
use mathbot::{send_embed, Error, Result, SendCtx};

pub async fn rockpaperscissors(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    if params.args.len() == 0 {
        return Err(Error::Client(ClientError::RockPaperScissorsNothingSpecified));
    }
    let response = match params.args[0].to_lowercase().as_str() {
        "r" | "rock" => "Paper",
        "p" | "paper" => "Scissors",
        "s" | "scissors" => "Rock",
        "rick" => "Astley",
        _ => {return Err(Error::Client(ClientError::RockPaperScissorsInvalidInput(params.args[0].clone())));}
    };

    send_embed(
        base_embed(&EmbedCtx::from_account(account), ColorType::Fun)
            .title(response)
            .description("The AI won. You earned `0MTC$`"), 
        &SendCtx::from_params(&params)).await?;
    Ok(())
}
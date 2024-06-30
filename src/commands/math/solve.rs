use evalexpr::eval;
use mathbot::command::CommandParams;
use mathbot::error::ClientError;
use mathbot::{send_message, Error, Result};

pub async fn solve(params: CommandParams) -> Result<()> {
    if params.args.len() == 0 {
        return Err(Error::Client(ClientError::NoSolveExpression));
    }
    let result = eval(&params.args_str)
        .map_err(|_| Error::Client(ClientError::InvalidSolveExpression))?.to_string();

    send_message(result, &params).await?;
    Ok(())
}
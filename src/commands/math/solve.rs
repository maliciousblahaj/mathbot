use evalexpr::eval;
use mathbot::command::CommandParams;
use mathbot::error::ClientError;
use mathbot::{send_message, Error, Result, SendCtx};

pub async fn solve(params: CommandParams) -> Result<()> {
    if params.args.is_empty() {
        return Err(Error::Client(ClientError::NoSolveExpression));
    }
    let result = eval(&params.args_str)
        .map_err(|_| Error::Client(ClientError::InvalidSolveExpression))?.to_string();

    send_message(result, &SendCtx::from_params(&params)).await?;
    Ok(())
}
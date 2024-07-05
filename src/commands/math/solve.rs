use std::f64::consts::{PI, TAU};

use evalexpr::{context_map, eval_with_context, Value};
use mathbot::command::CommandParams;
use mathbot::error::ClientError;
use mathbot::{send_help, send_text, Error, Result, SendCtx};



pub async fn solve(params: CommandParams) -> Result<()> {
    if params.args.is_empty() {
        return send_help(params).await;
    }

    let context = context_map! {
        "PI" => PI,
        "TAU" => TAU,
        "sin" => Function::new(|n| Ok(Value::Float(n.as_float()?.sin()))),
        "cos" => Function::new(|n| Ok(Value::Float(n.as_float()?.cos()))),
        "tan" => Function::new(|n| Ok(Value::Float(n.as_float()?.tan()))),
    }.map_err(|e| Error::FailedToGetSolveContextMap(e))?;

    let expr = &params.args_str;
    let result = eval_with_context(&expr, &context)
        .map_err(|_| Error::Client(ClientError::InvalidSolveExpression(expr.to_string())))?.to_string();

    send_text(result, &SendCtx::from_params(&params)).await?;
    Ok(())
}
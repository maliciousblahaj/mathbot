use mathbot::{command::CommandParams, error::ClientError, send_help, send_text, Error, Result, SendCtx};
use num_bigint::BigUint;

pub async fn fibonacci(params: CommandParams) -> Result<()> {
    let Some(number) = params.args.get(0) else {return send_help(params).await;};
    let Ok(number) = number.parse::<usize>() else {return send_help(params).await;};
    if number > 9571 { return Err(Error::Client(ClientError::FibonacciTooHighInput));}

    let mut a = BigUint::from(1u32);
    let mut b = BigUint::ZERO;

    for _ in 0..number {
        let c = a.clone();
        a = a+b;
        b = c;
    }
    send_text(b.to_string(), &SendCtx::from_params(&params)).await?;

    Ok(())
}
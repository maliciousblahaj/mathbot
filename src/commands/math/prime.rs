use mathbot::{command::CommandParams, send_embed, send_help, ui::embed::{base_embed, ColorType}, Error, Result, SendCtx};
use num_bigint::BigUint;
use num_prime::{nt_funcs::{is_prime, is_prime64}, Primality};

pub async fn prime(params: CommandParams) -> Result<()> {
    let Some(input) = params.args.get(0) else {return send_help(params).await;};
    
    let Ok(n) = input.parse::<BigUint>() 
        else {return send_help(params).await;};

    let desc =
        if n <= BigUint::from(u64::MAX) {
            let n: u64 = n.try_into().map_err(|_| Error::PrimeFailedToGetU64)?;
            match is_prime64(n) {
                true => format!("`{n}` is a prime number!"),
                false => format!("`{n}` is not a prime number!")
            }
        } else {
            match is_prime(&n, None) {
                Primality::Yes => format!("`{n}` is a prime number!"),
                Primality::No => format!("`{n}` is not a prime number!"),
                Primality::Probable(prob) => format!("`{n}` is maybe a prime number! Probability: `{}%`", prob*100.0)
            }
        };

    send_embed(
        base_embed(&params.get_embed_ctx(), ColorType::Tool)
            .description(desc), 
        &SendCtx::from_params(&params)
    ).await?;

    Ok(())
}
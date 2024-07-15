use lazy_regex::regex_captures;
use mathbot::{command::CommandParams, error::ClientError, send_help, send_text, Error, Result, SendCtx};
use num_bigint::BigInt;
use num_integer::gcd;

pub async fn fractionify(params: CommandParams) -> Result<()> {
    let Some(input) = params.args.get(0) else {return send_help(params).await;};

    #[allow(non_snake_case)]
    let Some((a, b, c, Y, Z, )) = parse_fractionify_input(input)
        else {return Err(Error::Client(ClientError::InvalidFractionifyInput))};

    let x10 = BigInt::from(10);
    //formula: https://cdn.discordapp.com/attachments/992385192931106846/1223029753657950370/image.png
    
    let numerator: BigInt = 
        if Z == 0 {a*x10.pow(Y) + b} 
        else { (x10.pow(Y+Z)-x10.pow(Y))*a + (x10.pow(Z)-1)*b + c };

    let denominator: BigInt = 
        if Z == 0 {x10.pow(Y)} 
        else { x10.pow(Y+Z) - x10.pow(Y) };

    let gcd = gcd(numerator.clone(), denominator.clone());

    let numerator = &numerator/&gcd;
    let denominator = &denominator/&gcd;

    send_text(format!("{numerator}/{denominator}"), &SendCtx::from_params(&params)).await?;
    Ok(())
}

///Returns the integer part, decimal part, repeating pattern, length of decimal part, length of repeating pattern
fn parse_fractionify_input(input: &String) -> Option<(BigInt, BigInt, BigInt, u32, u32)>{
    let output = regex_captures!("^([0-9]+)(?:(?:.([0-9]+)?)(?:\\(([0-9]+)\\))?)?$", input);
    output.map(
        |(_, a, b, c)| {
            (str_to_bigint(a), str_to_bigint(b), str_to_bigint(c), b.len() as u32, c.len() as u32)
        }
    )
}

fn str_to_bigint(input: &str) -> BigInt {
    if input.is_empty() {
        BigInt::ZERO
    } else {
        input.parse::<BigInt>().unwrap()//unwrap_or(BigInt::from(0))
    }
}
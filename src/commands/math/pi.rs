use std::{cmp::min, io::SeekFrom};

use mathbot::{command::CommandParams, error::ClientError, send_help, send_text, Error, Result, SendCtx};
use tokio::{fs::File, io::{AsyncReadExt, AsyncSeekExt}};

pub async fn pi(params: CommandParams) -> Result<()> {
    let Some(index) = params.args.get(0) else {return send_help(params).await;};
    let Ok(index) = index.parse::<u32>() else {return send_help(params).await;};

    let mut amount = match params.args.get(1) {
        Some(amount) => amount.parse::<u32>().unwrap_or(1),
        None => 1,
    };

    match amount {
        0 => {amount = 1;},
        2001.. => {return Err(Error::Client(ClientError::PiDigitsTooHighAmount));}
        _ => (),
    };
    if index > 1000000000 {return Err(Error::Client(ClientError::PiDigitsTooHighIndex));}

    let digits: String = get_pi_digits(index, amount).await?.iter().map(|&byte| (byte + b'0')as char).collect();

    send_text(digits, &SendCtx::from_params(&params)).await?;

    Ok(())
}



///The startindex parameter is 1-indexed
async fn get_pi_digits(startdigit: u32, amount: u32) -> Result<Vec<u8>> {
    let amount = min(amount, 1000000001-startdigit) as usize;
    let startdigit = if startdigit == 0 {0} else {startdigit-1};

    let mut file = File::open("./assets/pi-digits.mathbot")
        .await
        .map_err(|e| Error::FailedToGetPiDigitFile(e))?;

    // Each byte contains two digits
    let byte_index = startdigit / 2;

    file.seek(SeekFrom::Start(byte_index as u64))
        .await
        .map_err(|e| Error::FailedToSeekInPiDigitFile(e))?;

    let mut bytes = vec![0; if amount % 2 == 1 {amount/2 + 1} else {amount/2}];
    file.read_exact(&mut bytes)
        .await
        .map_err(|e| Error::FailedToReadInPiDigitFile(e))?;

    println!("{}", bytes.len());
    let mut digits = Vec::with_capacity(amount);
    let mut bytes_iter = bytes.iter();
    let byte = bytes_iter.next().ok_or(Error::PiFailedToGetFirstByte)?;

    if startdigit % 2 == 0 {
        digits.push((byte >> 4) & 0x0F);
    }
    digits.push(byte & 0x0F);

    for byte in bytes_iter {
        digits.push((byte >> 4) & 0x0F);
        digits.push(byte & 0x0F);
    }
    if digits.len() > amount {
        digits.pop();
    }
    
    Ok(digits)
}
use mathbot::{command::CommandParams, error::ClientError, format_f64, format_i64, send_embed, send_help, ui::embed::{base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};
use rand::{thread_rng, Rng};
use num_format::{Locale, ToFormattedString};

const WIN_CHANCE: f64 = 0.3;

//input should be between 0 and 1. Graph the function to see more clearly
fn win_multiplier(x: f64) -> f64 {
    match x {
        0.9..=1.0 => {
            (1.1*x).powi(40) + 1.7
        }
        _ => {
            2.0*x.powi(2) + 0.73
        }
    }
}

pub async fn gamble(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let Some(amount) = params.args.get(0)
        else {return send_help(params).await;};
    let Ok(amount) = amount.parse::<f64>()
        else {return Err(Error::Client(ClientError::GambleInvalidAmount(amount.to_string())));};
    if &amount < &100.0 {
        return Err(Error::Client(ClientError::GambleTooLowAmount));
    }
    if &amount > &account.balance {
        return Err(Error::Client(ClientError::GambleInsufficientFunds));
    }
    if &amount > &1000000.0 {
        return Err(Error::Client(ClientError::GambleTooHighAmount));
    }
    let (won, multi) = get_rng();
    let responseembed = match won {
        true => {
            let multiplier = win_multiplier(multi);
            let won =  multiplier*amount;
            sqlx::query!("UPDATE Accounts SET balance = balance + ? WHERE id=?", won, account.id)
                .execute(params.state.get_model_controller().get_database())
                .await
                .map_err(|e| Error::FailedToAddToAccountBalance(e))?;
            base_embed(&EmbedCtx::from_account(account), ColorType::Success)
                .title("You won!")
                .description(format!("You won **{}MTC$**\n\nPercent won: `{:.0}%`\n\nNew balance: `{}MTC$`", format_f64(&won), multiplier*100.0, format_f64(&(account.balance + won))))
        },
        false => {
            sqlx::query!("UPDATE Accounts SET balance = balance - ? WHERE id=?", amount, account.id)
                .execute(params.state.get_model_controller().get_database())
                .await
                .map_err(|e| Error::FailedToRemoveFromAccountBalance(e))?;
            base_embed(&EmbedCtx::from_account(account), ColorType::Failure)
                .title("You lost")
                .description(format!("You lost **{}MTC$**\n\nNew balance: `{}MTC$`\n\nBut don't give up; keep gambling! Remember, you can win 5000% of your money, but you can only lose 100%!", format_f64(&amount), format_f64(&(account.balance - amount))))
        }
    };

    send_embed(responseembed, &SendCtx::from_params(&params)).await?;
    Ok(())
}

//(if the user won, win range to input to function)
fn get_rng() -> (bool, f64) {
    let mut rng = thread_rng();
    (rng.gen_bool(WIN_CHANCE), rng.gen::<f64>())
}
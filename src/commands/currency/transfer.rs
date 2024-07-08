use std::cmp;

use mathbot::{command::{Command, CommandParams}, error::ClientError, model::account::Account, send_embed, send_help, ui::{embed::{base_embed, ButtonEmoji, ColorType, EmbedCtx}, ButtonInfo, ButtonMessage}, Error, Result, SendCtx};
use rand::Rng;
use serenity::all::{ButtonStyle, CreateButton, CreateMessage};

pub async fn transfer(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    //get args
    let (Some(amountinput), Some(recieverinput)) = (params.args.get(0),params.args.get(1))
        else { return send_help(params).await; };
    
    //get amount to transfer
    let amount = match amountinput.parse::<f64>().ok() {
        Some(amount) => amount,
        _ if amountinput == "all" => account.balance,
        _ => { return Err(Error::Client(ClientError::TransferInvalidAmount(amountinput.to_string()))); },
    };
    //amount must be greater than 100
    if &amount < &100.0 {
        return Err(Error::Client(ClientError::TransferTooSmallAmount));
    }
    //user must have at least the amount of money they want to transfer 
    //the next if statement requires awaiting which may desyncronize the account balance data
    if &account.balance < &amount {
        return Err(Error::Client(ClientError::TransferInsufficientFunds));
    }
    //reciever must be a valid user
    let Some(recieveraccount) = params.get_account_by_user_input(recieverinput).await
        else { return Err(Error::Client(ClientError::TransferRecieverDoesntExist)); };
    //reciever must not be self
    if &recieveraccount.id == &account.id {
        return Err(Error::Client(ClientError::TransferRecieverIsSelf));
    }
    if recieveraccount.is_admin() && !account.is_admin() {
        let amount = f64::min(account.balance, amount*2.0);
        sqlx::query!("UPDATE Accounts SET balance = balance - ? WHERE id=?; UPDATE Accounts SET balance = balance + ? WHERE id=?",
            amount, account.id, amount, recieveraccount.id
        ).execute(params.state.get_model_controller().get_database())
            .await.map_err(|e| Error::FailedToTransferMathCoins(e))?;
        send_embed(
            base_embed(&EmbedCtx::from_account(&account), ColorType::Success)
                .description("Thank you for attempting to transfer to an admin. Fortunately (for you) we just taxed you for the money you was about to send, so you don't need to make an effort to press the confirm button. Heck, we even taxed you for double the amount to prepare for future transfers. Aren't we just so incredibly thoughtful?"), 
            &SendCtx::from_params(&params)).await?;
        return Ok(());
    }

    let confirm = CreateMessage::new()
    .embed(base_embed(&params.get_embed_ctx(), ColorType::UserInfo)
        .title("Confirm Transfer")
        .description(format!("Are you sure you want to transfer `{:.0}MTC$` to `@{}`?", amount, recieveraccount.username).as_str())
    );

    if params.await_confirmation(confirm).await? == false {
        return Ok(());
    }

    let tax = get_tax(&account);
    let tax_change_rate = (100.0-f64::from(tax))*0.01;

    let amount = amount*tax_change_rate;

    sqlx::query!("UPDATE Accounts SET balance = balance - ? WHERE id=?; UPDATE Accounts SET balance = balance + ? WHERE id=?",
            amount, account.id, amount, recieveraccount.id
        ).execute(params.state.get_model_controller().get_database())
            .await.map_err(|e| Error::FailedToTransferMathCoins(e))?;

    send_embed(
        base_embed(&EmbedCtx::from_account(&account), ColorType::Success)
            .description(format!("Successfully transferred `{amount:.0}MTC$` to `@{}` after `{tax}%` tax", recieveraccount.username)), 
        &SendCtx::from_params(&params)).await?;
    Ok(())
}

fn get_tax(sender_account: &Account) -> i32 {
    let mut rng = rand::thread_rng();
    let randomvar = rng.gen_range(1..=100);
    match randomvar {
        _ if sender_account.is_admin() => 0,
        100 => 200,
        97..=99 => 100,
        94..=96 => 99,
        70..=93 => 60,
        _ => 30,
    }
}


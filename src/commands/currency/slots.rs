use mathbot::error::ClientError;
use mathbot::ui::embed::{base_embed, ColorType, EmbedCtx};
use mathbot::{format_f64, send_embed, send_help, send_text, Error, SendCtx};
use mathbot::{command::CommandParams, Result};
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use serenity::all::EmbedField;
use strum::IntoEnumIterator;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub async fn slots(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;
    let Some(amount) = params.args.get(0)
        else {return send_help(params).await;};
    let Ok(amount) = amount.parse::<f64>()
        else {return Err(Error::Client(ClientError::GambleInvalidAmount(amount.to_string())));};
    if &amount < &10.0 {
        return Err(Error::Client(ClientError::SlotsTooLowAmount));
    }
    if &amount > &account.balance {
        return Err(Error::Client(ClientError::GambleInsufficientFunds));
    }
    if &amount > &100000.0 {
        return Err(Error::Client(ClientError::SlotsTooHighAmount));
    }
    let a = sqlx::query!("UPDATE Accounts SET balance = balance - ? WHERE id=? RETURNING balance as newbalance", amount, account.id)
        .fetch_one(params.state.get_model_controller().get_database())
        .await
        .map_err(|e| Error::FailedToRemoveFromAccountBalance(e))?;
    let balance = a.newbalance;

    let slots = Slots::new();
    let (win, jackpot) = slots.get_win();
    let slot_fields = [
        ("**Slot machine**", slots.get_string(), true)
        ];
    let thumbnail = "https://cdn.discordapp.com/attachments/1063151823151693895/1261085833918812180/slotsexplanation.png";
    let responseembed = match win {
        Some(multiplier) => {
            let won =  multiplier*amount;
            let a = sqlx::query!("UPDATE Accounts SET balance = balance + ? WHERE id=? RETURNING balance as newbalance", won, account.id)
                .fetch_one(params.state.get_model_controller().get_database())
                .await
                .map_err(|e| Error::FailedToAddToAccountBalance(e))?;
            base_embed(&EmbedCtx::from_account(account), ColorType::Success)
                .title(if jackpot {"Jackpot!"} else {"You won!"})
                .description(format!("You won **{}MTC$**\n\nPercent won: `{:.0}%`\n\nSpent `{}MTC$`\nNew balance: `{}MTC$`", format_f64(&won), multiplier*100.0, format_f64(&amount), format_f64(&(a.newbalance))))
                .thumbnail(thumbnail)
                .fields(slot_fields)
                
        },
        None => {
            base_embed(&EmbedCtx::from_account(account), ColorType::Failure)
                .title("You lost")
                .description(format!("Better luck next time!\n\nSpent `{}MTC$`\nNew balance: `{}MTC$`", format_f64(&amount), format_f64(&balance)))
                .thumbnail(thumbnail)
                .fields(slot_fields)
        }
    };

    send_embed(responseembed, &SendCtx::from_params(&params)).await?;

    /* For the purpose of balancing slots
    let mut balance = 1000000000.0;
    for _ in 0..100000 {
        let slots = Slots::new();
        balance -= 10000.0;
        match slots.get_win() {
            (Some(mult),_) => {balance += mult*10000.0},
            (None,_) => ()
        }
    }
    send_text(format!("Ran `100,000` slots of `10,000MTC$` each\n\nDelta balance: `{}MTC$`", format_f64(&(balance-1000000000.0))), 
        &SendCtx::from_params(&params)).await?;
    */
    Ok(())
}

struct Slots {
    rows: Vec<[SlotItem;5]>,
}

impl Slots {
    pub fn new() -> Self {
        let items = SlotItem::iter().collect::<Vec<_>>();
        let mut rng = thread_rng();
        let mut rows = Vec::new();
        for _ in 0..3 {
            let mut row = [SlotItem::Malako; 5];
            for i in 0..5 {
                let random = rng.gen_range(1..=100);
                row[i] = match random {
                    96..=100 => SlotItem::Planet, //    5%
                    86..=95 => SlotItem::Mathtopia, //  10&
                    66..=85 => SlotItem::Ferris, //     20%
                    36..=65 => SlotItem::Victor, //     30%
                    _ => SlotItem::Malako, //           35%
                }
            }
            rows.push(row);
        }

        Self {
            rows
        }
    }

    //returns the win multiplier and the second argument if it's a jackpot
    pub fn get_win(&self) -> (Option<f64>, bool) {
        let mut multiplier = 0.0;
        let mut jackpot = false;
        for row in &self.rows {
            let mut previousitem = (&row[0], 1);
            for item in &row[1..] {
                if item == previousitem.0 {
                    previousitem.1 += 1;
                    continue;
                }
                if previousitem.1 >= 3 {
                    break;
                }
                previousitem = (item, 1);
            }
            match previousitem {
                (item, 3) => {multiplier += item.multiplier();},
                (item, 4) => {multiplier += item.multiplier()*2.5;},
                (item, 5) => {multiplier += item.multiplier()*25.0; if item != &SlotItem::Malako {jackpot = true;};},
                _ => {continue;},
            }
        }
        
        (if multiplier == 0.0 {None} else {Some(multiplier)}, jackpot)
    }

    pub fn get_string(&self) -> String {
        let rows: &Vec<[SlotItem; 5]> = &self.rows;
        format!(
            "\n**->  {}  {}  {}  {}  {}  <-\n->  {}  {}  {}  {}  {}  <-\n->  {}  {}  {}  {}  {}  <-**",
            rows[0][0], rows[0][1], rows[0][2], rows[0][3], rows[0][4],
            rows[1][0], rows[1][1], rows[1][2], rows[1][3], rows[1][4],
            rows[2][0], rows[2][1], rows[2][2], rows[2][3], rows[2][4],
        )
    }
}

#[derive(strum_macros::EnumIter, Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum SlotItem {
    Planet,
    Mathtopia,
    Ferris,
    Victor,
    Malako
}

impl SlotItem {
    pub fn emoji_str(&self) -> &str {
        match self {
            Self::Planet => "<:slot1:1260985506020200548>",
            Self::Mathtopia => "<:slot2:1260985504472629419>",
            Self::Ferris => "<:slot3:1260985501704388798>",
            Self::Victor => "<:slot4:1260985503038312549>",
            Self::Malako => "<:slot5:1260985500483850300>",
        }
    }

    pub fn multiplier(&self) -> f64 {
        match self {
            Self::Planet => 100.0,
            Self::Mathtopia => 10.0,
            Self::Ferris => 2.5,
            Self::Victor => 1.0,
            Self::Malako => 0.2,
        }
    }
}

impl Display for SlotItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.emoji_str())
    }
}
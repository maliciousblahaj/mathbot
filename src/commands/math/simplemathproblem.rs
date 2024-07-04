use std::cmp;

use mathbot::{command::CommandParams, send_embed, ui::embed::{base_embed, base_embed_no_author, ColorType, EmbedCtx}, Result, SendCtx};
use rand::Rng;


pub async fn simplemathproblem(params: CommandParams) -> Result<()> {
    
    let (question, answer) = get_smp_problem();

    (&params).state.get_smp_answers().lock().await.insert(params.msg.channel_id.into(), answer);

    let embed = match &params.account {
        Some(acc) => base_embed(&EmbedCtx::new(acc.username.clone(), acc.avatar_url.clone()), ColorType::Fun),
        None => base_embed_no_author(ColorType::Fun),
    }.description(question);

    send_embed(embed, &SendCtx::from_params(&params)).await?;

    Ok(())
}

fn get_smp_problem() -> (String, i64) {
    let mut rng = rand::thread_rng();
    let randomvar = rng.gen_range(1..=5);
    match randomvar {
        1 => {
            let (a, b) = (rng.gen_range(3..=50), rng.gen_range(3..=50));
            (format!("What is {a}*{b}?"), a*b)
        },
        2 => {
            let (a, b) = (rng.gen_range(100..=1000), rng.gen_range(100..=1000));
            (format!("What is {a}-{b}?"), a-b)
        },
        3 => {
            let (a, b) = (rng.gen_range(100..=5000), rng.gen_range(100..=5000));
            (format!("What is {a}+{b}?"), a+b)
        },
        4 => {
            let (a, b) = (rng.gen_range(50..=500), rng.gen_range(3..=50));
            (format!("What is the floor of {a}/{b}?"), a/b)
        },
        5 => {
            let (a, b) = (rng.gen_range(2..=20), rng.gen_range(2..=20));
            (format!("What is {}/{}?", a*b, cmp::min(a,b)), cmp::max(a,b))
        },
        _ => {panic!("Impossible error")},
    }
}
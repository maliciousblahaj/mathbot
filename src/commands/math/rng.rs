use std::cmp;

use mathbot::{command::CommandParams, send_embed, send_help, ui::embed::{base_embed, ColorType}, Result, SendCtx};
use rand::{distributions::uniform::SampleRange, Rng};

pub async fn rng(params: CommandParams) -> Result<()> {
    let (Some(Some(a)), Some(Some(b))) = (
            params.args.get(0).map(|a| a.parse::<i64>().ok()), 
            params.args.get(1).map(|a| a.parse::<i64>().ok())
        ) else {return send_help(params).await;};

    let randomvalue = get_in_range(cmp::min(a, b)..=cmp::max(a,b));
    send_embed(
        base_embed(&params.get_embed_ctx(), ColorType::Tool).description(randomvalue.to_string()),
        &SendCtx::from_params(&params),
    ).await?;
    Ok(())
}

fn get_in_range<R: SampleRange<i64>>(range: R) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}
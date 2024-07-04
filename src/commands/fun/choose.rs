use mathbot::command::CommandParams;
use mathbot::error::ClientError;
use mathbot::ui::embed::{base_embed, ColorType};
use mathbot::{send_embed, Error, Result, SendCtx};
use rand::seq::SliceRandom;

pub async fn choose(params: CommandParams) -> Result<()> {
    if params.args.len() <= 1 {
        return Err(Error::Client(ClientError::ChooseNoArgsSpecified));
    }
    let randomchoice = (&params.args).choose(&mut rand::thread_rng()).unwrap();
    send_embed(base_embed(&params.get_embed_ctx(), ColorType::Fun).description(randomchoice), &SendCtx::from_params(&params)).await?;
    Ok(())
}
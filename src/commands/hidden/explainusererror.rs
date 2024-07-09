use mathbot::{command::CommandParams, error::ClientError, send_embed, ui::embed::{base_embed, ColorType}, Error, Result, SendCtx};

pub async fn explainusererror(params: CommandParams) -> Result<()> {
    let Some(error_code) = params.args.get(0) else {
        return Err(Error::Client(ClientError::UserError(params.bot_prefix)));
    };
    let (explanation, error) = match error_code.to_lowercase().as_str() {
        "api" => ("**A**rrogante **P**essoa **I**diota error; Indicates the error is caused by an arrogant idiot.", "API Error"),
        "bios" => ("**B**icho **I**gnorante **O**perando o **S**istema error; Indicates the error is caused by a dumb idiot operating the system", "BIOS Error"),
        "usb" => ("**U**suário **S**uper **B**urro error; Indicates the error is caused by a super dumb user.", "USB Error"),
        "error" => ("Indicates the error exists **40**cm (16in) from the device", "Error 40"),
        "ibm" => ("**I**diot **B**ehind **M**achine error; Indicates the error originates from behind the machine.", "IBM Error"),
        _ => {return Err(Error::Client(ClientError::InvalidUserErrorExplainCode(error_code.to_string())))},
    };

    send_embed(base_embed(
        &params.get_embed_ctx(), ColorType::Info).title(error).description(explanation),
        &SendCtx::from_params(&params)).await?;

    Ok(())
}
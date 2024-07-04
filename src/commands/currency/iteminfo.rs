use mathbot::{command::CommandParams, error::ClientError, model::item::{Item, ItemController, ItemType}, send_embed, ui::embed::{self, base_embed, ColorType, EmbedCtx}, Error, Result, SendCtx};
use serenity::all::{CreateEmbed, EmbedField};

pub async fn iteminfo(params: CommandParams) -> Result<()> {
    let account = params.require_account()?;

    let item = params.args.get(0)
        .ok_or(Error::Client(ClientError::ItemInfoArgumentsNotSpecified))?;

    let mc = params.state.get_model_controller();
    let itemc = ItemController::new(
        mc, 
        mathbot::model::item::ItemQueryKey::name_id_incomplete(item.clone())
    );
    let item = itemc.fetch_item().await
        .map_err(|e| Error::Client(ClientError::ItemInfoItemNotFound(item.to_string(), Box::new(e))))?;
    
    send_embed(item_embed(&EmbedCtx::from_account(account), item), &SendCtx::from_params(&params)).await?;

    Ok(())
}

fn item_embed(ctx: &EmbedCtx, item: Item) -> CreateEmbed {
    let mut embed = base_embed(ctx, ColorType::Info)
        .title(item.display_name);

    if let Some(desc) = item.description {
        embed = embed.description(desc);
    }
    if let Some(url) = item.image_url {
        embed = embed.image(url);
    }

    let mut embedfields: Vec<(&str, String, bool)> = vec![
        ("**ITEM TYPE**", format!("`{}`", item.item_type.get_string()), true),
        ("**AMOUNT OWNED**", format!("`{}`", 0), true),
        (
            "**PRICE**", 
            match item.price {
                Some(price) => format!("`{price}MTC$`"),
                None => "Not for sale".to_string(),
            },
            true
        ),
    ];

    if let Some(mps) = item.mps {
        embedfields.push(
            ("**MTC$/h**", format!("`{:.0}`", mps*3600.0), true)
        );
    }

    embed.fields(embedfields)

}
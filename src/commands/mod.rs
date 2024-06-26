pub mod math;
pub mod bot;






use crate::{Error, Result};
use serenity::all::{Context, Message};

pub struct CommandParams {
    ctx: Context, 
    msg: Message,

    // TODO: add more things like args
}


pub struct Command {
    handle: fn(CommandParams) -> Result<()>,
    aliases: Vec<String>,
}

impl Command {
    fn new(
        handle: fn(CommandParams) -> Result<()>,
        aliases: Vec<String>,
    ) -> Self {
        
        Self {
            handle,
            aliases,
        }
    }
}
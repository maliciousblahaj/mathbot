use crate::bot::Bot;

pub struct State {
    bot: Bot,
    //TODO: add database/modelcontroller to this
}


impl State {
    pub fn new(bot: Bot) -> Self {
        Self {
            bot
        }
    }
}
use crate::commands::Command;
use crate::{Error, Result};
use async_trait::async_trait;










pub struct Bot {
    prefix: String,
    commands: Option<Vec<Command>>,
}

impl Bot {
    fn new(prefix: String) -> Self {
        Self {
            prefix,
            commands: None,
        }
    }

    fn register()
}
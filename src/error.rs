use std::{error, fmt};

pub type Result<T> = core::result::Result<T, Error>;
use derive_more::From;
use serenity::prelude::SerenityError;


#[derive(Debug, From, strum_macros::AsRefStr)]
pub enum Error {
    // -- Bot setup errors
    RegisterCommandAlreadyExists,
    RegisterAliasAlreadyExists,
    SubcommandAtRootLevel,
    RootCommandAtSubLevel,
    SubcommandIndexAtRootLevel,
    IncompatibleCommandTypes,
    CommandCategoryKeyDoesntExist,
    CommandCategoryVecDoesntExist,
    CommandIndexDoesntExist,
    
    // -- Bot run errors
    FailedToSendMessage,
    NoCommandHandle,

    // -- Misc errors
    Test,

    // -- External errors
    #[from]
    Serenity(SerenityError),
}

// region:    --- Error boilerplate
impl error::Error for Error{}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.as_ref().to_string())
    }
}
// endregion: --- Error boilerplate

pub enum ClientError {

}

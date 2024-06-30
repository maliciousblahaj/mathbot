use std::{error, fmt};

pub type Result<T> = core::result::Result<T, Error>;
use derive_more::From;


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
    CommandIndexWrongType,
    
    // -- Bot run errors
    FailedToSendMessage,
    FailedToEditMessage,
    NoCommandHandle,
    FailedToGetPingTime,
    PoisonedStateMutex,
    PoisonedCommandMutex,
    FailedToGetSystemTimestamp,
    InvalidTimeDelta,

    // -- Client errors
    Client(ClientError),


    // -- Misc errors
    Test,
    ImpossibleError, //for when you have already checked if an option is none but you still need to cover the none arm
}

// region:    --- Error boilerplate
impl error::Error for Error{}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.as_ref().to_string())
    }
}
// endregion: --- Error boilerplate
#[allow(unused)]
#[derive(Debug)]
pub enum ClientError {
    InvalidSolveExpression,
    NoSolveExpression,
}

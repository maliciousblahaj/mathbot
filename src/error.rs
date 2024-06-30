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

pub struct ClientErrInfo {
    title: String,
    description: String,
}

impl ClientErrInfo {
    pub fn new(title: &str, description: &str) -> Self {
        Self { title: title.to_string(), description: description.to_string() }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }
}

impl ClientError {
    pub fn get_description(&self) -> ClientErrInfo {
        match self {
            Self::NoSolveExpression => ClientErrInfo::new("No expression specified", "You have to specify an expression to solve"),
            Self::InvalidSolveExpression => ClientErrInfo::new("Invalid expression", "You need to specify a valid expression"),
        }
    }
}

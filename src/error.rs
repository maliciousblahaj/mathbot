use std::{error, fmt};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, strum_macros::AsRefStr)]
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
    InvalidAccountQueryParameter(strum::ParseError, String),
    InvalidAccountSearchParameter(String),
    
    // -- Bot run errors
    FailedToSendMessage(serenity::Error),
    FailedToEditMessage(serenity::Error),
    NoCommandHandle,
    FailedToGetPingTime,
    PoisonedStateMutex,
    PoisonedCommandMutex,
    FailedToGetSystemTimestamp(std::time::SystemTimeError),
    InvalidTimeDelta,
    SayAliasNotFoundInMessageContent,
    FailedToGetSolveContextMap(evalexpr::EvalexprError),

    // -- Database errors
    FailedToFetchItem(sqlx::Error),
    FailedToFetchAccount(sqlx::Error),
    FailedToParseItemType(strum::ParseError),
    DatabaseFailedToGetAccountId,
    CannotGetAccountQueryItemAsI64,
    CannotGetAccountQueryItemAsString,

    // -- Client errors
    Client(ClientError),

    // -- Misc errors
    TestError, //not for production. only use this in one place at a time so you can know when testing if something went wrong
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


#[derive(Debug)]
pub enum ClientError {
    // -- Info
    
    // -- User
    
    // -- Currency
    
    // -- Fun
    NoSayContent,

    // -- Math
    InvalidSolveExpression(String),
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
            // -- Fun
            Self::NoSayContent => ClientErrInfo::new("Invalid input", "The bot is unable to send an empty message"),
            // -- Math
            Self::NoSolveExpression => ClientErrInfo::new("No expression specified", "You have to specify an expression to solve"),
            Self::InvalidSolveExpression(expr) => ClientErrInfo::new("Invalid expression", format!("`{expr}` is not a valid expression!").as_str()),
        }
    }
}

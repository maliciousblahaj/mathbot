use std::{convert::Infallible, error, fmt};

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
    CommandTypeNotRegistered,
    ButtonMessageNotSentYet,
    
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
    ButtonComponentNotFound,
    NonButtonComponentInMessage,
    NoEmojiOnButton,
    InteractionDataKindNotAButton,
    InteractionButtonIdNotFound,
    FailedToDeferButtonMessage(serenity::Error),
    InvalidInteractionId,
    FetchedSlotsBeforeFetchingAccount,
    ProcessMessageAccountIdConversionFailed(Infallible),

    // -- Database errors
    FailedToFetchItem(sqlx::Error),
    FailedToFetchAccount(sqlx::Error),
    FailedToFetchAccountSlots(sqlx::Error),
    FailedToParseItemType(strum::ParseError),
    DatabaseFailedToGetAccountId,
    CannotGetAccountQueryItemAsI64,
    CannotGetAccountQueryItemAsString,
    FailedToCreateAccount(sqlx::Error),

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
        write!(f, "Error: {:?}", self)
    }
}
// endregion: --- Error boilerplate


#[derive(Debug)]
pub enum ClientError {
    // -- Info
    
    // -- User
    AccountCreateAccountAlreadyExists,
    
    // -- Currency
    ItemInfoArgumentsNotSpecified,
    ItemInfoItemNotFound(String, Box<Error>),
    
    // -- Fun
    NoSayContent,

    // -- Math
    InvalidSolveExpression(String),
    NoSolveExpression,

    // -- Misc
    AccountRequired(String),
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
            // -- User
            Self::AccountCreateAccountAlreadyExists => ClientErrInfo::new("Account already exists", "You already have a MathBot©™ account"),
            // -- Currency
            Self::ItemInfoArgumentsNotSpecified => ClientErrInfo::new("Item not specified", "This command requires an item name as an argument"),
            Self::ItemInfoItemNotFound(item, _error) => ClientErrInfo::new("Item not found", format!("Couldn't find an item matching `{item}`").as_str()),
            // -- Fun
            Self::NoSayContent => ClientErrInfo::new("Invalid input", "The bot is unable to send an empty message"),
            // -- Math
            Self::NoSolveExpression => ClientErrInfo::new("No expression specified", "You have to specify an expression to solve"),
            Self::InvalidSolveExpression(expr) => ClientErrInfo::new("Invalid expression", format!("`{expr}` is not a valid expression!").as_str()),
            // -- Misc
            Self::AccountRequired(prefix) => ClientErrInfo::new("🔒Account required", format!("To gain access to this command you must first create a MathBot©™ account\n\nTo create a MathBot©™ account, simply execute `{prefix}account create`").as_str()),
        }
    }
}

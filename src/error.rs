use std::{error, fmt, num::TryFromIntError};


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
    SubCommandsRwLockPoisoned,
    CommandsRwLockPoisoned,
    
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
    UpdateBioAliasNotFoundInMessageContent,
    FailedToGetSolveContextMap(evalexpr::EvalexprError),
    ButtonComponentNotFound,
    NonButtonComponentInMessage,
    NoEmojiOnButton,
    InteractionDataKindNotAButton,
    InteractionButtonIdNotFound,
    FailedToDeferButtonMessage(serenity::Error),
    InvalidInteractionId,
    FetchedSlotsBeforeFetchingAccount,
    DeletedAccountBeforeFetchingAccount,
    ProcessMessageAccountIdConversionFailed,
    SendHelpNoHelpCommandConfigured,
    TimestampToI64Failed(TryFromIntError),
    FailedToGetAvatarUrl(reqwest::Error),
    FailedToConvertAvatarContentType(reqwest::header::ToStrError),

    // -- Database errors
    FailedToFetchItem(sqlx::Error),
    FailedToFetchAccount(sqlx::Error),
    FailedToFetchAccountSlots(sqlx::Error),
    FailedToParseItemType(strum::ParseError),
    DatabaseFailedToGetAccountId,
    CannotGetAccountQueryItemAsI64,
    CannotGetAccountQueryItemAsString,
    FailedToCreateAccount(sqlx::Error),
    FailedToCheckIfAccountExists(sqlx::Error),
    FailedToDeleteAccount(sqlx::Error),
    FailedToIncrementSmpsSolved(sqlx::Error),
    FailedToTransferMathCoins(sqlx::Error),
    FailedToUpdateAccountBio(sqlx::Error),
    FailedToUpdateAccountAvatar(sqlx::Error),

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
    FailedToCreateAccount(Box<Error>),
    FailedToDeleteAccount(Box<Error>),
    UpdateAvatarInvalidAvatarUrl(String),
    
    // -- Currency
    ItemInfoItemNotFound(String, Box<Error>),
    TransferRecieverDoesntExist,
    TransferRecieverIsSelf,
    TransferInvalidAmount(String),
    TransferTooSmallAmount,
    TransferInsufficientFunds,
    
    // -- Fun
    NoSayContent,
    RockPaperScissorsInvalidInput(String),

    // -- Math
    InvalidSolveExpression(String),
    AnswerNoProblemInChannel(String),
    SolutionNoProblemInChannel(String),

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
            Self::FailedToCreateAccount(_) => ClientErrInfo::new("Account creation failed", "An internal error happened"),
            Self::FailedToDeleteAccount(_) => ClientErrInfo::new("Account deletion failed", "An internal error happened"),
            Self::UpdateAvatarInvalidAvatarUrl(url) => ClientErrInfo::new("Invalid avatar url", format!("`{url}` is not a valid avatar url! Supported image formats are `png`, `jpeg`, `jpg`, `webp` and `gif`.").as_str()),
            // -- Currency
            Self::ItemInfoItemNotFound(item, _error) => ClientErrInfo::new("Item not found", format!("Couldn't find an item matching `{item}`").as_str()),
            Self::TransferRecieverDoesntExist => ClientErrInfo::new("Invalid reciever", "There is no MathBot©™ account connected to the user you specified"),
            Self::TransferRecieverIsSelf => ClientErrInfo::new("Invalid reciever", "Did you just think you could fool the admins by transferring to yourself? Pathetic."),
            Self::TransferInvalidAmount(a) => ClientErrInfo::new("Invalid amount", format!("`{a}` is not a valid amount").as_str()),
            Self::TransferTooSmallAmount => ClientErrInfo::new("Invalid amount", "The minimum transfer amount is `100MTC$`"),
            Self::TransferInsufficientFunds => ClientErrInfo::new("Insufficient funds", "After attempting to transfer the money you came to the conclusion that you're broke"),
            // -- Fun
            Self::NoSayContent => ClientErrInfo::new("Invalid input", "The bot is unable to send an empty message"),
            Self::RockPaperScissorsInvalidInput(i) => ClientErrInfo::new("Invalid input", format!("`{i}` is not a valid choice. Valid choices are `rock`, `r`, `paper`, `p`, `scissors`, `s`.").as_str()),
            // -- Math
            Self::InvalidSolveExpression(expr) => ClientErrInfo::new("Invalid expression", format!("`{expr}` is not a valid expression!").as_str()),
            Self::AnswerNoProblemInChannel(prefix) => ClientErrInfo::new("Nothing to answer", format!("There is no ongoing math problem in this channel. To recieve one, execute `{prefix}simplemathproblem`").as_str()),
            Self::SolutionNoProblemInChannel(prefix) => ClientErrInfo::new("Nothing to reveal the solution of", format!("There is no ongoing math problem in this channel. To recieve one, execute `{prefix}simplemathproblem`").as_str()),
            // -- Misc
            Self::AccountRequired(prefix) => ClientErrInfo::new("🔒Account required", format!("To gain access to this command you must first create a MathBot©™ account\n\nTo create a MathBot©™ account, simply execute `{prefix}account create`").as_str()),
        }
    }
}

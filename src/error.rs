use std::{error, fmt::{self, Display}};

use rand::seq::SliceRandom;

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
    FetchedInventoryBeforeFetchingAccount,
    DeletedAccountBeforeFetchingAccount,
    ClaimedMineBeforeFetchingAccount,
    ProcessMessageAccountIdConversionFailed,
    SendHelpNoHelpCommandConfigured,
    TimestampToI64Failed(std::num::TryFromIntError),
    FailedToGetAvatarUrl(reqwest::Error),
    FailedToConvertAvatarContentType(reqwest::header::ToStrError),
    FailedToOpenPrimeFile(std::io::Error),
    PrimeFailedToGetU64,
    FailedToGetPiDigitFile(std::io::Error),
    FailedToSeekInPiDigitFile(std::io::Error),
    FailedToReadInPiDigitFile(std::io::Error),
    PiFailedToGetFirstByte,
    AskAliasNotFoundInMessageContent,
    AskFailedToGetGptResponse(openai_api_rust::Error),

    // -- Database errors
    FailedToFetchItem(sqlx::Error),
    FailedToFetchAccount(sqlx::Error),
    FailedToFetchAccountSlots(sqlx::Error),
    FailedToFetchAccountInventory(sqlx::Error),
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
    FailedToUpdateAccountUsername(sqlx::Error),
    FailedToUpdateAccountPronouns(sqlx::Error),
    FailedToAddToAccountBalance(sqlx::Error),
    FailedToRemoveFromAccountBalance(sqlx::Error),
    FailedToFetchShop(sqlx::Error),
    FailedToBuyItems(sqlx::Error),
    FailedToClaimMine(sqlx::Error),
    FailedToFetchAccountMps(sqlx::Error),
    FailedToBuySlot(sqlx::Error),
    FailedToGetInventoryCount(sqlx::Error),
    FailedToGetPreviousSlotItem(sqlx::Error),
    FailedToSetSlotItem(sqlx::Error),
    FailedToRemoveSlotItem(sqlx::Error),
    FailedToBanAccount(sqlx::Error),
    FailedToUnbanAccount(sqlx::Error),
    
    // -- For client errors
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
    UpdateUsernameAlreadyExists(String),
    UpdateUsernameInvalidLength,
    UpdateUsernameTooSoon(i64),
    UpdatePronounsInvalid,
    
    // -- Currency
    ItemInfoItemNotFound(String, Box<Error>),
    TransferRecieverDoesntExist,
    TransferRecieverIsSelf,
    TransferInvalidAmount(String),
    TransferTooSmallAmount,
    TransferInsufficientFunds,
    GambleInvalidAmount(String),
    GambleTooLowAmount,
    GambleTooHighAmount,
    GambleInsufficientFunds,
    SlotsTooLowAmount,
    SlotsTooHighAmount,
    ShopBuyItemNotFound(String),
    ShopBuyInsufficientFunds,
    MineClaimNotOpenedYet,
    SlotBuyInsufficientFunds,
    MineSlotNotOwned,
    MineSetInvalidItemId,
    MineSetItemNotOwned,
    MineRemoveNothingToRemove,
    
    // -- Fun
    NoSayContent,
    RockPaperScissorsInvalidInput(String),
    AskNoOpenAiApiKey,

    // -- Math
    InvalidSolveExpression(String),
    AnswerNoProblemInChannel(String),
    SolutionNoProblemInChannel(String),
    InvalidFractionifyInput,
    PiDigitsTooHighAmount,
    PiDigitsTooHighIndex,
    FibonacciTooHighInput,

    // -- Admin
    AdminBanInvalidAccount,

    // -- Misc
    AccountRequired(String),
    UserError(String),
    InvalidUserErrorExplainCode(String),
    AccountIsBanned(i64),
}

pub struct ClientErrInfo {
    title: String,
    description: String,
}

impl ClientErrInfo {
    pub fn new<S: AsRef<str> + Display, T: AsRef<str> + Display>(title: S, description: T) -> Self {
        Self { title: title.to_string(), description: description.to_string() }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }
}

//to minimize boilerplate instead of writing ClientErrInfo::new(title, description) or (title.to_string, description.to_string) on all match arms
fn a<S: AsRef<str> + Display, T: AsRef<str> + Display>(title: S, description: T) -> (String, String) {
    (title.to_string(), description.to_string())
}

impl ClientError {
    pub fn get_description(&self) -> ClientErrInfo {
        let (title, description) = match self {
            // -- User
            Self::AccountCreateAccountAlreadyExists => a("Account already exists", "You already have a MathBot©™ account"),
            Self::FailedToCreateAccount(_) => a("Account creation failed", "An internal error happened"),
            Self::FailedToDeleteAccount(_) => a("Account deletion failed", "An internal error happened"),
            Self::UpdateAvatarInvalidAvatarUrl(url) => a("Invalid avatar url", format!("`{url}` is not a valid avatar url! Supported image formats are `png`, `jpeg`, `jpg`, `webp` and `gif`.")),
            Self::UpdateUsernameAlreadyExists(username) => a("Username update failed", format!("The username `@{username}` is already taken by another MathBot user! Please choose a different username")),
            Self::UpdateUsernameInvalidLength => a("Invalid username length", "Usernames must be 2-20 characters long"),
            Self::UpdateUsernameTooSoon(timestamp) => a("You're a little bit too quick", format!("You can update your username again in <t:{timestamp}:R>")),
            Self::UpdatePronounsInvalid => a("Invalid pronouns", "Pronouns must be 3-20 characters long and in the correct format"),
            // -- Currency
            Self::ItemInfoItemNotFound(item, _error) => a("Item not found", format!("Couldn't find an item matching `{item}`")),
            Self::TransferRecieverDoesntExist => a("Invalid reciever", "There is no MathBot©™ account connected to the user you specified"),
            Self::TransferRecieverIsSelf => a("Invalid reciever", "Did you just think you could fool the admins by transferring to yourself? Pathetic."),
            Self::TransferInvalidAmount(amount) => a("Invalid amount", format!("`{amount}` is not a valid amount")),
            Self::TransferTooSmallAmount => a("Invalid amount", "The minimum transfer amount is `100MTC$`"),
            Self::TransferInsufficientFunds => a("Insufficient funds", "After attempting to transfer the money you came to the conclusion that you're broke"),
            Self::GambleInvalidAmount(amount) => a("Invalid amount", format!("`{amount}` is not a valid amount")),
            Self::GambleTooLowAmount => a("Invalid amount", "Gambling amount must be greater than `100MTC$`"),
            Self::GambleTooHighAmount => a("Invalid amount", "Gambling amount must be less than `1,000,000MTC$`. We don't want to ruin the economy now, do we?"),
            Self::GambleInsufficientFunds => a("Insufficient funds", "Hey, wait a minute... you don't really have all that money do you? You see, we can't have people steal money from our precious gambling industry; Corporations are people too, my friend"),
            Self::SlotsTooLowAmount => a("Invalid amount", "Slots betting amount must be greater than `10MTC$`"),
            Self::SlotsTooHighAmount => a("Invalid amount", "Slots betting amount must be less than `100000MTC$`"),
            Self::ShopBuyItemNotFound(itemid) => a("Invalid item id", format!("There exists no item matching `{itemid}`")),
            Self::ShopBuyInsufficientFunds => a("Insufficient funds", "After attempting to purchase your items you came to the conclusion that you're broke"),
            Self::MineClaimNotOpenedYet => a("Nothing to claim", "You haven't initiated your mine yet"),
            Self::SlotBuyInsufficientFunds => a("Insufficient funds", "You are too broke to buy that new mine slot"),
            Self::MineSlotNotOwned => a("Invalid slot", "You don't own the slot you specified"),
            Self::MineSetInvalidItemId => a("Invalid item", "The item you specified does not exist"),
            Self::MineSetItemNotOwned => a("Invalid item", "You cannot set an item you don't own into your mine!"),
            Self::MineRemoveNothingToRemove => a("Nothing to remove", "There is no item present at the slot you specified"),
            // -- Fun
            Self::NoSayContent => a("Invalid input", "The bot is unable to send an empty message"),
            Self::RockPaperScissorsInvalidInput(i) => a("Invalid input", format!("`{i}` is not a valid choice. Valid choices are `rock`, `r`, `paper`, `p`, `scissors`, `s`.")),
            Self::AskNoOpenAiApiKey => a("Command currently unavailable", "This command is currently unavailable due to an invalid OpenAI token"),
            // -- Math
            Self::InvalidSolveExpression(expr) => a("Invalid expression", format!("`{expr}` is not a valid expression!")),
            Self::AnswerNoProblemInChannel(prefix) => a("Nothing to answer", format!("There is no ongoing math problem in this channel. To recieve one, execute `{prefix}simplemathproblem`")),
            Self::SolutionNoProblemInChannel(prefix) => a("Nothing to reveal the solution of", format!("There is no ongoing math problem in this channel. To recieve one, execute `{prefix}simplemathproblem`")),
            Self::InvalidFractionifyInput => a("Invalid input", "You must specify a decimal number, and optionally a repeating pattern wrapped in parenthesis, for example `1.2(3)` for `1.233333...`"),
            Self::PiDigitsTooHighAmount => a("Invalid amount", "Amount must be less than or equal to 2000, since that's the maximum discord message size"),
            Self::PiDigitsTooHighIndex => a("Invalid index", "Only 1 billion digits of pi are stored in our dataset"),
            Self::FibonacciTooHighInput => a("Number too high", "The bot cannot send a message longer than 2000 characters"),
            // -- Admin
            Self::AdminBanInvalidAccount => a("Invalid account", "There is no MathBot©™ account connected to the user you specified"),
            // -- Misc
            Self::AccountRequired(prefix) => a("🔒Account required", format!("To gain access to this command you must first create a MathBot©™ account\n\nTo create a MathBot©™ account, simply execute `{prefix}account create`")),
            Self::UserError(prefix) => {
                let err = get_user_error();
                a(&err, format!("To explain this error, execute `{prefix}--explain {err}`"))
            },
            Self::InvalidUserErrorExplainCode(code) => a("Invalid error code", format!("`{code}` is not a valid error code!")),
            Self::AccountIsBanned(unbanned) => a("You are banned!", format!("You have been banned by the admins, and will be unbanned <t:{unbanned}:R>. Your access to MathBot commands is limited during your ban period.")),
        };
        ClientErrInfo::new(title, description)
    }
}


const USER_ERRORS: &'static [&'static str] = &[
    "API error",
    "BIOS error",
    "USB error",
    "Error 40",
    "IBM error",
];

fn get_user_error() -> String {
    USER_ERRORS.choose(&mut rand::thread_rng()).unwrap_or(&"Error 40").to_string()
}
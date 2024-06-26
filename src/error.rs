use std::{error, fmt};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, strum_macros::AsRefStr)]
pub enum Error {
    // -- Misc errors
    Test,
}


// region:    --- Error boilerplate
impl error::Error for Error{}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.as_ref().to_string())
    }
}
// endregion: --- Error boilerplate

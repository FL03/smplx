/*
    Appellation: error <module>
    Contrib: @FL03
*/

/// a type alias for a [Result] with an error type of [SimplexError]
pub type Result<T = ()> = core::result::Result<T, SimplexError>;

#[derive(Clone, Debug, PartialEq, PartialOrd, thiserror::Error)]
pub enum SimplexError {
    #[error("[Angle Error] {0}")]
    AngleError(String),
    #[error("[Dimension Error] {expected} != {found}")]
    IncompatibleDimension { expected: usize, found: usize },
    #[error("[Unknown Error] {0}")]
    Unknown(String),
}

impl Default for SimplexError {
    fn default() -> Self {
        SimplexError::Unknown("An unknown error occurred".to_string())
    }
}

/*
    Appellation: error <module>
    Contrib: @FL03
*/

pub type Result<T = ()> = core::result::Result<T, SimplexError>;

#[derive(Clone, Debug, thiserror::Error)]
pub enum SimplexError {
    #[error("[Angle Error] {0}")]
    AngleError(String),
    #[error("[Dimension Error] {expected} != {found}")]
    IncompatibleDimension { expected: usize, found: usize },
    #[error("[Unknown Error] {0}")]
    Unknown(String),
}

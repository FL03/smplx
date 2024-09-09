/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Error {
    Unknown(String),
}

impl Error {
    pub fn unknown<T: Into<String>>(msg: T) -> Self {
        Self::Unknown(msg.into())
    }
}

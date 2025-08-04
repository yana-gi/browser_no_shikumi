use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Network(String),
    UnexpectedResponse(String),
    InvalidUI(String),
    Other(String)
}

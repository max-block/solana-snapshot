use solana_client::client_error::ClientError;

#[derive(Debug)]
pub enum Error {
    ClintError(String),
    NoGoodNodes,
    Error(String),
}

impl From<ClientError> for Error {
    fn from(e: ClientError) -> Self {
        Error::ClintError(e.to_string())
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Error(e)
    }
}

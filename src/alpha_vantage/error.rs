use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlphaVantageError {
    #[error(transparent)]
    ParseError(#[from] url::ParseError),

    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
}

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Arboard(#[from] arboard::Error),
    #[error(transparent)]
    Pareg(#[from] pareg::ArgError),
    #[error(transparent)]
    Termal(#[from] termal::Error),
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Missing cplipboard provider.")]
    NoClipboardProvider,
    #[error("Timeout for clipboard has passed.")]
    Timeout,
    #[error("{} |and| {}", .0.0, .0.1)]
    Double(Box<(Error, Error)>),
}

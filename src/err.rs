use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Pareg(#[from] pareg::ArgError),
    #[error(transparent)]
    Termal(#[from] termal::error::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Missing cplipboard provider.")]
    NoClipboardProvider,
    #[error("Timeout for clipboard has passed.")]
    Timeout,
}

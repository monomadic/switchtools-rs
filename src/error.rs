pub type Error = NSPError;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum NSPError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Static error: {0}")]
    Static(&'static str),
}

use derive_more::From;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From, PartialEq)]
pub enum Error {
    // Internal
    // Date
    DateInvalidDay,
    DateInvalidMonth,
    // External
    // #[from]
    // Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

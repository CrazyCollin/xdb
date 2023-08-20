use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AlreadyOpened,
    InvalidSavepoint,
    TooLargeValue,
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AlreadyOpened => write!(f, "Already opened"),
            Error::InvalidSavepoint => write!(f, "Invalid savepoint"),
            Error::TooLargeValue => write!(f, "Too large value"),
            Error::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum TransactionError{
    Storage(StorageError),
}

#[derive(Debug)]
pub enum StorageError{
    Corrupted(String),
    TooLargeValue(usize),
    Io(std::io::Error),
    LockPoisoned(&'static str),
}

#[derive(Debug)]
pub enum TableError{
    TableMisMatch,
    Storage(StorageError),
}

#[derive(Debug)]
pub enum DatabaseError{
    DatabaseAlreadyOpen,
    UpgradeRequired(u8),
    Storage(StorageError),
}


#[derive(Debug)]
pub enum SavepointError{

}

#[derive(Debug)]
pub enum CompactionError{
    PersistentSavepointExists,
    Storage(StorageError),
}





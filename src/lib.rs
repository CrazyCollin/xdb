mod transaction;
mod types;
mod error;
mod btree;
mod macros;
mod db;
mod table;

pub use error::{StorageError};


type Result<T=(),E=StorageError>=std::result::Result<T,E>;
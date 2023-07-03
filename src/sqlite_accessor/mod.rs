mod accessor;

mod queries;

mod client;
pub use client::Client;

mod error;
pub use error::Error;

pub type SqlitePoolOptions = sqlx::pool::PoolOptions<sqlx::sqlite::Sqlite>;

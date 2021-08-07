use std::env;
use std::marker::PhantomData;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use thiserror::Error;
use std::future::Future;
use diesel::connection::TransactionManager;

pub mod photo;
mod schema;
pub mod user;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct Dao<T> {
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Dao<T> {
    pub fn new() -> Self {
        Dao {
            _phantom: PhantomData,
        }
    }
}

pub struct Tx {}

impl Tx {
    pub fn run<R, F>(conn: &MysqlConnection, f: F) -> DaoResult<R>
        where
            F: FnOnce() -> DaoResult<R>,
    {
        conn.transaction(|| f())
    }

    pub async fn run_async<R, F>(conn: &MysqlConnection, f: F) -> DaoResult<R>
        where
            F: Future<Output = DaoResult<R>>,
    {
        let transaction_manager = conn.transaction_manager();
        transaction_manager.begin_transaction(conn)?;
        match f.await {
            Ok(value) => {
                transaction_manager.commit_transaction(conn)?;
                Ok(value)
            }
            Err(e) => {
                transaction_manager.rollback_transaction(conn)?;
                Err(e)
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum DaoError {
    #[error("notfound")]
    NotFound,
    #[error("forbidden")]
    Forbidden,
    #[error("internal error: {0}")]
    InternalError(String),
}

impl From<diesel::result::Error> for DaoError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => Self::NotFound,
            _ => Self::InternalError(e.to_string()),
        }
    }
}

impl From<String> for DaoError {
    fn from(v: String) -> Self {
        Self::InternalError(v)
    }
}

pub type DaoResult<R> = Result<R, DaoError>;

pub mod photo;
pub mod user;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::env;
use std::marker::PhantomData;
use thiserror::Error;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct Dao<T> {
    pub conn: MysqlConnection,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Dao<T> {
    pub fn new(conn: MysqlConnection) -> Self {
        Dao {
            conn,
            _phantom: PhantomData,
        }
    }

    pub fn tx<R, F>(&self, exec: F) -> DaoResult<R>
    where
        F: FnOnce() -> DaoResult<R>,
    {
        self.conn.transaction(|| exec())
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

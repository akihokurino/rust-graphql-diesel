mod errors;
mod me;
mod mutation;
mod other;
mod photo;
mod query;

use self::mutation::*;
use self::query::*;
use crate::graphql::me::*;
use crate::graphql::other::*;
use crate::graphql::photo::*;
use juniper::*;
use juniper_from_schema::graphql_schema_from_file;

use crate::ddb;
use diesel::MysqlConnection;
use std::sync::{Arc, Mutex, MutexGuard};

#[allow(unused)]
graphql_schema_from_file!("src/graphql/schema.graphql", context_type: Context);

pub struct Context {
    pub authorized_user_id: Option<String>,
    pub connection: Arc<Mutex<MysqlConnection>>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(authorized_user_id: Option<String>) -> Self {
        let conn_ref = Arc::new(Mutex::new(ddb::establish_connection()));
        Self {
            authorized_user_id,
            connection: Arc::clone(&conn_ref),
        }
    }

    pub fn ddb_dao<T>(&self) -> ddb::Dao<T> {
        ddb::Dao::new()
    }

    pub fn get_mutex_connection(&self) -> MutexGuard<MysqlConnection> {
        self.connection.lock().unwrap()
    }
}

pub fn new_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}

mod me;
mod query;

use self::query::*;
use crate::graphql::me::*;
use juniper::*;
use juniper_from_schema::graphql_schema_from_file;

#[allow(unused)]
graphql_schema_from_file!("src/graphql/schema.graphql", context_type: Context);

pub struct Context {}

impl juniper::Context for Context {}

pub fn new_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}

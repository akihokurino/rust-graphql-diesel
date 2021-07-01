use crate::domain::*;
use crate::graphql::me::Me;
use crate::graphql::Context;
use crate::graphql::*;
use async_trait::async_trait;
use juniper::{Executor, FieldResult};
use juniper_from_schema::{QueryTrail, Walked};

pub struct Query;

#[async_trait]
impl QueryFields for Query {
    async fn field_me<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Me, Walked>,
    ) -> FieldResult<Me> {
        Ok(Me {
            user: user::User {
                id: "1".to_string(),
                name: "akiho".to_string(),
            },
        })
    }
}

use crate::domain::*;
use crate::graphql::me::Me;
use crate::graphql::Context;
use crate::graphql::*;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
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
        let _ = exec.context();
        let now: DateTime<Utc> = Utc::now();

        Ok(Me {
            user: user::User {
                id: "1".to_string(),
                name: "akiho".to_string(),
                created_at: now.naive_utc(),
                updated_at: now.naive_utc(),
            },
        })
    }
}

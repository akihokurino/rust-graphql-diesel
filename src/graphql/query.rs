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
        let ctx = exec.context();
        let user_dao = ctx.ddb_dao::<user::User>();
        let authorized_user_id: FieldResult<String> = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"));
        if let Err(e) = authorized_user_id {
            return Err(e);
        }

        let user = user_dao.get(authorized_user_id.ok().unwrap());
        if let Err(e) = user {
            return Err(FieldError::from(e));
        }

        Ok(Me {
            user: user.unwrap(),
        })
    }
}

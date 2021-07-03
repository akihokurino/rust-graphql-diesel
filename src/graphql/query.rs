use crate::domain::*;
use crate::graphql::me::*;
use crate::graphql::other::*;
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
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        let user = user_dao.get(authorized_user_id).map_err(FieldError::from)?;

        Ok(Me { user })
    }

    async fn field_others<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, OtherConnection, Walked>,
    ) -> FieldResult<OtherConnection> {
        let ctx = exec.context();
        let user_dao = ctx.ddb_dao::<user::User>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        let others = user_dao
            .get_all_with_exclude(authorized_user_id)
            .map_err(FieldError::from)?;

        let edges = others
            .into_iter()
            .map(|v| OtherEdge {
                user_id: v.id.clone(),
            })
            .collect::<Vec<_>>();

        Ok(OtherConnection(edges))
    }
}

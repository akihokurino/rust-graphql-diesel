use crate::domain;
use crate::graphql::me::*;
use crate::graphql::other::*;
use crate::graphql::photo::*;
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
        let user_dao = ctx.ddb_dao::<domain::user::User>();
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
        let user_dao = ctx.ddb_dao::<domain::user::User>();
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

    async fn field_my_photos<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, PhotoConnection, Walked>,
    ) -> FieldResult<PhotoConnection> {
        let ctx = exec.context();
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        let photos = photo_dao
            .get_all_by_user(authorized_user_id)
            .map_err(FieldError::from)?;

        let edges = photos
            .into_iter()
            .map(|v| PhotoEdge {
                photo_id: v.id.clone(),
            })
            .collect::<Vec<_>>();

        Ok(PhotoConnection(edges))
    }

    async fn field_my_photo<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Photo, Walked>,
        input: MyPhotoInput,
    ) -> FieldResult<Photo> {
        let ctx = exec.context();
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        let id = input.id;

        let photo = photo_dao.get(id.clone()).map_err(FieldError::from)?;
        if photo.user_id != authorized_user_id {
            return Err(FieldError::from("forbidden"));
        }

        Ok(Photo { photo })
    }
}

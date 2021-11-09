use crate::domain;
use crate::graphql::me::*;
use crate::graphql::other::*;
use crate::graphql::photo::*;
use crate::graphql::Context;
use crate::graphql::*;
use async_trait::async_trait;
use juniper::{Executor, FieldResult};
use juniper_from_schema::{QueryTrail, Walked};
use errors::*;

pub struct Query;

#[async_trait]
impl QueryFields for Query {
    async fn field_me<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Me, Walked>,
    ) -> FieldResult<Me> {
        let ctx = exec.context();
        let conn = ctx.get_mutex_connection();
        let user_dao = ctx.ddb_dao::<domain::user::User>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldErrorWithCode::un_authenticate())?;

        let (user, photos) = user_dao
            .get_with_photos(&conn, authorized_user_id)
            .map_err(FieldErrorWithCode::from)?;

        Ok(Me { user, photos })
    }

    async fn field_others<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, OtherConnection, Walked>,
    ) -> FieldResult<OtherConnection> {
        let ctx = exec.context();
        let conn = ctx.get_mutex_connection();
        let user_dao = ctx.ddb_dao::<domain::user::User>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldErrorWithCode::un_authenticate())?;

        let others = user_dao
            .get_all_with_exclude(&conn, authorized_user_id)
            .map(|v| v.iter().map(|v| (v.to_owned(), None)).collect::<Vec<_>>())
            .map_err(FieldErrorWithCode::from)?;

        Ok(OtherConnection(others))
    }

    async fn field_all_users<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, OtherConnection, Walked>,
    ) -> FieldResult<OtherConnection> {
        let ctx = exec.context();
        let conn = ctx.get_mutex_connection();
        let user_dao = ctx.ddb_dao::<domain::user::User>();
        let _authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldErrorWithCode::un_authenticate())?;

        let others = user_dao
            .get_all_with_photos(&conn)
            .map(|v| v.iter().map(|v| (v.0.to_owned(), Some(v.1.to_owned()))).collect::<Vec<_>>())
            .map_err(FieldErrorWithCode::from)?;

        Ok(OtherConnection(others))
    }

    async fn field_photos<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, PhotoConnection, Walked>,
    ) -> FieldResult<PhotoConnection> {
        let ctx = exec.context();
        let conn = ctx.get_mutex_connection();
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldErrorWithCode::un_authenticate())?;

        let photos = photo_dao
            .get_all_by_user(&conn, authorized_user_id)
            .map_err(FieldErrorWithCode::from)?;

        Ok(PhotoConnection(
            photos.iter().map(|v| (v.to_owned(), None)).collect::<Vec<_>>(),
        ))
    }

    async fn field_photo<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Photo, Walked>,
        id: String,
    ) -> FieldResult<Photo> {
        let ctx = exec.context();
        let conn = ctx.get_mutex_connection();
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldErrorWithCode::un_authenticate())?;

        let photo = photo_dao.get(&conn, id.clone()).map_err(FieldErrorWithCode::from)?;
        if photo.user_id != authorized_user_id {
            return Err(FieldErrorWithCode::forbidden().into());
        }

        Ok(Photo { photo, user: None })
    }

    async fn field_all_photos<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, PhotoConnection, Walked>,
    ) -> FieldResult<PhotoConnection> {
        let ctx = exec.context();
        let conn = ctx.get_mutex_connection();
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();
        let _authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldErrorWithCode::un_authenticate())?;

        let photos = photo_dao.get_all_with_user(&conn).map_err(FieldErrorWithCode::from)?;

        Ok(PhotoConnection(
            photos
                .iter()
                .map(|v| (v.0.to_owned(), Some(v.1.to_owned())))
                .collect::<Vec<_>>(),
        ))
    }
}

use crate::ddb::{DaoError, Tx};
use crate::domain;
use crate::graphql::me::Me;
use crate::graphql::photo::Photo;
use crate::graphql::Context;
use crate::graphql::*;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use juniper::{Executor, FieldResult};
use juniper_from_schema::{QueryTrail, Walked};

pub struct Mutation;
#[async_trait]
impl MutationFields for Mutation {
    async fn field_sign_up<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Me, Walked>,
        input: SignUpInput,
    ) -> FieldResult<Me> {
        let ctx = exec.context();
        let conn = ddb::establish_connection();
        let user_dao = ctx.ddb_dao::<domain::user::User>();

        let now: DateTime<Utc> = Utc::now();
        let name = input.name;

        let user = domain::user::User::new(name, now);

        if let Err(e) = user_dao.insert(&conn, &user) {
            return Err(FieldError::from(e));
        }

        Ok(Me {
            user,
            photos: Vec::new(),
        })
    }

    async fn field_update_user_info<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Me, Walked>,
        input: UpdateUserInfoInput,
    ) -> FieldResult<Me> {
        let ctx = exec.context();
        let conn = ddb::establish_connection();
        let user_dao = ctx.ddb_dao::<domain::user::User>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        let now: DateTime<Utc> = Utc::now();
        let name = input.name;

        let user = Tx::run(&conn, || {
                let mut user = user_dao.get(&conn, authorized_user_id)?;

                user.update(name, now);

                user_dao.update(&conn, &user)?;

                Ok(user)
            })
            .map_err(FieldError::from)?;

        Ok(Me {
            user,
            photos: Vec::new(),
        })
    }

    async fn field_leave<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
    ) -> FieldResult<bool> {
        let ctx = exec.context();
        let conn = ddb::establish_connection();
        let user_dao = ctx.ddb_dao::<domain::user::User>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        if let Err(e) = user_dao.delete(&conn, authorized_user_id) {
            return Err(FieldError::from(e));
        }

        Ok(true)
    }

    async fn field_create_photo<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Photo, Walked>,
        input: CreatePhotoInput,
    ) -> FieldResult<Photo> {
        let ctx = exec.context();
        let conn = ddb::establish_connection();
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        let now: DateTime<Utc> = Utc::now();
        let url = input.url;
        let is_public = input.is_public;

        let photo = domain::photo::Photo::new(authorized_user_id, url, is_public, now);

        if let Err(e) = photo_dao.insert(&conn, &photo) {
            return Err(FieldError::from(e));
        }

        Ok(Photo { photo, user: None })
    }

    async fn field_update_photo<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Photo, Walked>,
        input: UpdatePhotoInput,
    ) -> FieldResult<Photo> {
        let ctx = exec.context();
        let conn = ddb::establish_connection();
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        let now: DateTime<Utc> = Utc::now();
        let id = input.id;
        let is_public = input.is_public;

        let photo = Tx::run(&conn, || {
                let mut photo = photo_dao.get(&conn, id.clone())?;
                if photo.user_id != authorized_user_id {
                    return Err(DaoError::Forbidden);
                }

                photo.update_visibility(is_public, now);

                photo_dao.update(&conn, &photo)?;

                Ok(photo)
            })
            .map_err(FieldError::from)?;

        Ok(Photo { photo, user: None })
    }

    async fn field_delete_photo<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        input: DeletePhotoInput,
    ) -> FieldResult<bool> {
        let ctx = exec.context();
        let conn = ddb::establish_connection();
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        let id = input.id;

        let photo = photo_dao.get(&conn, id.clone()).map_err(FieldError::from)?;
        if photo.user_id != authorized_user_id {
            return Err(FieldError::from("forbidden"));
        }

        if let Err(e) = photo_dao.delete(&conn, id.clone()) {
            return Err(FieldError::from(e));
        }

        Ok(true)
    }
}

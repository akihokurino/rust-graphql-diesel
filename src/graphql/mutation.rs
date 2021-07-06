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
        let user_dao = ctx.ddb_dao::<domain::user::User>();

        let now: DateTime<Utc> = Utc::now();
        let name = input.name;

        let user = domain::user::User::new(name, now);

        if let Err(e) = user_dao.insert(user.clone()) {
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
        let user_dao = ctx.ddb_dao::<domain::user::User>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        let now: DateTime<Utc> = Utc::now();
        let name = input.name;

        let mut user = user_dao.get(authorized_user_id).map_err(FieldError::from)?;

        user.update(name, now);

        if let Err(e) = user_dao.update(user.clone()) {
            return Err(FieldError::from(e));
        }

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
        let user_dao = ctx.ddb_dao::<domain::user::User>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        if let Err(e) = user_dao.delete(authorized_user_id) {
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
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        let now: DateTime<Utc> = Utc::now();
        let url = input.url;
        let is_public = input.is_public;

        let photo = domain::photo::Photo::new(authorized_user_id, url, is_public, now);

        if let Err(e) = photo_dao.insert(photo.clone()) {
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
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();
        let authorized_user_id = ctx
            .authorized_user_id
            .clone()
            .ok_or(FieldError::from("unauthorized"))?;

        let now: DateTime<Utc> = Utc::now();
        let id = input.id;
        let is_public = input.is_public;

        let mut photo = photo_dao.get(id.clone()).map_err(FieldError::from)?;
        if photo.user_id != authorized_user_id {
            return Err(FieldError::from("forbidden"));
        }

        photo.update_visibility(is_public, now);

        if let Err(e) = photo_dao.update(photo.clone()) {
            return Err(FieldError::from(e));
        }

        Ok(Photo { photo, user: None })
    }

    async fn field_delete_photo<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        input: DeletePhotoInput,
    ) -> FieldResult<bool> {
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

        if let Err(e) = photo_dao.delete(id.clone()) {
            return Err(FieldError::from(e));
        }

        Ok(true)
    }
}

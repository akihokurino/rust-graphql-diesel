use crate::domain::*;
use crate::graphql::me::Me;
use crate::graphql::Context;
use crate::graphql::*;
use crate::schema::users;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::{Executor, FieldResult};
use juniper_from_schema::{QueryTrail, Walked};
use uuid::Uuid;

use crate::ddb;
use crate::ddb::user::Entity;

pub struct Mutation;

#[async_trait]
impl MutationFields for Mutation {
    async fn field_sign_up<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Me, Walked>,
        input: SignUpInput,
    ) -> FieldResult<Me> {
        let _ = exec.context();
        let conn = ddb::establish_connection();
        let now: DateTime<Utc> = Utc::now();

        let name = input.name;

        let new_user = user::User {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            created_at: now.naive_utc(),
            updated_at: now.naive_utc(),
        };

        let e: Entity = new_user.clone().into();
        diesel::insert_into(users::table)
            .values(e)
            .execute(&conn)
            .expect("error in save");

        Ok(Me { user: new_user })
    }
}

use crate::domain::*;
use crate::graphql::me::Me;
use crate::graphql::Context;
use crate::graphql::*;
use async_trait::async_trait;
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
        let name = input.name;
        println!("{}", name);
        Ok(Me {
            user: user::User {
                id: "1".to_string(),
                name: "akiho".to_string(),
            },
        })
    }
}

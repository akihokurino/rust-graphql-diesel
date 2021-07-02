use crate::domain;
use crate::schema::*;
use std::convert::TryFrom;

#[derive(Queryable, Insertable, Debug, Clone, Eq, PartialEq)]
#[table_name = "users"]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl TryFrom<Entity> for domain::user::User {
    type Error = String;

    fn try_from(e: Entity) -> Result<Self, Self::Error> {
        Ok(domain::user::User {
            id: e.id.to_string(),
            name: e.name.to_string(),
            created_at: e.created_at,
            updated_at: e.updated_at,
        })
    }
}

impl From<domain::user::User> for Entity {
    fn from(d: domain::user::User) -> Entity {
        Entity {
            id: d.id,
            name: d.name,
            created_at: d.created_at,
            updated_at: d.updated_at,
        }
    }
}

use crate::ddb::photo;
use crate::ddb::{Dao, DaoError, DaoResult};
use crate::domain;
use crate::schema::users;
use diesel::prelude::*;
use std::convert::TryFrom;

#[derive(Queryable, Insertable, Debug, Clone, Eq, PartialEq, Identifiable)]
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

impl Dao<domain::user::User> {
    pub fn get_all_with_exclude(&self, exclude_id: String) -> DaoResult<Vec<domain::user::User>> {
        return users::table
            .filter(users::id.ne(exclude_id))
            .order(users::created_at.desc())
            .load::<Entity>(&self.conn)
            .map(|v: Vec<Entity>| {
                v.into_iter()
                    .map(|v| domain::user::User::try_from(v).unwrap())
                    .collect::<Vec<_>>()
            })
            .map_err(DaoError::from);
    }

    pub fn get(&self, id: String) -> DaoResult<domain::user::User> {
        users::table
            .find(id)
            .first(&self.conn)
            .map(|v: Entity| domain::user::User::try_from(v).unwrap())
            .map_err(DaoError::from)
    }

    pub fn get_with_photos(
        &self,
        id: String,
    ) -> DaoResult<(domain::user::User, Vec<domain::photo::Photo>)> {
        let user_entity = users::table
            .find(id)
            .first::<Entity>(&self.conn)
            .map_err(DaoError::from)?;

        let photo_entities = photo::Entity::belonging_to(&user_entity)
            .load::<photo::Entity>(&self.conn)
            .map_err(DaoError::from)?;

        Ok((
            domain::user::User::try_from(user_entity).unwrap(),
            photo_entities
                .into_iter()
                .map(|v| domain::photo::Photo::try_from(v).unwrap())
                .collect::<Vec<_>>(),
        ))
    }

    pub fn insert(&self, item: domain::user::User) -> DaoResult<domain::user::User> {
        let e: Entity = item.clone().into();
        if let Err(e) = diesel::insert_into(users::table)
            .values(e)
            .execute(&self.conn)
            .map_err(DaoError::from)
        {
            return Err(e);
        }
        Ok(item)
    }

    pub fn update(&self, item: domain::user::User) -> DaoResult<domain::user::User> {
        let e: Entity = item.clone().into();
        if let Err(e) = diesel::update(users::table.find(e.id))
            .set((users::name.eq(e.name), users::updated_at.eq(e.updated_at)))
            .execute(&self.conn)
            .map_err(DaoError::from)
        {
            return Err(e);
        }
        Ok(item)
    }

    pub fn delete(&self, id: String) -> DaoResult<bool> {
        if let Err(e) = diesel::delete(users::table.find(id))
            .execute(&self.conn)
            .map_err(DaoError::from)
        {
            return Err(e);
        }
        Ok(true)
    }
}

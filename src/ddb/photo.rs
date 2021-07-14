use crate::ddb::photo;
use crate::ddb::schema::photos;
use crate::ddb::schema::users;
use crate::ddb::user;
use crate::ddb::{Dao, DaoError, DaoResult};
use crate::domain;
use diesel::prelude::*;
use std::convert::TryFrom;

#[derive(Queryable, Insertable, Debug, Clone, Eq, PartialEq, Identifiable, Associations)]
#[belongs_to(user::Entity, foreign_key = "user_id")]
#[table_name = "photos"]
pub struct Entity {
    pub id: String,
    pub user_id: String,
    pub url: String,
    pub is_public: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl TryFrom<Entity> for domain::photo::Photo {
    type Error = String;

    fn try_from(e: Entity) -> Result<Self, Self::Error> {
        Ok(domain::photo::Photo {
            id: e.id,
            user_id: e.user_id,
            url: e.url,
            is_public: e.is_public,
            created_at: e.created_at,
            updated_at: e.updated_at,
        })
    }
}

impl From<domain::photo::Photo> for Entity {
    fn from(d: domain::photo::Photo) -> Entity {
        Entity {
            id: d.id,
            user_id: d.user_id,
            url: d.url,
            is_public: d.is_public,
            created_at: d.created_at,
            updated_at: d.updated_at,
        }
    }
}

impl Dao<domain::photo::Photo> {
    pub fn get_all_with_user(&self) -> DaoResult<Vec<(domain::photo::Photo, domain::user::User)>> {
        let join = photos::table.inner_join(users::table);

        join.load::<(photo::Entity, user::Entity)>(&self.conn)
            .map(|v: Vec<(photo::Entity, user::Entity)>| {
                v.into_iter()
                    .map(|v| {
                        (
                            domain::photo::Photo::try_from(v.0).unwrap(),
                            domain::user::User::try_from(v.1).unwrap(),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .map_err(DaoError::from)
    }

    pub fn get_all_by_user(&self, user_id: String) -> DaoResult<Vec<domain::photo::Photo>> {
        return photos::table
            .filter(photos::user_id.eq(user_id))
            .order(photos::created_at.desc())
            .load::<Entity>(&self.conn)
            .map(|v: Vec<Entity>| {
                v.into_iter()
                    .map(|v| domain::photo::Photo::try_from(v).unwrap())
                    .collect::<Vec<_>>()
            })
            .map_err(DaoError::from);
    }

    pub fn get(&self, id: String) -> DaoResult<domain::photo::Photo> {
        photos::table
            .find(id)
            .first(&self.conn)
            .map(|v: Entity| domain::photo::Photo::try_from(v).unwrap())
            .map_err(DaoError::from)
    }

    pub fn insert(&self, item: &domain::photo::Photo) -> DaoResult<()> {
        let e: Entity = item.clone().into();
        if let Err(e) = diesel::insert_into(photos::table)
            .values(e)
            .execute(&self.conn)
            .map_err(DaoError::from)
        {
            return Err(e);
        }
        Ok(())
    }

    pub fn update(&self, item: &domain::photo::Photo) -> DaoResult<()> {
        let e: Entity = item.clone().into();
        if let Err(e) = diesel::update(photos::table.find(e.id))
            .set((
                photos::is_public.eq(e.is_public),
                photos::updated_at.eq(e.updated_at),
            ))
            .execute(&self.conn)
            .map_err(DaoError::from)
        {
            return Err(e);
        }
        Ok(())
    }

    pub fn delete(&self, id: String) -> DaoResult<bool> {
        if let Err(e) = diesel::delete(photos::table.find(id))
            .execute(&self.conn)
            .map_err(DaoError::from)
        {
            return Err(e);
        }
        Ok(true)
    }
}

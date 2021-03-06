use crate::ddb::photo;
use crate::ddb::schema::photos;
use crate::ddb::schema::users;
use crate::ddb::{Dao, DaoError, DaoResult};
use crate::domain;
use async_trait::async_trait;
use dataloader::{cached, BatchFn};
use diesel::prelude::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};

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
    pub fn get_all_with_photos(
        &self,
        conn: &MysqlConnection,
    ) -> DaoResult<Vec<(domain::user::User, Vec<domain::photo::Photo>)>> {
        let user_entities = users::table
            .order(users::created_at.desc())
            .load::<Entity>(conn)
            .map_err(DaoError::from)?;

        let photo_entities = photo::Entity::belonging_to(&user_entities)
            .order(photos::created_at.desc())
            .load::<photo::Entity>(conn)
            .map_err(DaoError::from)?
            .grouped_by(&user_entities);

        let zipped = user_entities
            .into_iter()
            .zip(photo_entities)
            .map(|v: (Entity, Vec<photo::Entity>)| {
                (
                    domain::user::User::try_from(v.0).unwrap(),
                    v.1.into_iter()
                        .map(|v| domain::photo::Photo::try_from(v).unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        Ok(zipped)
    }

    pub fn get_all_with_exclude(
        &self,
        conn: &MysqlConnection,
        exclude_id: String,
    ) -> DaoResult<Vec<domain::user::User>> {
        return users::table
            .filter(users::id.ne(exclude_id))
            .order(users::created_at.desc())
            .load::<Entity>(conn)
            .map(|v: Vec<Entity>| {
                v.into_iter()
                    .map(|v| domain::user::User::try_from(v).unwrap())
                    .collect::<Vec<_>>()
            })
            .map_err(DaoError::from);
    }

    pub fn get(&self, conn: &MysqlConnection, id: String) -> DaoResult<domain::user::User> {
        users::table
            .find(id)
            .first(conn)
            .map(|v: Entity| domain::user::User::try_from(v).unwrap())
            .map_err(DaoError::from)
    }

    pub fn get_with_photos(
        &self,
        conn: &MysqlConnection,
        id: String,
    ) -> DaoResult<(domain::user::User, Vec<domain::photo::Photo>)> {
        let user_entity = users::table
            .find(id)
            .first::<Entity>(conn)
            .map_err(DaoError::from)?;

        let photo_entities = photo::Entity::belonging_to(&user_entity)
            .order(photos::created_at.desc())
            .load::<photo::Entity>(conn)
            .map_err(DaoError::from)?;

        Ok((
            domain::user::User::try_from(user_entity).unwrap(),
            photo_entities
                .into_iter()
                .map(|v| domain::photo::Photo::try_from(v).unwrap())
                .collect::<Vec<_>>(),
        ))
    }

    pub fn insert(&self, conn: &MysqlConnection, item: &domain::user::User) -> DaoResult<()> {
        let e: Entity = item.clone().into();
        if let Err(e) = diesel::insert_into(users::table)
            .values(e)
            .execute(conn)
            .map_err(DaoError::from)
        {
            return Err(e);
        }
        Ok(())
    }

    pub fn update(&self, conn: &MysqlConnection, item: &domain::user::User) -> DaoResult<()> {
        let e: Entity = item.clone().into();
        if let Err(e) = diesel::update(users::table.find(e.id))
            .set((users::name.eq(e.name), users::updated_at.eq(e.updated_at)))
            .execute(conn)
            .map_err(DaoError::from)
        {
            return Err(e);
        }
        Ok(())
    }

    pub fn delete(&self, conn: &MysqlConnection, id: String) -> DaoResult<bool> {
        if let Err(e) = diesel::delete(users::table.find(id))
            .execute(conn)
            .map_err(DaoError::from)
        {
            return Err(e);
        }
        Ok(true)
    }

    fn batch_get(
        &self,
        conn: &MysqlConnection,
        hashmap: &mut HashMap<String, DaoResult<Vec<domain::user::User>>>,
        ids: Vec<String>,
    ) {
        let result: DaoResult<Vec<domain::user::User>> = users::table
            .filter(users::id.eq_any(ids.clone()))
            .order(users::created_at.desc())
            .load::<Entity>(conn)
            .map(|v: Vec<Entity>| {
                v.into_iter()
                    .map(|v| domain::user::User::try_from(v).unwrap())
                    .collect::<Vec<_>>()
            })
            .map_err(DaoError::from);

        if let Err(e) = result {
            for id in ids {
                hashmap.insert(id, Err(e.to_owned()));
            }
            return;
        }

        let items = result.unwrap();

        for id in ids {
            let mut vec = vec![];
            for row in items.iter().filter(|v| v.id == id) {
                vec.push(row.to_owned());
            }
            hashmap.insert(id.to_owned(), Ok(vec));
        }
    }
}

pub struct BatchImpl {
    dao: Dao<domain::user::User>,
    conn: Arc<Mutex<MysqlConnection>>,
}

#[async_trait]
impl BatchFn<String, DaoResult<Vec<domain::user::User>>> for BatchImpl {
    async fn load(
        &mut self,
        keys: &[String],
    ) -> HashMap<String, DaoResult<Vec<domain::user::User>>> {
        let conn = self.conn.lock().unwrap();
        let mut hashmap = HashMap::new();
        self.dao.batch_get(&conn, &mut hashmap, keys.to_vec());
        hashmap
    }
}

impl BatchImpl {
    pub fn new_loader(conn: Arc<Mutex<MysqlConnection>>) -> Loader {
        cached::Loader::new(BatchImpl {
            dao: Dao::new(),
            conn,
        })
        .with_max_batch_size(100)
    }
}

pub type Loader = cached::Loader<String, DaoResult<Vec<domain::user::User>>, BatchImpl>;

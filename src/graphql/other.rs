use crate::domain;
use crate::graphql::*;
use errors::*;
use juniper_from_schema::{QueryTrail, Walked};

#[derive(Debug, Clone)]
pub struct Other {
    pub user: domain::user::User,
    pub photos: Option<Vec<domain::photo::Photo>>,
}
#[async_trait]
impl OtherFields for Other {
    fn field_id(&self, _: &Executor<Context>) -> FieldResult<ID> {
        Ok(Into::into(self.user.id.clone()))
    }

    fn field_name(&self, _: &Executor<Context>) -> FieldResult<String> {
        Ok(self.user.name.clone())
    }

    async fn field_photos<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Photo, Walked>,
    ) -> FieldResult<Vec<Photo>> {
        let ctx = exec.context();
        let conn = ctx.get_mutex_connection();
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();

        if let Some(photos) = self.photos.clone() {
            return Ok(photos
                .iter()
                .map(|v| Photo {
                    photo: v.to_owned(),
                    user: None,
                })
                .collect::<Vec<_>>());
        }

        let photos = photo_dao
            .get_all_by_user(&conn, self.user.id.clone())
            .map_err(FieldErrorWithCode::from)?;

        Ok(photos
            .iter()
            .map(|v| Photo {
                photo: v.to_owned(),
                user: None,
            })
            .collect::<Vec<_>>())
    }

    async fn field_load_photos<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Photo, Walked>,
    ) -> FieldResult<Vec<Photo>> {
        let ctx = exec.context();

        let photos: Vec<domain::photo::Photo> = ctx.photo_loader.load(self.user.id.clone()).await?;

        Ok(photos
            .iter()
            .map(|v| Photo {
                photo: v.to_owned(),
                user: None,
            })
            .collect::<Vec<_>>())
    }
}

#[derive(Debug, Clone)]
pub struct OtherEdge {
    pub user: domain::user::User,
    pub photos: Option<Vec<domain::photo::Photo>>,
}
#[async_trait]
impl OtherEdgeFields for OtherEdge {
    async fn field_node<'s, 'r, 'a>(
        &'s self,
        _exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Other, Walked>,
    ) -> FieldResult<Other> {
        Ok(Other {
            user: self.user.clone(),
            photos: self.photos.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct OtherConnection(pub Vec<(domain::user::User, Option<Vec<domain::photo::Photo>>)>);
#[async_trait]
impl OtherConnectionFields for OtherConnection {
    async fn field_edges<'s, 'r, 'a>(
        &'s self,
        _exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, OtherEdge, Walked>,
    ) -> FieldResult<Vec<OtherEdge>> {
        let edges = self
            .0
            .iter()
            .map(|v| OtherEdge {
                user: v.0.to_owned(),
                photos: v.1.to_owned(),
            })
            .collect::<Vec<_>>();
        Ok(edges)
    }
}

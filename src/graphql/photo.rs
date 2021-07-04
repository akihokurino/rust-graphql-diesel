use crate::domain;
use crate::graphql::*;
use juniper_from_schema::{QueryTrail, Walked};

#[derive(Debug, Clone)]
pub struct Photo {
    pub photo: domain::photo::Photo,
}
impl PhotoFields for Photo {
    fn field_id(&self, _: &Executor<Context>) -> FieldResult<ID> {
        Ok(Into::into(self.photo.id.clone()))
    }

    fn field_user_id(&self, _: &Executor<Context>) -> FieldResult<ID> {
        Ok(Into::into(self.photo.user_id.clone()))
    }

    fn field_url(&self, _: &Executor<Context>) -> FieldResult<String> {
        Ok(self.photo.url.clone())
    }

    fn field_is_public(&self, _: &Executor<Context>) -> FieldResult<bool> {
        Ok(self.photo.is_public)
    }
}

#[derive(Debug, Clone)]
pub struct PhotoEdge {
    pub photo_id: String,
}
#[async_trait]
impl PhotoEdgeFields for PhotoEdge {
    async fn field_node<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        trail: &QueryTrail<'r, Photo, Walked>,
    ) -> FieldResult<Photo> {
        resolve_photo(exec, trail, self.photo_id.clone()).await
    }
}

#[derive(Debug, Clone)]
pub struct PhotoConnection(pub Vec<PhotoEdge>);
#[async_trait]
impl PhotoConnectionFields for PhotoConnection {
    async fn field_edges<'s, 'r, 'a>(
        &'s self,
        _exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, PhotoEdge, Walked>,
    ) -> FieldResult<Vec<PhotoEdge>> {
        Ok(self.0.clone())
    }
}

pub async fn resolve_photo<'r, 'a>(
    exec: &Executor<'r, 'a, Context>,
    _: &QueryTrail<'r, Photo, Walked>,
    id: String,
) -> FieldResult<Photo> {
    let photo = exec
        .context()
        .ddb_dao::<domain::photo::Photo>()
        .get(id)
        .map_err(FieldError::from)?;
    Ok(Photo { photo })
}

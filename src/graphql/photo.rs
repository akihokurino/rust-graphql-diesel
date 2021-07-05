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
    pub photo: domain::photo::Photo,
}
#[async_trait]
impl PhotoEdgeFields for PhotoEdge {
    async fn field_node<'s, 'r, 'a>(
        &'s self,
        _exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, Photo, Walked>,
    ) -> FieldResult<Photo> {
        Ok(Photo {
            photo: self.photo.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PhotoConnection(pub Vec<domain::photo::Photo>);
#[async_trait]
impl PhotoConnectionFields for PhotoConnection {
    async fn field_edges<'s, 'r, 'a>(
        &'s self,
        _exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, PhotoEdge, Walked>,
    ) -> FieldResult<Vec<PhotoEdge>> {
        let edges = self
            .0
            .clone()
            .into_iter()
            .map(|v| PhotoEdge { photo: v.clone() })
            .collect::<Vec<_>>();
        Ok(edges)
    }
}

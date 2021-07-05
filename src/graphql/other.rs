use crate::domain;
use crate::graphql::*;
use juniper_from_schema::{QueryTrail, Walked};

#[derive(Debug, Clone)]
pub struct Other {
    pub user: domain::user::User,
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
        _: &QueryTrail<'r, photo::PhotoConnection, Walked>,
    ) -> FieldResult<photo::PhotoConnection> {
        let ctx = exec.context();
        let photo_dao = ctx.ddb_dao::<domain::photo::Photo>();

        let photos = photo_dao
            .get_all_by_user(self.user.id.clone())
            .map_err(FieldError::from)?;

        Ok(photo::PhotoConnection(photos.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct OtherEdge {
    pub user: domain::user::User,
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
        })
    }
}

#[derive(Debug, Clone)]
pub struct OtherConnection(pub Vec<domain::user::User>);
#[async_trait]
impl OtherConnectionFields for OtherConnection {
    async fn field_edges<'s, 'r, 'a>(
        &'s self,
        _exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, OtherEdge, Walked>,
    ) -> FieldResult<Vec<OtherEdge>> {
        let edges = self
            .0
            .clone()
            .into_iter()
            .map(|v| OtherEdge { user: v.clone() })
            .collect::<Vec<_>>();
        Ok(edges)
    }
}

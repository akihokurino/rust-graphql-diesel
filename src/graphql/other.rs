use crate::domain;
use crate::graphql::*;
use juniper_from_schema::{QueryTrail, Walked};

#[derive(Debug, Clone)]
pub struct Other {
    pub user: domain::user::User,
}
impl OtherFields for Other {
    fn field_id(&self, _: &Executor<Context>) -> FieldResult<ID> {
        Ok(Into::into(self.user.id.clone()))
    }

    fn field_name(&self, _: &Executor<Context>) -> FieldResult<String> {
        Ok(self.user.name.clone())
    }
}

#[derive(Debug, Clone)]
pub struct OtherEdge {
    pub user_id: String,
}
#[async_trait]
impl OtherEdgeFields for OtherEdge {
    async fn field_node<'s, 'r, 'a>(
        &'s self,
        exec: &Executor<'r, 'a, Context>,
        trail: &QueryTrail<'r, Other, Walked>,
    ) -> FieldResult<Other> {
        resolve_other(exec, trail, self.user_id.clone()).await
    }
}

#[derive(Debug, Clone)]
pub struct OtherConnection(pub Vec<OtherEdge>);
#[async_trait]
impl OtherConnectionFields for OtherConnection {
    async fn field_edges<'s, 'r, 'a>(
        &'s self,
        _exec: &Executor<'r, 'a, Context>,
        _: &QueryTrail<'r, OtherEdge, Walked>,
    ) -> FieldResult<Vec<OtherEdge>> {
        Ok(self.0.clone())
    }
}

pub async fn resolve_other<'r, 'a>(
    exec: &Executor<'r, 'a, Context>,
    _: &QueryTrail<'r, Other, Walked>,
    user_id: String,
) -> FieldResult<Other> {
    let user = exec
        .context()
        .ddb_dao::<domain::user::User>()
        .get(user_id)
        .map_err(FieldError::from)?;
    Ok(Other { user })
}

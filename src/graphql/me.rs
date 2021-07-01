use crate::domain::*;
use crate::graphql::*;

#[derive(Debug, Clone)]
pub struct Me {
    pub user: user::User,
}

impl MeFields for Me {
    fn field_id(&self, _: &Executor<Context>) -> FieldResult<String> {
        Ok(self.user.id.clone())
    }

    fn field_name(&self, _: &Executor<Context>) -> FieldResult<String> {
        Ok(self.user.name.clone())
    }
}

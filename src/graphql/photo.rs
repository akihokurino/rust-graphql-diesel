use crate::domain;
use crate::graphql::*;

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

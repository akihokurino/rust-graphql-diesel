use crate::domain;
use crate::graphql::*;
use juniper_from_schema::{QueryTrail, Walked};

#[derive(Debug, Clone)]
pub struct Me {
    pub user: domain::user::User,
    pub photos: Vec<domain::photo::Photo>,
}
#[async_trait]
impl MeFields for Me {
    fn field_id(&self, _: &Executor<Context>) -> FieldResult<ID> {
        Ok(Into::into(self.user.id.clone()))
    }

    fn field_name(&self, _: &Executor<Context>) -> FieldResult<String> {
        Ok(self.user.name.clone())
    }

    fn field_photos<'r>(
        &self,
        _: &Executor<Context>,
        _: &QueryTrail<'r, Photo, Walked>,
    ) -> FieldResult<Vec<Photo>> {
        Ok(self
            .photos
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

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Photo {
    pub id: String,
    pub user_id: String,
    pub url: String,
    pub is_public: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Photo {
    pub fn new(user_id: String, url: String, is_public: bool, now: DateTime<Utc>) -> Self {
        Photo {
            id: Uuid::new_v4().to_string(),
            user_id,
            url,
            is_public,
            created_at: now.naive_utc(),
            updated_at: now.naive_utc(),
        }
    }

    pub fn update_visibility(&mut self, is_public: bool, now: DateTime<Utc>) {
        self.is_public = is_public;
        self.updated_at = now.naive_utc();
    }
}

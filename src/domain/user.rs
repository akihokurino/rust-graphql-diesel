#[derive(Debug, Clone, Eq, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

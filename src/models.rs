use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct Quote {
    pub(crate) id: Uuid,
    pub(crate) book: String,
    pub(crate) quote: String,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    pub(crate) fn new(book: String, quote: String) -> Self {
        let now = chrono::Utc::now();

        Self {
            id: Uuid::new_v4(),
            book,
            quote,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    pub(crate) book: String,
    pub(crate) quote: String,
}

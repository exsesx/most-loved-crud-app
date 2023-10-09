use axum::{extract, http};
use serde::{Deserialize, Serialize};
use std::os::macos::raw::stat;

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct Quote {
    id: Uuid,
    book: String,
    quote: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    fn new(book: String, quote: String) -> Self {
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
    book: String,
    quote: String,
}

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn create_quote(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateQuote>,
) -> Result<(http::StatusCode, axum::Json<Quote>), http::StatusCode> {
    let quote = Quote::new(payload.book, payload.quote);

    let res = sqlx::query(
        r#"
        INSERT INTO quotes (id, book, quote, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
    "#,
    )
    .bind(&quote.id)
    .bind(&quote.book)
    .bind(&quote.quote)
    .bind(&quote.created_at)
    .bind(&quote.updated_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, axum::Json(quote))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn read_quotes(
    extract::State(pool): extract::State<PgPool>,
) -> Result<axum::Json<Vec<Quote>>, http::StatusCode> {
    let res = sqlx::query_as::<_, Quote>("SELECT * FROM quotes")
        .fetch_all(&pool)
        .await;

    match res {
        Ok(quotes) => Ok(axum::Json(quotes)),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_quote(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<Uuid>,
    axum::Json(payload): axum::Json<CreateQuote>,
) -> http::StatusCode {
    let now = chrono::Utc::now();

    let res = sqlx::query(
        r#"
        UPDATE quotes
        SET book = $1, quote = $2, updated_at = $3
        WHERE id = $4
        "#,
    )
    .bind(payload.book)
    .bind(payload.quote)
    .bind(now)
    .bind(id)
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => http::StatusCode::NOT_FOUND,
        _ => http::StatusCode::OK,
    });

    match res {
        Ok(status) => status,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_quote(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<Uuid>,
) -> http::StatusCode {
    let res = sqlx::query(
        r#"
    DELETE FROM quotes WHERE id = $4
        "#,
    )
    .bind(id)
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => http::StatusCode::NOT_FOUND,
        _ => http::StatusCode::OK,
    });

    match res {
        Ok(status) => status,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

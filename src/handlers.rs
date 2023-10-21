use axum::{
    extract::{self, Path},
    http,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuote {
    book: Option<String>,
    quote: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Quote {
    id: uuid::Uuid,
    book: String,
    quote: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    fn new(book: String, quote: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            book,
            quote,
            created_at: now,
            updated_at: now,
        }
    }
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

pub async fn update_quote(
    extract::State(pool): extract::State<PgPool>,
    Path(id): Path<uuid::Uuid>,
    axum::Json(payload): axum::Json<UpdateQuote>,
) -> Result<(http::StatusCode, axum::Json<Quote>), http::StatusCode> {
    let now = chrono::Utc::now();

    let res = sqlx::query_as::<_,Quote>(
        r#"
          UPDATE quotes 
          SET book = COALESCE($2, book), quote = COALESCE($3, quote), updated_at = COALESCE($4, updated_at)
          WHERE id = $1
          RETURNING *
        "#,
    )
    .bind(id)
    .bind(&payload.book )
    .bind(&payload.quote)
    .bind(now)
    .fetch_one(&pool)
    .await;

    match res {
        Ok(quote) => Ok((http::StatusCode::CREATED, axum::Json(quote))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn read_quotes(
    extract::State(pool): extract::State<PgPool>,
) -> Result<axum::Json<Vec<Quote>>, http::StatusCode> {
    let res = sqlx::query_as::<_, Quote>(
        r#"
          SELECT * FROM quotes
        "#,
    )
    .fetch_all(&pool)
    .await;

    match res {
        Ok(data) => Ok(axum::Json(data)),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

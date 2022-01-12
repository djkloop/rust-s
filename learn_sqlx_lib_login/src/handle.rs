use serde::Serialize;
use serde_json::json;
use sqlx::{Acquire, Postgres};
use tide_sqlx::SQLxRequestExt;

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Record {
    name: String,
    dept: String,
}

pub async fn get_hello(_req: tide::Request<()>) -> tide::Result {
    Ok("Api v1 Get Hello!".into())
}

pub async fn get_world(_req: tide::Request<()>) -> tide::Result {
    Ok("Api v2 Get World!".into())
}

pub async fn users(req: tide::Request<()>) -> tide::Result {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let users = sqlx::query_as::<Postgres, Record>("select name, dept from company")
        .fetch_all(pg_conn.acquire().await?)
        .await?;
    Ok(json!(users).into())
}

extern crate pretty_env_logger;

#[macro_use]
extern crate log;

use std::env;

use dotenv::dotenv;
use serde_json::json;
use sqlx::{PgPool, Pool, query_as};
use tide::{Server, Response, StatusCode, Request, Body};

mod entity;

async fn make_db_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL").unwrap();
    info!("make db pool with url -> {}", db_url);
    Pool::connect(&db_url).await.unwrap()
}

#[derive(Debug, Clone)]
struct State {
    db_pool: PgPool,
}

async fn server() -> tide::Result<Server<State>> {
    dotenv().ok();
    pretty_env_logger::init();
    let db_pool = make_db_pool().await;
    let state = State {
        db_pool
    };
    let mut app = Server::with_state(state);
    app.at("/users").get(|req: Request<State>| async move {
        let db_share_pool = &req.state().db_pool;
        // 获取到数据库里面所有的用户...
        let users = query_as!(entity::user::User, "select id, username from users").fetch_all(db_share_pool).await?;
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(Body::from_json(&json!({ "users": &users }))?);
        Ok(res)
    });
    Ok(app)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let app = server().await?;
    app.listen("127.0.0.1:8888").await?;
    Ok(())
}

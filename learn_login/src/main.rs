extern crate pretty_env_logger;

#[macro_use]
extern crate log;

use std::env;

use chrono::Utc;

use argon2::Config;
use dotenv::dotenv;
use serde::Serialize;
use serde_json::json;
use sqlx::{query, query_as, PgPool, Pool};
use tide::{Body, Request, Response, Server, StatusCode};

mod entity;

// 创建一个共有的 pg pool
async fn make_db_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL").unwrap();
    info!("make db pool with url -> {}", db_url);
    Pool::connect(&db_url).await.unwrap()
}

#[derive(Debug, Clone)]
struct State {
    db_pool: PgPool,
}

#[derive(Debug, Serialize)]
struct ResultMessage {
    token: String,
    message: String
}

async fn server() -> tide::Result<Server<State>> {
    dotenv().ok();
    pretty_env_logger::init();
    let db_pool = make_db_pool().await;
    let state = State { db_pool };
    let mut app = Server::with_state(state);
    app.at("/users")
        // get 获取所有用户
        .get(|req: Request<State>| async move {
            let db_share_pool = &req.state().db_pool;
            // 获取到数据库里面所有的用户...
            let users = query_as!(entity::user::User, "select id, username from users")
                .fetch_all(db_share_pool)
                .await?;
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&json!({ "users": &users }))?);
            Ok(res)
        })
        // 创建用户
        .post(|mut req: Request<State>| async move {
            let db_share_pool = req.state().db_pool.clone();
            // 先从body里获取数据
            let create_user = req.body_json::<entity::user::CreateUser>().await?;
            // 然后把密码加一遍hash
            let password = create_user.password.as_bytes();
            let salt = env::var("SALT").unwrap();
            let config = Config::default();
            let hashed_password = argon2::hash_encoded(password, salt.as_bytes(), &config).unwrap();
            // 存数据库
            let row = query!(
                r#"
                    insert into users (id, username, hashed_password, create_at, updated_at) 
                    values ($1, $2, $3, $4, $5);
                "#,
                uuid::Uuid::new_v4(),
                create_user.username,
                hashed_password,
                Utc::now(),
                Utc::now(),
            )
            .execute(&db_share_pool)
            .await?;
            info!("row affected {}", row.rows_affected());
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&json!({
                "affected": row.rows_affected(),
                "message": "添加成功",
            }))?);
            Ok(res)
        });
    // 登陆接口
    app.at("login").post(|mut req: Request<State>| async move {
        let db_share_pool = req.state().db_pool.clone();
        let login_user = req.body_json::<entity::user::LoginUser>().await?;
        let hashed_password = query_as!(
            entity::user::Password,
            "select hashed_password from users where username=$1",
            login_user.username
        )
        .fetch_one(&db_share_pool)
        .await?;
        let mut res_json = ResultMessage{
            token: "".to_string(),
            message : "登陆失败".to_string()
        };

        if argon2::verify_encoded(
            &hashed_password.hashed_password,
            login_user.password.as_bytes(),
        )
        .unwrap()
        {
            res_json.token = "todo token".to_string();
            res_json.message = "登陆成功".to_string();
        }
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(Body::from_json(&json!(res_json))?);
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

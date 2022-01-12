use dotenv::dotenv;
use log::LevelFilter;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};
use tide_fluent_routes::prelude::*;
use tide_fluent_routes::router::Router;
use tide_sqlx::SQLxMiddleware;

mod handle;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    tide::log::start();
    let mut app = tide::new();

    let mut connect_opts = PgConnectOptions::new();
    connect_opts.log_statements(LevelFilter::Debug);

    // pg pool
    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(connect_opts)
        .await?;

    // 使用中间件
    app.with(SQLxMiddleware::from(pg_pool));
    // 路由插件
    app.register(
        root()
            .at("api/users", |route| route.get(handle::users))
            .at("api/v1", |route| route.get(handle::get_hello))
            .at("api/v2", |route| route.get(handle::get_world)),
    );

    app.listen("127.0.0.1:8888").await?;
    Ok(())
}

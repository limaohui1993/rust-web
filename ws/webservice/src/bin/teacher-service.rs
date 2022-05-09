use actix_web::{web,App,HttpServer};
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

#[path="../dbaccess/mod.rs"]
mod dbaccess;

#[path="../errors.rs"]
mod errors;
#[path="../handlers/mod.rs"]
mod handlers;

#[path="../models/mod.rs"]
mod models;

#[path="../routers.rs"]
mod routers;

#[path="../state.rs"]
mod state;


use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() ->io::Result<()> {
    dotenv().ok();

    let database_url=env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool=PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data=web::Data::new(AppState{
        health_check_response:"I'm OK.".to_string(),
        visit_count:Mutex::new(0),
        // courses:Mutex::new(vec![]),
        db:db_pool
    });
    let app=move||{
        App::new()
        .app_data(shared_data.clone())
        .configure(general_routes)
        .configure(course_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

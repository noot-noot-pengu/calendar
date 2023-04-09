use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{dev::Service as _, middleware::Logger, web, App, HttpServer};
use futures_util::future::FutureExt;
use mongodb::Client;
use std::time::Duration;

pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod utils;
use crate::handlers::auth;
use crate::handlers::events;
use crate::middlewares::authorization::CheckLogin;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();

    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    let store = MemoryStore::new();
    HttpServer::new(move || {
        App::new()
            //.wrap(
            //    RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
            //        .with_interval(Duration::from_secs(60))
            //        .with_max_requests(100),
            //)
            .wrap(Logger::default())
            .wrap(CheckLogin)
            .app_data(web::Data::new(client.clone()))
            .service(auth::login)
            .service(auth::register)
            .service(auth::logout)
            .service(auth::verify)
            .service(events::create)
            .service(events::delete)
            .service(events::edit)
        //.route("/hey", web::get().to(handlers::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

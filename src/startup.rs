use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{healthcheck, subscribe};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    
    let server = HttpServer::new(move || {
        App::new()
            .route("/healthcheck", web::get().to(healthcheck))
            .route("/subscriptions", web::post().to(subscribe))
            // get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
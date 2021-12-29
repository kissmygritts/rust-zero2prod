use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;
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
            // add middleware with the wrap method on App
            .wrap(TracingLogger::default())
            .route("/healthcheck", web::get().to(healthcheck))
            .route("/subscriptions", web::post().to(subscribe))
            // attach the database ppol to the server with .app_data
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
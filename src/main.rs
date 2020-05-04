#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{middleware, App, HttpServer, };
use dotenv::dotenv;
use std::env;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;


type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

//mod db;
mod users;
mod schema;
mod error_handler;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let port = 8998;
    println!("starting http server at {:?}", port);
    HttpServer::new( move|| {
        App::new()
            .data(pool.clone())
            .wrap( middleware::Logger::default())
            .configure(users::init_routes)
    })
        .bind(("127.0.0.1",port))?
        .run()
        .await

/*    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server= HttpServer::new(|| App::new().configure(users::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await*/
}

// Copyright 2022 Eray Erdin.
// Use of this source code is governed by the WTFPL
// license that can be found in the LICENSE file.

use std::path;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer, Responder};
use reactix::{
    config::{init_command, init_config},
    logging::init_logger,
};
use sqlx::{postgres::PgPoolOptions, Connection, PgConnection};

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    log::debug!("Initializing server...");

    let cli_matches = init_command().get_matches();
    let dotenv_path = cli_matches
        .get_one::<path::PathBuf>("env-file")
        .expect("Could not get `env-file` flag value.");

    let config = init_config(dotenv_path)?;
    let server_config = config.clone(); // to move in HttpServer closure

    init_logger(config.log_target.clone(), config.log_level, config.log_file)?;
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(config.throttle_per_second)
        .burst_size(config.throttle_burst_size)
        .methods(config.throttle_methods)
        .finish()
        .expect("Could not build governor throttle configuration.");

    let db_pool = PgPoolOptions::new()
        .min_connections(config.db_pool_min_connections)
        .max_connections(config.db_pool_max_connections)
        .connect_lazy(&config.db_url)?;

    let mut migration_connection = PgConnection::connect(&config.db_url).await?;
    sqlx::migrate!().run(&mut migration_connection).await?;

    log::info!("Running server on: http://{}:{}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_config.clone()))
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(Governor::new(&governor_conf))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::from(server_config.session_secret_key.as_bytes()),
            ))
            // TODO add your other routes here instead of below static files
            // otherwise, they overshadow the url for static files, thus react
            .service(actix_files::Files::new("/", "frontend/build").index_file("index.html"))
    })
    .bind((config.host, config.port))?
    .run()
    .await
    .map_err(|err| anyhow::anyhow!("An error occured while running the server. {:?}", err))
}

// Copyright 2022 Eray Erdin.
// Use of this source code is governed by the WTFPL
// license that can be found in the LICENSE file.

use std::path;

use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer, Responder};
use reactix::{
    config::{init_command, init_config},
    logging::init_logger,
};

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

    log::info!("Running server on: http://{}:{}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_config.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                Key::from(server_config.session_secret_key.as_bytes()),
            ))
            .service(hello)
    })
    .bind((config.host, config.port))?
    .run()
    .await
    .map_err(|err| anyhow::anyhow!("An error occured while running the server. {:?}", err))
}

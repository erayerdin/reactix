// Copyright 2022 Eray Erdin.
// Use of this source code is governed by the WTFPL
// license that can be found in the LICENSE file.

use std::{io, path};

use actix_web::{web, App, HttpServer, Responder};
use reactix::config::{init_command, init_config};

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    log::debug!("Initializing server...");

    let cli_matches = init_command().get_matches();
    let dotenv_path = cli_matches
        .get_one::<path::PathBuf>("env-file")
        .expect("Could not get `env-file` flag value.");

    let config = init_config(dotenv_path).expect("Could not initialize Config.");
    let server_config = config.clone(); // to move in HttpServer closure

    HttpServer::new(move || {
        let config = server_config.clone(); // fails if not clone, because server_config drops at the end of closure
        App::new().app_data(web::Data::new(config)).service(hello)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}

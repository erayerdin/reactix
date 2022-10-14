// Copyright 2022 Eray Erdin.
// Use of this source code is governed by the WTFPL
// license that can be found in the LICENSE file.

use std::io;

use actix_web::{App, HttpServer, Responder};

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

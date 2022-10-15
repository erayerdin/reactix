// Copyright 2022 Eray Erdin.
// Use of this source code is governed by the WTFPL
// license that can be found in the LICENSE file.

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate actix_multipart_extract;

pub mod config;
pub mod logging;

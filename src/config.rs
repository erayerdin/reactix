// Copyright 2022 Eray Erdin.
// Use of this source code is governed by the WTFPL
// license that can be found in the LICENSE file.

use std::path;

// TODO change env var prefix
const ENV_PREFIX: &str = "WEBAPP_";

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_host")]
    pub host: String,
    #[serde(default = "Config::default_port")]
    pub port: u16,
}

impl Config {
    fn default_host() -> String {
        "0.0.0.0".to_string()
    }

    fn default_port() -> u16 {
        8080
    }
}

pub fn init_config(dotenv_path: &path::PathBuf) -> envy::Result<Config> {
    log::debug!("Initializing Config...");

    dotenvy::from_path(dotenv_path).expect("Could not load dotenv file.");
    log::trace!("dotenv path: {}", dotenv_path.to_string_lossy().to_string());

    // TODO change prefix
    envy::prefixed(ENV_PREFIX).from_env::<Config>()
}

pub fn init_command() -> clap::Command {
    log::debug!("Initializing command...");

    clap::command!().arg(
        clap::Arg::new("env-file")
            .short('e')
            .long("env-file")
            .required(false)
            .default_value(".env")
            .value_parser(clap::value_parser!(path::PathBuf)),
    )
}

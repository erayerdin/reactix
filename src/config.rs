// Copyright 2022 Eray Erdin.
// Use of this source code is governed by the WTFPL
// license that can be found in the LICENSE file.

use std::path;

use crate::logging;

// TODO change env var prefix
const ENV_PREFIX: &str = "WEBAPP_";

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_host")]
    pub host: String,
    #[serde(default = "Config::default_port")]
    pub port: u16,
    #[serde(
        default = "Config::default_log_level",
        deserialize_with = "Config::string_to_level_filter"
    )]
    pub log_level: log::LevelFilter,
    #[serde(
        default = "Config::default_log_target",
        deserialize_with = "Config::string_to_log_target"
    )]
    pub log_target: logging::LogTarget,
    #[serde(default = "Config::default_log_file")]
    pub log_file: Option<path::PathBuf>,
}

impl Config {
    fn default_host() -> String {
        "0.0.0.0".to_string()
    }

    fn default_port() -> u16 {
        8080
    }

    fn default_log_level() -> log::LevelFilter {
        log::LevelFilter::Info
    }

    fn default_log_target() -> logging::LogTarget {
        logging::LogTarget::Crate
    }

    fn default_log_file() -> Option<path::PathBuf> {
        None
    }

    fn string_to_level_filter<'de, D>(deserializer: D) -> Result<log::LevelFilter, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val: String = serde::Deserialize::deserialize(deserializer)?;

        match val.to_lowercase().as_str() {
            "trace" => Ok(log::LevelFilter::Trace),
            "debug" => Ok(log::LevelFilter::Debug),
            "warn" => Ok(log::LevelFilter::Warn),
            "info" => Ok(log::LevelFilter::Info),
            "error" => Ok(log::LevelFilter::Error),
            "off" => Ok(log::LevelFilter::Off),
            _ => Err(serde::de::Error::custom(
                "Unknown log level. Levels are: off, trace, debug, warn, info, error",
            )),
        }
    }

    fn string_to_log_target<'de, D>(deserializer: D) -> Result<logging::LogTarget, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val: String = serde::Deserialize::deserialize(deserializer)?;

        match val.to_lowercase().as_str() {
            "crate" => Ok(logging::LogTarget::Crate),
            "all" => Ok(logging::LogTarget::All),
            _ => Err(serde::de::Error::custom(
                "Unknown log target. Targets are: crate, all",
            )),
        }
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

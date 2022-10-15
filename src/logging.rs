// Copyright 2022 Eray Erdin.
// Use of this source code is governed by the WTFPL
// license that can be found in the LICENSE file.

use std::{io, path};

#[derive(Debug, Error)]
pub enum LoggingError {
    #[error("An error occured while initializing logger. {0}")]
    LogError(log::SetLoggerError),
    #[error("An error occured while setting log file. {0}")]
    IoError(io::Error),
}

#[derive(Debug, Clone)]
pub enum LogTarget {
    All,
    Crate,
}

// TODO edit this
pub fn init_logger(
    log_target: LogTarget,
    log_level: log::LevelFilter,
    log_file: Option<path::PathBuf>,
) -> Result<(), LoggingError> {
    let mut log_dispatch = fern::Dispatch::new()
        .filter(move |metadata| match log_target {
            LogTarget::All => true,
            LogTarget::Crate => metadata.target().starts_with(env!("CARGO_PKG_NAME")),
        })
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                record.level(),
                record.target(),
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                message
            ))
        })
        .level(log_level.clone())
        .chain(std::io::stdout());

    if let Some(log_file) = log_file {
        log_dispatch =
            log_dispatch.chain(fern::log_file(log_file).map_err(|err| LoggingError::IoError(err))?);
    }

    log_dispatch
        .apply()
        .map_err(|err| LoggingError::LogError(err))
}

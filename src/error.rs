// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeployError {
    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("I/O error reading {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("parse error in {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("missing required field: {0}")]
    MissingField(&'static str),

    #[error("invalid value for {field}: {detail}")]
    InvalidValue {
        field: &'static str,
        detail: String,
    },

    #[error("version {0} is not supported")]
    UnsupportedVersion(u32),

    #[error("{0}")]
    Custom(String),
}

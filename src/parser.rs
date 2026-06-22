// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::Path;

use crate::error::DeployError;
use crate::model::DeployManifest;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Yaml,
    Json,
    Toml,
}

impl Format {
    pub fn from_path(path: &Path) -> Option<Format> {
        match path.extension()?.to_str()? {
            "yaml" | "yml" => Some(Format::Yaml),
            "json" => Some(Format::Json),
            "toml" => Some(Format::Toml),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Format::Yaml => "yaml",
            Format::Json => "json",
            Format::Toml => "toml",
        }
    }
}

fn parse_bytes(content: &[u8], format: Format) -> Result<DeployManifest, DeployError> {
    match format {
        Format::Yaml => {
            #[cfg(not(feature = "yaml"))]
            return Err(DeployError::UnsupportedFormat("yaml (feature not enabled)".into()));
            #[cfg(feature = "yaml")]
            crate::yaml::from_slice(content).map_err(|e| DeployError::Parse {
                path: "<yaml>".into(),
                source: e.into(),
            })
        }
        Format::Json => {
            crate::json::from_slice(content).map_err(|e| DeployError::Parse {
                path: "<json>".into(),
                source: e.into(),
            })
        }
        Format::Toml => {
            #[cfg(not(feature = "toml"))]
            return Err(DeployError::UnsupportedFormat("toml (feature not enabled)".into()));
            #[cfg(feature = "toml")]
            crate::toml::from_slice(content).map_err(|e| DeployError::Parse {
                path: "<toml>".into(),
                source: e.into(),
            })
        }
    }
}

pub fn from_file(path: &Path) -> Result<DeployManifest, DeployError> {
    let format = Format::from_path(path).ok_or_else(|| {
        DeployError::UnsupportedFormat(
            path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown")
                .to_string(),
        )
    })?;

    let content = std::fs::read(path).map_err(|source| DeployError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    parse_bytes(&content, format).map_err(|e| match e {
        DeployError::Parse { source, .. } => DeployError::Parse {
            path: path.to_path_buf(),
            source,
        },
        other => other,
    })
}

pub fn from_reader(reader: impl std::io::Read, format: Format) -> Result<DeployManifest, DeployError> {
    let mut buf = Vec::new();
    let mut reader = reader;
    std::io::Read::read_to_end(&mut reader, &mut buf).map_err(|source| DeployError::Io {
        path: "<reader>".into(),
        source,
    })?;

    parse_bytes(&buf, format)
}

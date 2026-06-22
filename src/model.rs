// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployManifest {
    pub version: u32,
    pub app: App,
    pub runtime: Runtime,
    pub build: Option<Build>,
    pub start: Start,
    pub workers: Option<IndexMap<String, Worker>>,
    pub env: Option<IndexMap<String, EnvValue>>,
    pub network: Option<Network>,
    pub storage: Option<Vec<Storage>>,
    pub services: Option<IndexMap<String, Service>>,
    pub health: Option<Health>,
    pub scaling: Option<Scaling>,
    pub cron: Option<Vec<CronJob>>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub labels: Option<IndexMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Runtime {
    #[serde(rename = "type")]
    pub runtime_type: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub strategy: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<IndexMap<String, EnvValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Start {
    Simple(StartCommand),
    Multi(IndexMap<String, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartCommand {
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    pub command: String,
    pub replicas: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnvValue {
    Plain(String),
    Secret { secret: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub port: Option<u16>,
    pub domains: Option<Vec<String>>,
    pub routes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub name: String,
    pub mount: String,
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub ephemeral: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "type")]
    pub service_type: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Health {
    Path {
        path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timeout: Option<String>,
    },
    Command {
        command: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        timeout: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scaling {
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub cpu_target: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronJob {
    pub name: String,
    pub schedule: String,
    pub command: String,
}

impl DeployManifest {
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, crate::error::DeployError> {
        crate::parser::from_file(path.as_ref())
    }

    #[cfg(feature = "yaml")]
    pub fn from_yaml(path: impl AsRef<std::path::Path>) -> Result<Self, crate::error::DeployError> {
        Self::from_file(path)
    }

    #[cfg(feature = "json")]
    pub fn from_json(path: impl AsRef<std::path::Path>) -> Result<Self, crate::error::DeployError> {
        Self::from_file(path)
    }

    #[cfg(feature = "toml")]
    pub fn from_toml(path: impl AsRef<std::path::Path>) -> Result<Self, crate::error::DeployError> {
        Self::from_file(path)
    }

    #[cfg(feature = "yaml")]
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        crate::yaml::to_string(self)
    }

    #[cfg(feature = "json")]
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        crate::json::to_string(self)
    }

    #[cfg(feature = "toml")]
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        crate::toml::to_string(self)
    }

    pub fn from_reader(
        reader: impl std::io::Read,
        format: crate::parser::Format,
    ) -> Result<Self, crate::error::DeployError> {
        crate::parser::from_reader(reader, format)
    }

    pub fn validate(&self) -> Result<(), crate::error::ValidationError> {
        crate::validation::Validate::validate(self)
    }
}

fn is_false(b: &bool) -> bool {
    !b
}

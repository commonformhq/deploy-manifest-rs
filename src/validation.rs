// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::error::ValidationError;
use crate::model::*;

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

impl Validate for DeployManifest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.version != 1 {
            return Err(ValidationError::UnsupportedVersion(self.version));
        }

        self.app.validate()?;
        self.runtime.validate()?;
        self.start.validate()?;

        if let Some(ref build) = self.build {
            build.validate()?;
        }

        if let Some(ref workers) = self.workers {
            for (name, worker) in workers {
                worker.validate()?;
                if name.is_empty() {
                    return Err(ValidationError::InvalidValue {
                        field: "workers",
                        detail: "worker name must not be empty".into(),
                    });
                }
            }
        }

        if let Some(ref network) = self.network {
            network.validate()?;
        }

        if let Some(ref scaling) = self.scaling {
            scaling.validate()?;
        }

        if let Some(ref health) = self.health {
            health.validate()?;
        }

        if let Some(ref storage) = self.storage {
            for s in storage {
                if s.name.is_empty() {
                    return Err(ValidationError::InvalidValue {
                        field: "storage.name",
                        detail: "storage name must not be empty".into(),
                    });
                }
            }
        }

        Ok(())
    }
}

impl Validate for App {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.name.is_empty() {
            return Err(ValidationError::MissingField("app.name"));
        }
        if self.name.len() > 64 {
            return Err(ValidationError::InvalidValue {
                field: "app.name",
                detail: format!("maximum 64 characters, got {}", self.name.len()),
            });
        }
        if !self
            .name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(ValidationError::InvalidValue {
                field: "app.name",
                detail: "must be DNS-safe (lowercase alphanumeric and hyphens only)".into(),
            });
        }
        Ok(())
    }
}

const SUPPORTED_RUNTIMES: &[&str] = &[
    "php", "node", "python", "rust", "go", "java", "bun", "deno", "static", "custom",
];

impl Validate for Runtime {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.runtime_type.is_empty() {
            return Err(ValidationError::MissingField("runtime.type"));
        }
        if !SUPPORTED_RUNTIMES.contains(&self.runtime_type.as_str()) {
            return Err(ValidationError::InvalidValue {
                field: "runtime.type",
                detail: format!(
                    "unsupported runtime '{}', expected one of: {}",
                    self.runtime_type,
                    SUPPORTED_RUNTIMES.join(", ")
                ),
            });
        }
        Ok(())
    }
}

impl Validate for Build {
    fn validate(&self) -> Result<(), ValidationError> {
        let supported = ["auto", "custom", "dockerfile", "buildpack", "nixpacks", "none"];
        if !supported.contains(&self.strategy.as_str()) {
            return Err(ValidationError::InvalidValue {
                field: "build.strategy",
                detail: format!(
                    "unsupported strategy '{}', expected one of: {}",
                    self.strategy,
                    supported.join(", ")
                ),
            });
        }
        if self.strategy == "custom" && self.commands.is_none() {
            return Err(ValidationError::MissingField("build.commands"));
        }
        if let Some(ref vars) = self.variables {
            for name in vars.keys() {
                if name.is_empty() {
                    return Err(ValidationError::InvalidValue {
                        field: "build.variables",
                        detail: "variable name must not be empty".into(),
                    });
                }
            }
        }
        Ok(())
    }
}

impl Validate for Start {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Start::Simple(cmd) => {
                if cmd.command.is_empty() {
                    return Err(ValidationError::MissingField("start.command"));
                }
            }
            Start::Multi(processes) => {
                if processes.is_empty() {
                    return Err(ValidationError::MissingField("start"));
                }
            }
        }
        Ok(())
    }
}

impl Validate for Worker {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.command.is_empty() {
            return Err(ValidationError::MissingField("workers.command"));
        }
        if self.replicas < 1 {
            return Err(ValidationError::InvalidValue {
                field: "workers.replicas",
                detail: "must be >= 1".into(),
            });
        }
        Ok(())
    }
}

impl Validate for Network {
    fn validate(&self) -> Result<(), ValidationError> {
        if let Some(port) = self.port
            && port == 0
        {
            return Err(ValidationError::InvalidValue {
                field: "network.port",
                detail: "must be between 1 and 65535".into(),
            });
        }
        Ok(())
    }
}

impl Validate for Scaling {
    fn validate(&self) -> Result<(), ValidationError> {
        if let (Some(min), Some(max)) = (self.min, self.max)
            && min > max
        {
            return Err(ValidationError::InvalidValue {
                field: "scaling",
                detail: format!("min ({min}) must be <= max ({max})"),
            });
        }
        if let Some(cpu) = self.cpu_target
            && (cpu == 0 || cpu > 100)
        {
            return Err(ValidationError::InvalidValue {
                field: "scaling.cpu_target",
                detail: "must be between 1 and 100".into(),
            });
        }
        Ok(())
    }
}

impl Validate for Health {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Health::Path { path, .. } => {
                if path.is_empty() {
                    return Err(ValidationError::MissingField("health.path"));
                }
            }
            Health::Command { command, .. } => {
                if command.is_empty() {
                    return Err(ValidationError::MissingField("health.command"));
                }
            }
        }
        Ok(())
    }
}

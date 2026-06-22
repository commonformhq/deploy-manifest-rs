// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io::IsTerminal;
use std::path::Path;
use std::process;

use deploy_manifest::parser::Format;
use deploy_manifest::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let result = match args.len() {
        1 if std::io::stdin().is_terminal() => {
            eprintln!("Usage: deploy-manifest <file> [--format yaml|json|toml]");
            eprintln!("       deploy-manifest --stdin [--format yaml|json|toml]");
            process::exit(1);
        }
        1 => parse_stdin(None),
        _ => {
            let mut path = None;
            let mut explicit_format = None;
            let mut stdin_mode = false;

            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--format" | "-f" => {
                        i += 1;
                        if i >= args.len() {
                            eprintln!("error: --format requires an argument");
                            process::exit(1);
                        }
                        explicit_format = Some(match args[i].as_str() {
                            "yaml" | "yml" => Format::Yaml,
                            "json" => Format::Json,
                            "toml" => Format::Toml,
                            other => {
                                eprintln!("error: unsupported format '{other}'");
                                process::exit(1);
                            }
                        });
                    }
                    "--stdin" => stdin_mode = true,
                    p => path = Some(p.to_string()),
                }
                i += 1;
            }

            match path {
                Some(p) if !stdin_mode => parse_file(Path::new(&p), explicit_format),
                _ => parse_stdin(explicit_format),
            }
        }
    };

    match result {
        Ok(manifest) => {
            println!("✓ manifest valid");
            println!("  app:  {}", manifest.app.name);
            println!("  runtime: {}", manifest.runtime.runtime_type);
            if let Some(ref v) = manifest.runtime.version {
                println!("  version: {v}");
            }
            if let Some(ref build) = manifest.build {
                println!(
                    "  build: {} ({})",
                    build.strategy,
                    build
                        .commands
                        .as_ref()
                        .map(|c| c.len().to_string())
                        .unwrap_or_default()
                        + " commands"
                );
            }
            println!(
                "  start processes: {}",
                match &manifest.start {
                    Start::Simple(_) => 1,
                    Start::Multi(m) => m.len(),
                }
            );
        }
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    }
}

fn parse_file(path: &Path, explicit: Option<Format>) -> Result<DeployManifest, DeployError> {
    let manifest = if let Some(fmt) = explicit {
        let content = std::fs::read(path).map_err(|e| DeployError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
        parser::from_reader(&content[..], fmt).map_err(|e| match e {
            DeployError::Parse { source, .. } => DeployError::Parse {
                path: path.to_path_buf(),
                source,
            },
            other => other,
        })?
    } else {
        DeployManifest::from_file(path)?
    };
    manifest.validate()?;
    Ok(manifest)
}

fn parse_stdin(explicit: Option<Format>) -> Result<DeployManifest, DeployError> {
    let format = explicit.unwrap_or(Format::Yaml);
    let manifest = DeployManifest::from_reader(std::io::stdin().lock(), format)?;
    manifest.validate()?;
    Ok(manifest)
}

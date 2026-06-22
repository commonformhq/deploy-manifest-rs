// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use deploy_manifest::*;

#[test]
fn parse_and_validate_yaml() {
    let manifest = DeployManifest::from_file("examples/deploy.yaml").unwrap();
    manifest.validate().unwrap();
    assert_eq!(manifest.version, 1);
    assert_eq!(manifest.app.name, "store");
    assert_eq!(manifest.runtime.runtime_type, "php");
}

#[test]
fn parse_and_validate_toml() {
    let manifest = DeployManifest::from_file("examples/deploy.toml").unwrap();
    manifest.validate().unwrap();
    assert_eq!(manifest.version, 1);
    assert_eq!(manifest.app.name, "store");
}

#[test]
fn roundtrip_yaml() {
    let manifest = DeployManifest::from_file("examples/deploy.yaml").unwrap();
    let yaml_out = manifest.to_yaml().unwrap();
    let reparsed: DeployManifest = serde_yaml::from_str(&yaml_out).unwrap();
    assert_eq!(manifest.app.name, reparsed.app.name);
}

#[test]
fn validation_rejects_empty_name() {
    let manifest = DeployManifest {
        version: 1,
        app: App {
            name: "".into(),
            description: None,
            homepage: None,
            repository: None,
            labels: None,
        },
        runtime: Runtime {
            runtime_type: "rust".into(),
            version: None,
        },
        build: None,
        start: Start::Simple(StartCommand {
            command: "./server".into(),
        }),
        workers: None,
        env: None,
        network: None,
        storage: None,
        services: None,
        health: None,
        scaling: None,
        cron: None,
        extensions: Default::default(),
    };
    assert!(manifest.validate().is_err());
}

#[test]
fn validation_rejects_unsupported_version() {
    let manifest = DeployManifest {
        version: 999,
        app: App {
            name: "test".into(),
            description: None,
            homepage: None,
            repository: None,
            labels: None,
        },
        runtime: Runtime {
            runtime_type: "rust".into(),
            version: None,
        },
        build: None,
        start: Start::Simple(StartCommand {
            command: "./server".into(),
        }),
        workers: None,
        env: None,
        network: None,
        storage: None,
        services: None,
        health: None,
        scaling: None,
        cron: None,
        extensions: Default::default(),
    };
    assert!(manifest.validate().is_err());
}

#[test]
fn simple_start_command() {
    let yaml = "version: 1\napp:\n  name: test\nruntime:\n  type: rust\nstart:\n  command: ./server\n";
    let manifest: DeployManifest = serde_yaml::from_str(yaml).unwrap();
    assert!(matches!(manifest.start, Start::Simple(_)));
    manifest.validate().unwrap();
}

#[test]
fn parse_and_validate_json() {
    let manifest = DeployManifest::from_file("examples/deploy.json").unwrap();
    manifest.validate().unwrap();
    assert_eq!(manifest.version, 1);
    assert_eq!(manifest.app.name, "store");
}

#[test]
fn from_reader_yaml() {
    let data = std::fs::read("examples/deploy.yaml").unwrap();
    let manifest = DeployManifest::from_reader(&data[..], deploy_manifest::parser::Format::Yaml).unwrap();
    manifest.validate().unwrap();
    assert_eq!(manifest.app.name, "store");
}

#[test]
fn from_reader_json() {
    let data = std::fs::read("examples/deploy.json").unwrap();
    let manifest = DeployManifest::from_reader(&data[..], deploy_manifest::parser::Format::Json).unwrap();
    manifest.validate().unwrap();
    assert_eq!(manifest.app.name, "store");
}

#[test]
fn build_variables_parsed() {
    let manifest = DeployManifest::from_file("examples/deploy.yaml").unwrap();
    let build = manifest.build.as_ref().unwrap();
    let vars = build.variables.as_ref().unwrap();
    assert_eq!(vars["NEXT_PUBLIC_API_URL"], EnvValue::Plain("https://api.example.com".into()));
    assert_eq!(
        vars["BUILD_SECRET"],
        EnvValue::Secret { secret: "build-env-key".into() }
    );
}

#[test]
fn extensions_preserved() {
    let yaml = r#"
version: 1
app:
  name: ext-test
runtime:
  type: rust
start:
  command: ./app
x-vendor:
  region: us-east
  feature_flag: true
"#;
    let manifest: DeployManifest = serde_yaml::from_str(yaml).unwrap();
    manifest.validate().unwrap();
    assert!(manifest.extensions.contains_key("x-vendor"));
}

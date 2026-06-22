# deploy-manifest.rs

Reference Rust implementation of the Deploy Manifest Specification.

```rust
use deploy_manifest::DeployManifest;

let manifest = DeployManifest::from_file("deploy.yaml")?;
manifest.validate()?;
println!("{} v{}", manifest.app.name, manifest.version);
```

## Install

```toml
[dependencies]
deploy-manifest = "0.1"
```

Default features enable YAML and TOML. JSON support is always available.

## Usage

Parse a manifest:

```rust
// Auto-detect format from file extension
let m = DeployManifest::from_file("deploy.yaml")?;
let m = DeployManifest::from_file("deploy.json")?;
let m = DeployManifest::from_file("deploy.toml")?;

// Explicit format via reader
let m = DeployManifest::from_reader(reader, deploy_manifest::parser::Format::Yaml)?;

// Format-specific constructors
let m = DeployManifest::from_yaml("deploy.yaml")?;
let m = DeployManifest::from_json("deploy.json")?;
let m = DeployManifest::from_toml("deploy.toml")?;
```

Serialize back:

```rust
let yaml = manifest.to_yaml()?;
let json = manifest.to_json()?;
let toml = manifest.to_toml()?;
```

Validate against the specification:

```rust
manifest.validate()?; // returns Err(ValidationError) on any violation
```

## CLI

The crate ships a binary that parses and validates manifests:

```text
deploy-manifest deploy.yaml
deploy-manifest deploy.json --format json
cat deploy.yaml | deploy-manifest --stdin
```

Exits 0 on success, prints errors to stderr and exits 1 on failure.

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `yaml` | YAML support via `serde_yaml` | yes |
| `toml` | TOML support via `toml` | yes |

JSON is always available — `serde_json` is a required dependency.

## Specification

The full DMS spec lives in the [deploy-manifest](https://github.com/hisoka-root/deploy-manifest) repo.

## License

MPL 2.0 ([LICENSE](LICENSE))

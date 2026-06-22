// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::model::DeployManifest;

pub fn from_slice(slice: &[u8]) -> Result<DeployManifest, toml::de::Error> {
    toml::from_str(&String::from_utf8_lossy(slice))
}

pub fn to_string(manifest: &DeployManifest) -> Result<String, toml::ser::Error> {
    toml::to_string_pretty(manifest)
}

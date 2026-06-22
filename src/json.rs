// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::model::DeployManifest;

pub fn from_slice(slice: &[u8]) -> Result<DeployManifest, serde_json::Error> {
    serde_json::from_slice(slice)
}

pub fn to_string(manifest: &DeployManifest) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(manifest)
}

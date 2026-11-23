//! Location of the real CalculiX files used by the tests that are skipped when a file is absent.

use std::path::{Path, PathBuf};

/// `$CALCULIX_DATA/rel`, or `rel` under the directory that contains this workspace when the variable is unset.
#[allow(dead_code)]
pub fn data(rel: &str) -> PathBuf {
    std::env::var_os("CALCULIX_DATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.."))
        .join(rel)
}

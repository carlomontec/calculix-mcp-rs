// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Carlo Daniel Monjaraz Tec

//! FRD parser module supporting both ASCII and Binary (single/double precision) CalculiX result formats.

pub mod frd;
pub mod frd_index;
pub mod inp;
pub mod inp_cards;
pub mod inp_edit;
pub mod inp_mesh;
pub mod reader;

pub use frd::FrdParserEngine;
pub use frd_index::{DatasetEntry, DenseDataset, FrdIndex, StepRef};
pub use inp::{
    consolidate_inp, inspect_inp, load_companion_sets, parse_sets_from_inp, parse_sets_from_str, InpConsolidateStats,
    InpInclude, InpSummary,
};
pub use inp_cards::*;
pub use inp_edit::*;
pub use inp_mesh::*;
pub use reader::ByteCursor;

use crate::{FrdModel, Result};
use std::path::Path;

/// Convenience function to parse any CalculiX FRD model (ASCII, Binary Float, or Binary Double)
pub fn parse_frd_file<P: AsRef<Path>>(path: P) -> Result<FrdModel> {
    let path = path.as_ref();
    let file = std::fs::File::open(path)?;
    let mmap = unsafe { memmap2::Mmap::map(&file)? };

    let mut engine = FrdParserEngine::from_bytes(&mmap);
    let mut model = engine.parse_all()?;
    model.load_companion_sets(path);
    Ok(model)
}

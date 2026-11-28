# Changelog

All notable changes to this project are documented here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

First public version. It is not yet tagged.

### Added

- `calculix-mcp`, a Model Context Protocol server with 18 tools:
  - Decks: `inp_inspect`, `inp_cards`, `inp_consolidate`, `inp_edit`, which follow the whole `*INCLUDE` tree and write edits to new files with a diff.
  - Geometry and meshing through CalculiX GraphiX: `cgx_run_batch`, `cgx_help`.
  - Mesh: `mesh_check`, `mesh_select`, `mesh_contact_pairs`.
  - Solving: `ccx_solve`, `ccx_job_status`, `ccx_job_cancel`, `ccx_job_list`, `ccx_diagnose`, with stock CalculiX or CalculiX MultiSolver and its JSON telemetry.
  - Results: `frd_summary`, `frd_sets`, `frd_query_set`, with exact values and node ids from ASCII or binary `.frd` files.
  - Setup: `models_folder`.
- A safety model: the server is confined to the models folder, has no shell, clears the solver's environment, runs each solve in its own directory and cleans up the solver's whole process group.
- `cgx-core`, the library: a lazy `.frd` index, an `.inp` parser and card editor, mesh checks, face selection, contact search and `.sta`/`.cvg` convergence diagnosis.
- `cgx-top`, a terminal dashboard and command-line tool for results and decks.
- CI on Linux x86_64 and arm64, macOS and Windows, plus an end-to-end solve with CalculiX MultiSolver and with stock ccx.

[Unreleased]: https://github.com/carlomontec/calculix-mcp-rs/commits/main

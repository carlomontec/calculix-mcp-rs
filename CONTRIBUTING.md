# Contributing to calculix-mcp

Thank you for your interest. Bug reports, real-world decks that break a tool, documentation fixes and code are all welcome.

## Before you start

- **Bugs and questions:** open an [issue](https://github.com/carlomontec/calculix-mcp-rs/issues/new/choose). Include the tool call, the server's reply and, if you can, a small deck or `.frd` file that reproduces it.
- **Larger changes** (a new tool, a change to a tool's arguments or output): open an issue first so we can agree on the design. A tool's interface is what AI clients depend on, so it changes on purpose, not by accident.
- **Security problems:** do not open a public issue. See [SECURITY.md](SECURITY.md).

## Building

You need a Rust toolchain at version 1.89 or newer (see `rust-version` in `Cargo.toml`).

```bash
cargo build --workspace
cargo build --release -p calculix-mcp   # the server: target/release/calculix-mcp
cargo build --release -p cgx-top        # the terminal tool
```

The workspace has three crates:

| Crate | What it is |
|---|---|
| `crates/cgx-core` | The library: `.frd` reader, `.inp` parser and editor, mesh checks, face selection, contact search, solver telemetry |
| `crates/calculix-mcp` | The MCP server built on `cgx-core` |
| `crates/cgx-top` | The terminal dashboard and command-line tool |

## Tests

```bash
cargo test --workspace
```

This runs everything that needs only the files in this repository. Some tests also read real CalculiX models and are skipped when those files are absent:

- **Real-file tests** look for files under `$CALCULIX_DATA` (by default, the folder that contains this repository). They expect clones of [CalculiX MultiSolver](https://github.com/carlomontec/CalculiX-CrunchiX-MultiSolver) at `ccx/CalculiX-CrunchiX-MultiSolver` and [CalculiX GraphiX GLFW](https://github.com/carlomontec/CalculiX-GraphiX-GLFW) at `cgx/CalculiX-CGX-New3D` inside that folder.
- **GraphiX tests** are marked `#[ignore]` and need a `cgx` binary. Run them with `CGX_BIN=/abs/path/to/cgx cargo test -p calculix-mcp -- --ignored`.
- **Benchmarks** in `crates/cgx-core/tests/frd_index.rs` are marked `#[ignore]`. Run them with `FRD=/abs/path/to/file.frd cargo test --release -p cgx-core --test frd_index bench_ -- --ignored --nocapture`.

The end-to-end test drives the server over stdio with a real solver, as CI does on Linux:

```bash
cargo build -p calculix-mcp
python3 crates/calculix-mcp/tests/smoke/drive.py ccx=/abs/path/to/ccx
```

Add a test with every fix and every feature. Prefer small decks in `crates/*/tests/fixtures/` over files that only exist on your machine.

## Style

CI rejects a pull request that fails either of these, so run them before you push:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

A few rules specific to this project:

- **Never write to stdout in the server.** stdout carries the MCP protocol; log to stderr.
- **Return exact values.** Tools report numbers read from the files, with node or element ids, and say plainly when something is not available. They never estimate.
- **Keep paths inside the models folder.** Any new path argument goes through the existing path checks. See the safety model in the [server README](crates/calculix-mcp/README.md#safety-model).
- **Document tools.** A new or changed tool updates `docs/TOOLS.md` and the tool table in the server README.

## Commits and pull requests

- Use [Conventional Commits](https://www.conventionalcommits.org/) with a scope where it helps: `feat(mcp): ...`, `fix(core): ...`, `docs: ...`, `test(mesh): ...`, `ci: ...`.
- Keep a pull request to one topic, describe what changed and how you tested it, and add a line to `CHANGELOG.md` under *Unreleased* for anything a user would notice.

## License

This project is licensed under the [GNU Affero General Public License v3.0 or later](LICENSE). By submitting a contribution you agree that it is licensed under the same terms.

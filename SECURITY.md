# Security policy

`calculix-mcp` gives an AI client controlled access to files and to a solver process on your machine. Reports of anything that weakens that control are taken seriously.

## Supported versions

The project is young; only the latest code on `main` and the latest release receive fixes.

| Version | Supported |
|---|---|
| `main` / latest 0.1.x | Yes |
| Anything older | No |

## What counts as a vulnerability

The server's safety model is described in the [server README](crates/calculix-mcp/README.md#safety-model). Examples of what we want to hear about:

- A path that reads or writes outside the active models folder (through `..`, symlinks, `*INCLUDE` cards or anything else).
- A way to start a program other than a solver named with `--ccx`, or to pass arguments through a shell.
- Environment variables leaking into the solver beyond those documented.
- A solver process, or a process it started, that survives cancel, timeout or server shutdown.
- A crafted `.inp`, `.frd`, `.sta`, `.cvg` or `.json` file that crashes the server or makes it use unbounded memory or disk.

## Reporting

Please **do not open a public issue**. Report privately through GitHub: on the repository's **Security** tab, choose **Report a vulnerability**.

Include what you did, what happened, what you expected, and a minimal file or tool call that reproduces it. You can expect an acknowledgement within a week. Once a fix is released, the advisory is published with credit to you, unless you prefer to stay anonymous.

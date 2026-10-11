> [!IMPORTANT]
> Remove this line to confirm you've reviewed this PR before submitting.
# Zed

[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

Welcome to Zed, a high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

---

### Installation

On macOS, Linux, and Windows you can [download Zed directly](https://zed.dev/download) or install Zed via your local package manager ([macOS](https://zed.dev/docs/installation#macos)/[Linux](https://zed.dev/docs/linux#installing-via-a-package-manager)/[Windows](https://zed.dev/docs/windows#package-managers)).

Other platforms are not yet available:

- Web ([tracking discussion](https://github.com/zed-industries/zed/discussions/26195))

### Developing Zed

- [Building Zed for macOS](./docs/src/development/macos.md)
- [Building Zed for Linux](./docs/src/development/linux.md)
- [Building Zed for Windows](./docs/src/development/windows.md)

### Windows native TUI

The terminal frontend can be built as native Windows executables. Use Windows
Terminal on Windows 10 or Windows 11, and install the
[Windows build dependencies](./docs/src/development/windows.md).

From a Visual Studio developer PowerShell, in this checkout:

```powershell
.\script\zed-tui-install.ps1 -Build
& "$env:LOCALAPPDATA\zed-tui\bin\zed-tui.cmd" .
```

The installer copies the editor as `zed.exe` and the console launcher as
`bin\zed.exe` under `%LOCALAPPDATA%\zed-tui`. Add its `bin` directory to your user `PATH` to use
`zed-tui` directly. To run a development build without installing:

```powershell
cargo build -p zed -p cli
.\target\debug\cli.exe --tui .
```

Use the console launcher so PowerShell waits while the editor owns terminal
input. `Ctrl+\`, then `d`, detaches; `zed-tui attach` reconnects. `zed-tui ls`,
`zed-tui kill`, `--session NAME`, and `--wait FILE` use the same session model as
Unix. Sessions and logs are under Zed's data directory in `tui\<channel>`.
Windows uses local AF_UNIX sockets and UTF-16 path metadata; it does not need WSL.
If a long profile or custom data directory exceeds the Windows socket path
limit, use a shorter local `--user-data-dir` for all commands in that session.
Windows console input uses native key, mouse, and resize events; terminal palette
and enhanced keyboard capability queries are disabled on this platform.

To validate a native build:

```powershell
cargo test --locked -p gpui_tui
cargo test --locked -p cli tui::tests
cargo test --locked -p zed --bin zed tui
```

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Zed.

Also... we're hiring! Check out our [jobs](https://zed.dev/jobs) page for open roles.

### Licensing

Zed source code is licensed primarily under GPL-3.0-or-later, with Apache-2.0 components where marked.

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).

## Sponsorship

Zed is developed by **Zed Industries, Inc.**, a for-profit company.

If you’d like to financially support the project, you can do so via GitHub Sponsors.
Sponsorships go directly to Zed Industries and are used as general company revenue.
There are no perks or entitlements associated with sponsorship.

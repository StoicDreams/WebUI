# Rust Web UI Workspace - Stoic Dreams
## [Crates.io/crates/webui](https://crates.io/crates/webui)

## About

This project has been created to house a Rust based web UI framework to remove boilerplate when creating new SPA websites.

## Goals

- Minimize boilerplate code when starting new website projects.
- Adhere to Material UI standards.
- Provide robust component systems for handling common display types, user input and interactions, and data processing.

## Getting Started

This project was setup on a Mac, so for now instructions are only included for Mac setup.

### Download

Clone this solution into a folder.

### Install Required Dev Dependencies

Follow [instructions here to install Rust](https://rustup.rs/) for your system.

Make sure rust is up to date

```bash
rustup update
```

Install Rusts automatic formatting tool.

```bash
rustup component add rustfmt
```

Then you can run this command to apply formatting to your project.

```bash
cargo fmt
```

Install Rusts Clippy linting tool.

```bash
rustup component add clippy
```

Then you can run this command to find additional linting errors.

```bash
cargo clippy
```

Install [Trunk](https://trunkrs.dev/) executable - this will be used to run your website locally for testing in your browser.

```bash
cargo install trunk
```

Add waxm build target

```bash
rustup target add wasm-unknown-unknown
```

### Testing Changes to Web UI

Install webui executable from local development.

```bash
# From the workspace root folder
cargo install --path webui
```

Delete any starter files that need refeshing (index.html, etc) and run `webui` 

```bash
# From the webapp folder
webui
```

Run `trunk server --open` from your project root to run your site locally.

```bash
trunk server --open
```

### Increment Versions after updates

If a major or minor release update is required, then first manually apply the version update to `webui/Cargo.toml`, making sure to reset lower versions to 0.

Run this script to increment the patch version and apply the new version to any references / docs.

```powershell
# From the workspace root folder
.\IncrementVersion.ps1
```

### Publish Latest Updates to [crates.io](https://crates.io/crates/webui/)

First, commit any changes to Git.

Then, run the publish command from the `webui` folder to publish.

```bash
# From the webui folder
cargo publish
```


## Other Resources

[Rust Docs](https://www.rust-lang.org/)
[Yew Docs](https://yew.rs/)

## Author

**[Erik Gassler](https://www.erikgassler.com) - [Stoic Dreams](https://www.stoicdreams.com)** - Just a simpleton who likes making stuff with bits and bytes.

**Support** - Visit [Stoic Dreams' GitHub Sponsor page](https://github.com/sponsors/StoicDreams) if you would like to provide support to Stoic Dreams.

## License

MIT

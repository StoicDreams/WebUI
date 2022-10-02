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

Install webui executable - this will be used to build your boilerplate static files.

Run `webui` in your projects root folder (not `src`) to add static files (index.html, css, etc.).

```bash
webui
```

Run `trunk server --open` from your project root to run your site locally.

```bash
trunk server --open
```

## Other Resources

[Rust Docs](https://www.rust-lang.org/)
[Yew Docs](https://yew.rs/)

## Author

**[Erik Gassler](https://www.erikgassler.com) - [Stoic Dreams](https://www.stoicdreams.com)** - Just a simpleton who likes making stuff with bits and bytes.

**Support** - Visit [Stoic Dreams' GitHub Sponsor page](https://github.com/sponsors/StoicDreams) if you would like to provide support to Stoic Dreams.

## License

MIT

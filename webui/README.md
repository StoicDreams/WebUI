# Web UI

[View WebUI Docs - docs.rs/webui](https://docs.rs/webui)

[View on crates.io - crates.io/crates/webui](https://crates.io/crates/webui)

[View on GitHub - github.com/StoicDreams/RustWebUI](https://github.com/StoicDreams/RustWebUI)

## About

`webui` is a website framework for building webassembly SPA websites quickly and easily.

## Goals

- Minimize boilerplate code when starting new website projects.
- Adhere to Material UI standards.
- Provide robust component systems for handling common display types, user input and interactions, and data processing.

Development is just getting started, so we do not recommend using at this point for anything more than experimenting.

## Getting Started

These instructions assume you have at least some familiarity with the Rust language and ecosystem.

If you are new to rust, then get started by reading [The Rust Book](https://doc.rust-lang.org/book/) to learn about the language.

This project was setup on a Mac, so for now instructions have not been verified on Windows or Linux machines, though it is expected to have little to no differences in this scope.

### Install Required Dev Dependencies

Follow [instructions here to install Rust](https://rustup.rs/) for your system.

Make sure rust is up to date

```bash
rustup update
```

Install [Trunk](https://trunkrs.dev/)

```bash
cargo install trunk
```

Add waxm build target

```bash
rustup target add wasm-unknown-unknown
```

### Start a new Rust project

Start by creating your project using cargo.

```bash
cargo new name_of_your_app
cd name_of_your_app
```

Verify your Rust environment is setup correctly.

```bash
cargo run
```

Update your `Cargo.toml` file to include `webui` dependency.

```toml
[package]
name = "name_of_your_app"
version = "0.1.0"
edition = "2021"

[dependencies]
webui = "0.1.5"
```

Add an `index.html` file to the root of your project folder (not the `src` folder).

Make sure to copy the exact contents as below. Head and body content will be filled in by the `webui` framework code.


```html
<!DOCTYPE html>
<html lang="en">
	<head> </head>
	<body></body>
</html>
```

Start the development server

```bash
trunk serve --open
```

## Author

**[Erik Gassler](https://www.erikgassler.com) - [Stoic Dreams](https://www.stoicdreams.com)** - Just a simpleton who likes making stuff with bits and bytes.

**Support** - Visit [Stoic Dreams' GitHub Sponsor page](https://github.com/sponsors/StoicDreams) if you would like to provide support to Stoic Dreams.

## License

MIT

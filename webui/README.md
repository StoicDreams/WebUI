# Web UI

[View Rust WebUI Docs - docs.rs/webui](https://docs.rs/webui)

[View on crates.io - crates.io/crates/webui](https://crates.io/crates/webui)

[View on GitHub - github.com/StoicDreams/RustWebUI](https://github.com/StoicDreams/RustWebUI)

[View Demo and Doc Site - webui.stoicdreams.com](https://webui.stoicdreams.com)

[Stoic Dreams Discord](https://discord.gg/Z6WFptDyQn)

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

Install [Trunk](https://trunkrs.dev/) executable - this will be used to run your website locally for testing in your browser.

```bash
cargo install trunk wasm-bindgen-cli
```

Add waxm build target

```bash
rustup target add wasm32-unknown-unknown
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
webui = "0.3.21"
# Use the direct GitHub reference if you want bleeding edge updates
# webui = { git = "https://github.com/StoicDreams/RustWebUI", branch = "main" }
```

Install webui executable - this will be used to build your boilerplate static files.

```bash
cargo install webui
```

Run `webui` in your projects root folder (not `src`) to add static files (index.html, css, etc.).

```bash
webui
```

### Web UI Files

Note that you should run `webui` command anytime you update to a new version, making sure to update both the webui executable and the webui dependency in your project.

Certain files are considered static and are not meant to be manually updated. These files will be overwritten during updates.

Other files are considered starter files and will not overwrite an existing file during updates. If you want to update to the latest starter file, then you will need to delete or rename your existing file - recommend renaming file, such as postfixing .bck to the file name, so you can copy over your customizations to the new Web UI file once it's created.

| File | Starter | Static | Details |
| --- | :---:| :---: | --- |
| index.html | X | | Update metadata information and add any links for extended js/css functionality. |
| webui.css | | X | Base styles, do not edit. |
| app.webmanifest | X | | Update with information about your app. This is used for installable SPAs. |
| robots.txt | X | | Update as needed to manage search bot rules. |
| Logo.svg | X | | Placeholder logo. Update with your own.
| service-worker.js | X | | Basic service worker for file caching, also required for installable SPA apps. |

### Run Dev Server for Testing

Run `trunk server --open` from your project root to run your site locally. The `--open` flag will open your website in your default browser. If you already have the page open you can exclude the `--open` flag.

```bash
trunk server --open
```

## Author

**[Erik Gassler](https://www.erikgassler.com) - [Stoic Dreams](https://www.stoicdreams.com)** - Just a simpleton who likes making stuff with bits and bytes.

**Support** - Visit [Stoic Dreams' GitHub Sponsor page](https://github.com/sponsors/StoicDreams) if you would like to provide support to Stoic Dreams.

## License

MIT

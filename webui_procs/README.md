# Web UI

[WebUI Version: 0.7.21](https://github.com/StoicDreams/RustWebUI)

[View Rust WebUI Docs - docs.rs/webui](https://docs.rs/webui)

[View on crates.io - crates.io/crates/webui](https://crates.io/crates/webui)

[View on GitHub - github.com/StoicDreams/RustWebUI](https://github.com/StoicDreams/RustWebUI)

[View Demo and Doc Site - webui.stoicdreams.com](https://webui.stoicdreams.com)

[Stoic Dreams Discord](https://discord.gg/Z6WFptDyQn)

## About

`webui_procs` is a collection of macros used by `webui`.

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

### Features

Before you get started, it's important to understand the optional features that `WebUI` offers.

These features can be enable using feature flags. Some features are only applicable to the `WebUI` executable that is used to setup your core `WebUI` framework files, while others are only applicable to the `WebUI` library, and finally some are shared by both.

Flag | Lib | Exe | Detail
--- | --- | --- | ---
pages | X | | Include this flag to include some starter_page_* components for setting up some initial starter pages on a new website..
myfi | X | | Include this flag to include components that integrate with [MyFi.ws](https://www.myfi.ws) API services (`*Currently under development`.)

#### Current and Planned Features

Flag | Feature | Status | Detail
--- | --- | --- | ---
pages | privacy | available | General privacy page
pages | terms | available | General terms & conditions page
pages | under_construction | available | General under construction page/placeholder
myfi | feedback | development | Dialog for capturing user feedback
myfi | event logs | development | Helper method for logging event logs
myfi | account services | development | Pages & methods for users to signup/login/logout as well as restricting role based content
myfi | page data | planned | Store and retrieve page data
myfi | website editor | planned | Inline editor for managing page content through the website UI

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
webui = "0.7.21"
# Customize with specific feature flags (pages is included by default)
# webui = { version = "0.7.21", default-features = false }
# webui = { version = "0.7.21", features = ["all"] }
# webui = { version = "0.7.21", features = ["myfi", "pages"] }
# Use the direct GitHub reference if you want bleeding edge updates
# webui = { git = "https://github.com/StoicDreams/RustWebUI", branch = "main" }
```

Install webui executable - this will be used to build your boilerplate static files.

> Note: We recommend that you generally set the same feature flags for your `webui` executable that you set for your use of the `webui` library to assure accompanying starter files are included.
> That said, not all features are shared across the Lib version and Exe versions of `WebUI`. See the [Features](#features) section above for more information on specific features to use between the `WebUI` executable and library.

```bash
# default installation, includes starter pages md files.
cargo install webui

# explicitly include starter pages
cargo install webui --features pages

# example setting multiple features
cargo install webui --features "pages myfi"

# install with all features enabled
cargo install webui --all-features

# exlude any optional features
cargo install webui --no-default-features
```

Run `webui` in your projects root folder (not `src`) to add static files (index.html, css, etc.)

```bash
webui
```

### Web UI Files

Note that you should run `webui` command anytime you update to a new version, making sure to update both the webui executable and the webui dependency in your project.

Certain files are considered static and are not meant to be manually updated. These files will be overwritten during updates.

Other files are considered starter files that you will probably want or need to update and will not overwrite an existing file during updates. If you want to update to the latest starter file, then you will need to delete or rename your existing file - recommend renaming file, such as postfixing .bck to the file name, so you can copy over your customizations to the new Web UI file once it's created.

| File | Starter | Static | Details |
| --- | :---:| :---: | --- |
| index.html | X | | Update metadata information and add any links for extended js/css functionality. |
| app.webmanifest | X | | Update with information about your app. This is used for installable SPAs. |
| robots.txt | X | | Update as needed to manage search bot rules. |
| Logo.svg | X | | Placeholder logo. Update with your own.
| service-worker.js | X | | Basic service worker for file caching, also required for installable SPA apps. |

### Run Dev Server for Testing

Run `trunk serve --open` from your project root to run your site locally. The `--open` flag will open your website in your default browser. If you already have the page open you can exclude the `--open` flag.

```bash
trunk serve --open
```

## Author

**[Erik Gassler](https://www.erikgassler.com) - [Stoic Dreams](https://www.stoicdreams.com)** - Forging solutions for tomorrow's software development.

**Support** - Visit [Stoic Dreams' GitHub Sponsor page](https://github.com/sponsors/StoicDreams) if you would like to provide support to Stoic Dreams.

## License

[MIT](LICENSE)

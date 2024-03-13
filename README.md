# Web UI Workspace - Stoic Dreams

[WebUI Version: 0.7.38](https://github.com/StoicDreams/RustWebUI)

[View Rust WebUI Docs - docs.rs/webui](https://docs.rs/webui)

[View on crates.io - crates.io/crates/webui](https://crates.io/crates/webui)

[View on GitHub - github.com/StoicDreams/RustWebUI](https://github.com/StoicDreams/RustWebUI)

[View Demo and Doc Site - webui.stoicdreams.com](https://webui.stoicdreams.com)

[Stoic Dreams Discord](https://discord.gg/Z6WFptDyQn)

## About

Web UI is a framework with the purpose of simplifying and minizing boilerplate code when creating new SPA websites built in Rust and running solely on front-end webassembly (i.e. a web browser.)

Web UI is not suitable for those looking to build a website that is hosted and run on a webserver, only feeding html, css, and javascript to a browser.

## Goals

- Minimize boilerplate code when starting new website projects.
- Adhere to Material UI standards.
- Provide robust component systems for handling common display types, user input and interactions, and data processing.

## Getting Started

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
rustup target add wasm32-unknown-unknown
```

### Testing Changes to Web UI

Install webui executable from local development.

```bash
# From the workspace root folder
cargo install --path webui --features all
```

Delete any starter files that need refeshing (index.html, etc) and run `webui`

```bash
# From the webapp folder
webui
```

Run `trunk serve --open` from your UI project root (e.g. `webapp`) to run your site locally.

```bash
trunk serve --open
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

### Web UI Save and Publish

This console app provides a single command to:

- Sync static files from webapp to webui
- Increment the Patch version for webui
- Verify tests
- Commit changes
- Push to GitHub
- Publish to crates.io

Install the `webuisave` script

```bash
# From the workspace root folder
cargo install --path webuisave
```

Run the script whenever you're ready to save and publish updates.

```bash
# From the workspace root folder
webuisave -c "Your commit message"
```

```bash
# Increment minor version
webuisave -c "Your commit message" --minor
```

```bash
# Increment major version
webuisave -c "Your commit message" --major
```

## Other Resources

[Rust Docs](https://www.rust-lang.org/)
[Yew Docs](https://yew.rs/)

## Author

**[Erik Gassler](https://www.erikgassler.com) - [Stoic Dreams](https://www.stoicdreams.com)** - Forging solutions for tomorrow's software development.

**Support** - Visit [Stoic Dreams' GitHub Sponsor page](https://github.com/sponsors/StoicDreams) if you would like to provide support to Stoic Dreams.

## License

[MIT](LICENSE)

# Web UI Workspace - Stoic Dreams

[WebUI Version: 0.10.11](https://github.com/StoicDreams/WebUI)

[View Demo and Doc Site - webui.stoicdreams.com](https://webui.stoicdreams.com)

[View Rust WebUI Docs - docs.rs/webui](https://docs.rs/webui)

[View on crates.io - crates.io/crates/webui](https://crates.io/crates/webui)

[View on GitHub - github.com/StoicDreams/WebUI](https://github.com/StoicDreams/WebUI)

[Stoic Dreams Discord](https://discord.gg/Z6WFptDyQn)

## About

Web UI is a Vanilla JavaScript Web Component library purpose of simplifying and minizing boilerplate code when creating new SPA websites.

The Web UI Rust Crate (WebUI) is also being designed to allow running a Rust language WebAssembly companion that allows communication between your Rust services running in the browser and your JavaScript code and/or Web UI app services.

> It is important to note that the purpose of WebUI is to have a separation between Rust code and UI logic. It is our oppinion that Rust code should not be managing or building UI (i.e. HTML), but instead should be relegated to performing process intensive or long tasks.

Web UI is very early in development and is subject to breaking changes at any time while we are in this experimental stage of development.

## Goals

- Minimize boilerplate code when starting new website projects.
- Define and Adhere to Web UI standards.
- Provide robust component systems for handling common display types, user input and interactions, and data processing.

## Planned Features and Updates (Not currently implemented)

- Setup Rust wasm to run in Web-Worker.
- Implement boilerplate for interacting between Rust and JavaScript into WebUI.
- Implement local Web UI web-component files (JavaScript/CSS/HTML) into WebUI.
  - Developers will be able to choose between using a specific version/state of the Web UI files that are stored locally or use the live version on [cdn.myfi.ws](https://cdn.myfi.ws).

## Getting Started

The following instructions are for our developers working on the Rust Web UI solution.

If you're wanting instructions on how to use Web UI to develop websites, whether for new or existing sites, then check out our [Web UI documentation and Demo](https://webui.stoicdreams.com) website.

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

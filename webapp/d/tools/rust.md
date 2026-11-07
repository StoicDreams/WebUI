<webui-data data-page-title="Rust Integration" data-page-subtitle=""></webui-data>

## Rust Integration

For integrating Rust with the Web UI, you can use the following tools and setup:

### Rust Tools Setup

Follow the instructions [here](https://www.rust-lang.org/tools/install) to install Rust and Cargo, which are essential for building and managing Rust projects.

```terminal:Verify Rust and Cargo installation
rustc --version # Should print the version of Rust.
cargo --version # Should print the version of Cargo.
```

> You will also need to install the Cargo crate `trunk` for building and serving your Web UI applications.

```terminal:Install Trunk
cargo install trunk --locked
```

> You will also need to install the `wasm-pack` tool, which is used for building Rust-generated WebAssembly packages.

```terminal:Install wasm-pack
cargo install wasm-pack
```

### Web UI Rust Tools

Web UI has a post-build tool that helps with integrating Rust projects into Web UI applications. You can install it using Cargo:

```terminal:Install Web UI Post Build Tool
cargo install webapp_post_build --git https://github.com/StoicDreams/WebUI
```

Trunk runs it after each build, as the `post_build` hook in `Trunk.toml`. It:

- Copies `root_files` to the site root.
- Removes the copy of the site's WebAssembly that Trunk starts on the main thread of every page. Web UI runs the site's Rust in a web worker (`webui.worker`), which loads its own copy only when a page needs it. If any script uses `window.wasmBindings` or `TrunkApplicationStarted`, the main thread copy is kept.
- Writes a static HTML file for each page in `d/`, with its title, description, and text, for search engines and link previews.
- Adds preload hints to each of those pages for the components it uses, including everything the site's own `app-*` components use, so they load alongside the framework instead of one level at a time.

Web UI has a `webui` command tool that can be used to setup boilerplate code for a new or existing Web UI Rust project.

```terminal:Install Web UI CLI Tool
cargo install webui --git https://github.com/StoicDreams/WebUI
```

## Run Weebsite

Web UI with Rust integrations are expected to be built using Rust's `trunk serve` command.

```terminal:Run Website from Root
trunk serve --config webapp/Trunk.toml --port 3210
```

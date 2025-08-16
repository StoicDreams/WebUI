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
cargo install trunk
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

Web UI has a `webui` command tool that can be used to setup boilerplate code for a new or existing Web UI Rust project.

```terminal:Install Web UI CLI Tool
cargo install webui --git https://github.com/StoicDreams/WebUI
```

## Run Weebsite

Web UI with Rust integrations are expected to be built using Rust's `trunk serve` command.

```terminal:Run Website from Root
trunk serve --config webapp/Trunk.toml --port 3210
```

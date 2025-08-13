<webui-data data-page-title="Web UI Framework" data-page-subtitle=""></webui-data>

<webui-sideimage src="https://cdn.myfi.ws/v/Vecteezy/cartoon-style-cloud-storage-data-processing-message.svg">
> Build modern, fast web apps with a lightweight framework designed for WebAssembly‑powered SPAs - without drowning in boilerplate.
> <webui-flex align="center" justify="center">[Get Started](/getting-started) :dot: [Explore Components](/components) · [Concepts](/concepts/architecture) · [Icons](/icons) · [Changelog](/changelog)</webui-flex>

### Why Web UI

<webui-page-segment elevation="10">
- Vanilla JavaScript / HTML / CSS based Web Components.
- Fast starts, less boilerplate. Spin up projects quickly and focus on features, not scaffolding.
- WebAssembly friendly. Built with a smooth path for Rust → WASM apps and SPA workflows.
- Practical docs & examples. Clear guidance from “hello world” to production‑minded patterns.
- We chose to use Vanilla JavaScript as we at Stoic Dreams find developing with pure JavaScript, HTML, and CSS to be far more productive versus any other languages or frameworks. And it doesn't hurt that this also makes it virtually seemless to integrate Web UI components into any existing website, regardless of how it's built.
</webui-page-segment>

> [warning] That said, Web UI is not intended to be any kind of one-size-fits-all library. It is a very opinionated library. It is expected that if a web component does not work how you think it should, then you can copy the component to your own library and modify it there to fit your needs.
</webui-sideimage>

## Quick Start

<webui-page-segment elevation="10">
Web UI components require the Web UI loader.js file to manage dynamically loading component setup when component html is detected for unknown components. This file also handles a lot of other core Web UI features.
</webui-page-segment>

<webui-page-segment>
```html:index.html
<html>
<head>
    ...
    <link href="https://cdn.myfi.ws/css/webui.min.css" rel="stylesheet" />
    <script src="https://cdn.myfi.ws/webui/loader.min.js"></script>
    ...
</head>
<body>
    <webui-app-config src="appConfig.json"></webui-app-config>
    <webui-app data-removeclass=".nav|open;.shared|open">
        ...
    </webui-app>
</body>
</html>
```
</webui-page-segment>

> At this time, Web UI is designed with the intent that it will manage your page navigation and should be used with the `webui-app` for standard layouts, or a custom `app-app` component for customized layouts.
> In the future, we will add additional layout components as well as update the loader.js to make certain features able to be disabled through appConfig.json flags.

<webui-sideimage reverse src="https://cdn.myfi.ws/v/Vecteezy/online-big-data-courses-illustration-exclusive-design.svg">

### What You’ll Find in the Docs

<webui-page-segment elevation="10">
- **Getting Started** - install, run locally, first page, next steps.
- **Components** - reusable building blocks with guidance and examples.
- **Core Concepts** - architecture, state/events, routing, data/networking, styling & theming, accessibility, performance.
- **Guides** - tutorials, FAQs, and troubleshooting.
- **Deployment** - hosting, CDN/versioning, environments.
</webui-page-segment>

> <webui-flex align="center" justify="start">Jump in: [Getting Started](/getting-started) · [Components](/components) · [Guides](/guides/tutorials)</webui-flex>

#### Status & Roadmap

<webui-page-segment elevation="10">
Web UI is in early development. Expect rapid iteration and occasional breaking changes while we stabilize APIs and polish docs. Track updates in the [Changelog](/changelog).

</webui-page-segment>
> [warning] For now Web UI is not supporting versioning. Everything is considered beta (version 0.X). When we eventually level-up to version 1.0, that will include a versioning system.
</webui-sideimage>

<webui-sideimage src="https://cdn.myfi.ws/v/Vecteezy/filling-completed-not-completed-marking-important-dates-and.svg">

##### Community & Contributing

<webui-page-segment elevation="10">
We welcome issues, ideas, and improvements. See [Contributing](/contributing) for guidelines and ways to help shape the framework.
</webui-page-segment>

> [warning] Help [Sponsor Stoic Dreams](https://github.com/sponsors/StoicDreams) if you like what we do and would like to help support us and help with our server costs and eventually allow us to transition to working on Web UI, Web UI Icons, and our other software development tools full time.
</webui-sideimage>

## Web UI Powered Websites

<webui-cards card-width="500" elevation="n10" src="/cards/webui-powered-websites.json" theme="tertiary"></webui-cards>

> [info] Source code for all Web UI web components can be found in our [My Fidelity CDN Repo on GitHub](https://github.com/StoicDreams/MyFiCDN/tree/main/cdn/webui).

<webui-data data-page-title="Web UI Framework" data-page-subtitle=""></webui-data>


<webui-sideimage src="https://cdn.myfi.ws/img/webui/screenshot-code-index.webp" alt="Screenshot of index.html code">
## What is Web UI?

<webui-page-segment elevation="10">
Web UI is more than just a new library of web components. Web UI is a lightweight framework that can greatly speed up your UI development by reducing how much boilerplate is needed when building new sites or adding new features.

Web UI uses modern web standards - no JavaScript frameworks or time-consuming bundlers required. But Web UI is designed to work stand-alone or as part of any other tools you might be using. This is because Web UI is designed to be useable like any other HTML - just a lot more powerful.

Web UI is designed to be lightweight, fast, and easy to use. And while Web UI components are explicitly opinionated in nature, being open-source web components, they are still easily adaptable to suite your needs and styles.
</webui-page-segment>
</webui-sideimage>

### Web UI is 100% Free and Open-Source

<webui-page-segment elevation="10">
And We're not just talking about starter components that will leave you wanting more. 100% of Web UI is Free and Open-Source, now and forever.

We will never offer a paid "PRO" version, because we believe free and open source tools should be exactly that.
</webui-page-segment>

<webui-sideimage reverse src="https://cdn.myfi.ws/img/eg/me_256.webp" alt="Photo of Erik Gassler">
## Greetings from the founder of Stoic Dreams and creator of Web UI!
<webui-quote elevation="10" cite="Erik Gassler">
I originally founded Stoic Dreams to be the branding for my software development experiments back in 2008. Since then I've developed several websites, desktop tools, and even game concepts. But relatively recently I realized the projects that I get most passionate about is developing tooling that help to improve efficiencies, productivity, and workflows for myself and fellow developers.

I've done a lot of work in this space for myself as well as for the various companies I've worked for in my career, including a few years at Microsoft - optimizing workflows for the teams I worked in. But now I want to expand to do more open-source projects and not confine these tools to just myself or single companies and teams.

And this is why Web UI was created. While I have always loved building my own tooling and frameworks rather than use others, I am getting tired of always rebuilding new tools from scratch. So, I made the decision to build a new website UI framework that wouldn't be locked down to any languages, framework, or tooling. And with Web UI, I did just that.

Currently, the development of Web UI and its infrastructure is funded by me, but I have a very limited budget, as well as limited time working around paying jobs and spending time with family.

My goal long-term is to have enough site traffic that Stoic Dreams can be supported purely on ads and affiliate links, and maybe a few services that would utilize subscriptions. And when I have enough revenue I will be able to work on Stoic Dreams projects like Web UI full time, and hopefully even be able to hire other developers and artists.

In the meantime you can help support Web UI through our [Stoic Dreams sponsors page on GitHub](https://github.com/sponsors/StoicDreams).
</webui-quote>
</webui-sideimage>

<webui-sideimage src="https://cdn.myfi.ws/img/webui/screenshot-icons.webp" alt="Screenshot of Web UI Icons">
## Web UI Includes Icons with Animated Transitions

<webui-page-segment elevation="10">
Still early in development, but one of the Web UI components included is an icon system that enables animated transitions for browsers that support it.

Web UI icons are also designed to only load as needed, so your page loading is never bogged down by loading a bunch of icons that your site will never use. And you don't have to waste time creating and maintaining icon bundles.
</webui-page-segment>
</webui-sideimage>

<webui-sideimage reverse src="https://cdn.myfi.ws/img/webui/screenshot-methods.webp" alt="Screenshot of Web UI Methods">
### JavaScript Tools

<webui-page-segment elevation="10">
Web UI utilizes a loader that is used to both simplify web component development as well as dynamically load web components as they are used in HTML.

With this loader you also get the `webui` object with many helper methods - most of which were created for the explicit use inside of Web UI components.
</webui-page-segment>
</webui-sideimage>

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

> [warning] That said, Web UI components are not intended to be one-size-fits-all solutions. Web UI is a very opinionated library. But Web UI is also easily extensible, and it is expected that if a web component does not work how you think it should, then you can copy the component to your own library and modify it there to fit your needs. Web UI supports loading framework components defined in our CDN as well as custom components stored locally in your website files.
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

<webui-page-segment elevation="10">
Here are our featured projects currently powered by Web UI. If you are using Web UI in a project and would like it featured here, then let us know through our <a data-click="feedback">feedback</a>, on our [Web UI Discord Channel](https://discord.com/channels/972856291909332993/1409218226218467452), or send us a message at [support@stoicdreams.com](mailto:support@stoicdreams.com?subject=Feature Project).
</webui-page-segment>

<webui-cards card-width="500" elevation="n10" src="/cards/webui-powered-websites.json" theme="tertiary"></webui-cards>

> [info] Source code for all Web UI web components can be found in our [My Fidelity CDN Repo on GitHub](https://github.com/StoicDreams/MyFiCDN/tree/main/cdn/webui).

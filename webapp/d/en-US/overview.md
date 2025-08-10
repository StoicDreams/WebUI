<webui-data data-page-title="Overview" data-page-subtitle=""></webui-data>

<webui-side-by-side>
<webui-paper class="content">
## TL;DR

<webui-page-segment elevation="10">
Web UI is a lightweight, Vanilla JS component framework built on Web Components and a simple‑config loader. It helps teams ship features fast - without bundlers, compiling, lock‑in, or a massive dependency tree.
</webui-page-segment>

> **Core ideas** :key:: native‑first, predictable APIs, small surface area, CDN‑friendly delivery, and components that fail gracefully.
</webui-paper>

<webui-paper class="content">
### Why Web UI?

<webui-page-segment elevation="10">
Most projects only need a sharp set of **well‑behaved UI primitives** - not a 700kb framework and six build steps. After years of rebuilding the same patterns for security, performance, and reliability, we distilled them into a **reusable, opinionated** toolkit you can drop into any website or embeded webview.

**What you get**

- Native Web Components (no framework runtime tax)
- Auto discovery/initialization via a loader
- Stable, documented attributes & events
- Accessible defaults (ARIA‑friendly patterns)
- Design‑system‑friendly tokens and CSS variables
- CDN delivery for instant trials and production scale
</webui-page-segment>
</webui-paper>
</webui-side-by-side>

#### How it works (at a glance)

<webui-side-by-side>
<webui-page-segment elevation="10">
You apply the minimal HTML for Web UI to load and run.
1. **Include CSS & loader** for base styles and functionality
2. **Include App Config** to override defaults
3. **Include App Component** to define layout and include key
4. **Author semantic HTML** with attributes
The loader automatically handles complex processes for you.
- **Loader initializes** components it finds on the page
- **Loader manages nav** from relative links
- **Loader manages data** subscriptions and triggers
</webui-page-segment>

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
</webui-side-by-side>

> The loader autonomously discovers and hydrates components. No imports, no bundler config, no global init scripts.

<webui-side-by-side>
<webui-paper class="content">
##### Design Principles

<webui-page-segment elevation="10">
- **Native‑first:** Prefer platform APIs over abstractions.
- **Small & sharp:** Each component solves one thing well.
- **Explicit configuration:** Props via attributes; events are documented and typed.
- **Stable contracts:** Backwards‑compatible changes favored.
- **HTML data syncing:** Elements use `data-subscribe` and `data-trigger` to sync data changes on updates.
</webui-page-segment>
</webui-paper>
<webui-paper class="content">
###### What’s in the box

<webui-page-segment elevation="10">
Common categories you’ll reach for:
- **Inputs & Forms:** validated inputs, toggles, selects, date/time patterns
- **Navigation & Layout:** tabs, drawers, grids, responsive utilities
- **Feedback & Overlays:** toasts, dialogs, confirmations, progress/loading
- **Data Display:** reports/tables, empty states, cards, badges, avatars
- **Productivity Primitives:** menus, dropdowns, pagination, filter bars
- **Icons:** one SVG icon with endless translations
</wegbui-page-segment>
</webui-paper>
</webui-side-by-side>

## Getting started

<webui-side-by-side>
<webui-page-segment elevation="10">
1. Add the CDN links (CSS + loader)
2. Drop the following sample components into your markup
**Sample Components**
<webui-input-text label="Your Message" data-subscribe="page-test" data-trigger="page-test"></webui-input-text>
<webui-quote theme="success">
    Your Message: <span data-subscribe="page-test:text"></span>
</webui-quote>
</webui-page-segment>

<webui-page-segment>
```html:Sample Components
<webui-input-text label="Your Message" data-subscribe="page-test" data-trigger="page-test"></webui-input-text>
<webui-quote theme="success">
    Your Message: <span data-subscribe="page-test:text"></span>
</webui-quote>
```
</webui-page-segment>
</webui-side-by-side>

> Quick test: open a plain HTML file, paste the includes + one component. If it renders with default styling and behavior, and you can type a message and see your message duplicated in the quote, then you’re good to go.

<webui-side-by-side>
<webui-paper class="content">
### Theming & Tokens

<webui-page-segment elevation="10">
- **CSS Variables:** Customize color, radius, shadow, spacing, typography.
- **Opt‑in Dark Mode:** Respects `prefers-color-scheme`; override at container level.
- **Scale safely:** Set global tokens at `:root`; override per component as needed.
</webui-page-segment>
</webui-paper>
<webui-paper class="content">
#### Accessibility

<webui-page-segment elevation="10">
- Keyboard‑first UX, logical tab order, visible focus outlines
- ARIA roles/labels where applicable
- Motion reduced when `prefers-reduced-motion` is set
</webui-page-segment>
</webui-paper>
</webui-side-by-side>

> Ship with confidence: test with a screen reader and keyboard only. All critical actions should be reachable and visible.

<webui-side-by-side>
<webui-paper class="content">
##### Performance

<webui-page-segment elevation="10">
- Minimal JS per component, lazy‑loaded via the loader
- CSS is scoped and cacheable via CDN
- No required application framework runtime

**Tips**

- Keep initial HTML light; components progressively enhance.
- Prefer attribute configs over large inline scripts.
- Defer non‑critical UI until user interaction.
</webui-page-segment>
</webui-paper>
<webui-paper class="content">

###### Compatibility

<webui-page-segment elevation="10">
- **Modern evergreen browsers** (Chromium, Firefox, Safari)
- Works without a build step; integrates with static sites, SSR, SPAs, and classic multi‑page apps
</webui-page-segment>
> Supporting legacy browsers that lack Custom Elements/Shadow DOM may require polyfills.
</webui-paper>
</webui-side-by-side>

<webui-side-by-side>
<webui-paper class="content">
## Versioning & Stability

<webui-page-segment elevation="10">
- **Semantic Versioning:**
  - Patch = fixes
  - Minor = new, backwards‑compatible features
  - Major = intentional breaks with migration notes
- **Changelogs:** Documented per release with upgrade guidance
- **Deprecations:** Marked in docs with timelines and alternatives
</webui-page-segment>
</webui-paper>
<webui-paper class="content">

### When to use

<webui-page-segment elevation="10">
**Great fits:**
- A small, dependable toolkit you can learn once and reuse everywhere
- Teams mixing stacks or migrating incrementally
- Fast paths for prototypes and production
**Works alongside:**
- Reactive frameworks (React, Vue, Svelte): Use Web UI elements as leaf nodes. Pass state via attributes/data-*, and handle outputs through CustomEvents.
- SSR + hydration (Next.js, Nuxt, SvelteKit, Astro): Render markup on the server; include the loader on the client to progressively enhance. For islands/partial hydration, mount components inside the island to avoid double-hydration.
- Routers & state managers: Keep app state in your framework; communicate with components via attributes and events.
</webui-page-segment>

</webui-paper>
</webui-side-by-side>

> Web UI is HTML‑first. If your stack can render HTML and supports modern JavaScript, it can render Web UI components.

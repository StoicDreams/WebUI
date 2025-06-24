<webui-data data-page-title="Web UI documentation and demo website" data-page-subtitle=""></webui-data>

<webui-sideimage src="https://cdn.myfi.ws/v/Vecteezy/cartoon-style-cloud-storage-data-processing-message.svg">

Welcome to the Web UI toolkit.

Web UI at it's core is a collection of Vanilla JavaScript based Web Components.

We chose to use Vanilla JavaScript as we find developing with JavaScript to be far more productive versus any other languages or frameworks. And it doesn't hurt that this also makes it virtually seemless to integrate Web UI components into any existing website, regardless of how it's built.

That said, Web UI is not intended to be any kind of one-size-fits-all library. It is a very opinionated library. It is expected that if a web component does not work how you think it should, then you can copy the component to your own library and modify it there to your hearts content.

Source code for all Web UI web components can be found in our [My Fidelity CDN Repo on GitHub](https://github.com/StoicDreams/MyFiCDN/tree/main/cdn/webui).

</webui-sideimage>

<webui-quote theme="info">

<strong>Special Announcement!</strong> We have started develolping our own custom icons. Check out our [Web UI Icons Page](/icons) to see what we have so far.

</webui-quote>

<webui-quote theme="warning">

Help [Sponsor Stoic Dreams](https://github.com/sponsors/StoicDreams) if you like what we do and would like to help support us so we can transition to working on Web UI, Web UI Icons, and our other software development tools full time.

</webui-quote>

<webui-sideimage reverse src="https://cdn.myfi.ws/v/Vecteezy/online-big-data-courses-illustration-exclusive-design.svg">

Our concept for Web UI actually began with wanting to build a component library for Rust when we decided to switch our development from C# tech stacks to Rust tech stacks.

While we found Rust to be much faster and easier to work with versus C# (especially Blazor), we still found the developer experience severily lacking when compared to our previous years of developing frontends using Vanilla JavaScript, HTML, and CSS.

And then earlier in 2024 we discovered Web Components. A browser/JavaScript native method of creating custom html elements. How had we missed this? Since this discovery we have retooled our Web UI concept to be strict JavaScript/HTML/CSS at its core. While also rethinking how we will connect with Rust WebAssembly to offload work for specific tasks - like a background worker.

</webui-sideimage>

<webui-sideimage src="https://cdn.myfi.ws/v/Vecteezy/filling-completed-not-completed-marking-important-dates-and.svg">

We welcome you to explore our tools. And please don't hesitate to <a data-click="feedback">send us a message through our feedback dialog</a> or come see us on Discord.

Web UI is very much early in development, and our documentation is very minimal as the development of Web UI is in very experimental stages and prone to rapid changes, such as our recent move to making Web UI a Vanilla JavaScript component library instead of a Rust Framework.

For now consider Web UI in a version 0 stage. When we are ready to move to version 1.0 and beyond we will implement a version snapshot system so users can use a static version without fear of random/breaking changes being introduced when using a specific version of Web UI.

</webui-sideimage>

<webui-cards card-width="500" src="/cards/welcome.json"></webui-cards>

## Web UI Powered Websites

<webui-cards card-width="500" elevation="n10" src="/cards/webui-powered-websites.json" theme="tertiary"></webui-cards>

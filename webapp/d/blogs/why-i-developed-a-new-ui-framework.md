<webui-data data-page-title="Why I Decided to Create My Own UI Framework" data-page-subtitle=""></webui-data>

> [Erik Gassler] Published on: September 7, 2025

<webui-page-segment elevation="10">
I've always been the kind of developer who prefers to build things from scratch. While peers would argue with me "there's no need to reinvent the wheel", I'd argue back the available wheels aren't good enough to meet my standards.

I often found existing tools and libraries just didn't meet my expectations for speed, reliability, or ease of use. Or they might only solve 90% of the problem, while getting in the way of solving the remaining 10%. Or they might solve a specific problem well but were packaged with a bunch of bloat features that would consume resources even though my application didn't need or use them.
</webui-page-segment>

## Why not just use what's already out there?

<webui-page-segment elevation="10">
The front-end ecosystem is full of options. React, Vue, Angular, Svelte - along with countless component libraries - giving developers nearly endless ways to build. So why build another one?

Because time and again, I kept running into the same problems:

- **Complexity Over Simplicity**: Many frameworks ship with far more than most teams actually need or will ever use, leading to bloated bundles and learning curves that slow down development and potentially UI performance. I want a framework that doesn't require builds or bundling and will only load to the UI the minimum needed to run the framework and components that are used.
- **Inconsistent Experiences**: Switching projects meant re-learning patterns and conventions, even for basic UI building blocks. I wanted a framework that was native to the browser, using standard HTML, CSS, and JavaScript, so it was completely agnostic of any other tooling or languages I may be using on the backend, or might want to use alongside it on the front-end.
- **Accessibility as an Afterthought**: Too often, meeting accessibility requirements requires its own separate development cycles and resources. I want a framework that builds this in natively to minimize how much effort is required to make my projects meet accessibility standards.

At some point, I realized I didn't just want a library of components. I wanted something different - a **foundation** that was lightweight, adaptable, and usable no matter what stack I was working in.
</webui-page-segment>

## The philosophy behind Web UI

<webui-page-segment elevation="10">
From the start, I knew Web UI wasn't going to be "yet another component library". Instead, I built it with a few guiding principles:

- **Simplicity First - Minimize Boilerplate**: Package features into web-components that minimize boilerplate when developing UIs.
- **Framework-Agnostic**: Whether you use React, Vue, Svelte, plain HTML, or any other web frameworks, Web UI should fit in naturally.
- **Accessible by design**: Keyboard navigation, ARIA roles, and contrast checks aren't optional - they're defaults.
- **Customizable**: Theming should be easy and flexible, not a chore.
- **Opinionated but Extendable**: Web UI should by default be opinionated to provide a consistent user experience across its components while also being easy to extend with additional components and features to meet the demands of developers.
</webui-page-segment>

## Lessons learned along the way

<webui-page-segment elevation="10">
Looking back at the frameworks and tools I have built for myself and teams over the years, I noticed some patterns. The tools that stuck were the ones that:

- **Minimized Friction**: They didn't force developers into rigid or complex patterns. They didn't make maintenance harder. They let teams focus on building their products instead of fighting with their tools.
- **Solved Real-World Problems**: They weren't created to solve a conceptual problem that might be faced in the future, instead they solved a real-world problem that the team or I was facing at the time.
- **Solved a Core-Issue**: The issue solved were a core problem, not just a symptom of another issue. Tools that only resolve symptoms over time will often at-best become redundant, and at worst will mask the core issues and/or create their own issues.

Web UI is my attempt to distill all those lessons into something practical - a framework I would have loved to have on every past project, and a framework I will use on every current and future project.
</webui-page-segment>

## Looking forward

<webui-page-segment elevation="10">
I don't expect Web UI to be the tool for everyone, and that's okay. My goal isn't to replace any big players in the ecosystem. First and foremost, Web UI is a tool I designed for myself to simplify projects I work on so I can focus less on UI and more on solving other issues. But I also believe in sharing knowledge and tools freely to help everyone, not just myself - which is why Web UI is an open-source framework that is free for anyone to use, copy, and modify as they see fit.

If you're like me and feel like existing UI frameworks are too heavy, too complex, or just make development harder rather than easier, then Web UI might be for you.
</webui-page-segment>

<webui-page-segment elevation="10">
I welcome you to share with me your <a data-click="feedback">feedback, thoughts, or opinions</a> that may help me continue to evolve and improve Web UI.
</webui-page-segment>

## Welcome to the Web UI documentation and demo website

```SideImage "right" "https://cdn.myfi.ws/v/Vecteezy/cartoon-style-cloud-storage-data-processing-message.svg"
We are very excited to be sharing this tool with you, as well as our journey into the world of Rust development.

One of our last major projects at Stoic Dreams was working on a UI framework utilizing the C# Blazor framework, which allows for C# code and Razor pages to be compiled to webassembly, thus allowing for C# native libraries and code to be used for developing front-end web applications.

This was a very interesting framework for us, as we have always loved the C# language and were very excited by the prospect of having a single language for both front-end and back-end development.

But Blazor is not without it's major issues, which we will not dive into here. But, you can check out our [Blazor UI project](https://blazorui.stoicdreams.com "Blazor UI documentation and website demo") to see for yourself the differences between it and what we're accomplishing here.
```

```SideImage "left" "https://cdn.myfi.ws/v/Vecteezy/online-big-data-courses-illustration-exclusive-design.svg"
But then we were introduced to Rust. A language that has the power of C and C++ languages, but removes many of the pitfalls. A language with thread and memory safety built in, and no garbage collection. Something we hate about C#.

And doing more research into the language, we found that it has a robust and rapidly growing community of developers that are passionate about the language and the power and safety it provides us.

Then as we're digging more into the language we discover it has documentation and testing systems built into the language, that make it much easier to self document code in a manner that is easily consumable, as well as much easier to manage testing, such as the ability to write tests as part of the documentation so they serve as both an example for other developers, as well as a unit test to verify your code is working - which also assures your documentation stays up to date as you make changes.
```

```SideImage "right" "https://cdn.myfi.ws/v/Vecteezy/filling-completed-not-completed-marking-important-dates-and.svg"
So then after much consideration and evaluation, we made the decision to jump in and commit to Rust, re-writing all of our C# projects to be powered by Rust.

And so far we are very glad we made this decision.

We have fallen in love with Rust, and for a very simple reason. It's the most productive language we've ever used.

What do we mean by this? Well, Rust is fast. Super fast. And it makes us very fast to develop and update features.

And since Rust is so strict about compilation errors, this means that most bugs are caught by the compiler, giving us immediate feedback on not only the problem, but many times helpful solutions to fix the problem.

This feedback loop being so short, means problems are identified and resolved in much shorter cycles. Which then translates to us being able to create software that works in faster iteration cycles."
```

````cards

```card "Getting Started" "500" "tertiary" "fa-duotone fa-star"
Web UI is very early in development, as we just started working on Web UI on Sept 28th, 2022

Expect rapid iterations with breaking changes, even in patch version updates, prior to releasing a 1.0 version, which we expect to be sometime in early 2023.

Web UI is currently being used for this website, and will soon be powering other Stoic Dreams websites.

If you are fine dealing with frequent breaking changes and would like to start using Web UI for your project, we would appreciate you letting us know what you think about Web UI and what you would like to see in it through either a Feedback message or in [the Stoic Dreams discord server](https://discord.com/channels/972856291909332993/1025781071608037466 "Web UI at Stoic Dreams Discord server") to help us prioritize features and updates based on what people want from Web UI.

We will be updating this website to provide full documentation, but for now you can check out [our Rust-built documentation](https://crates.io/crates/webui "Web UI rust crate documentation").
```

```card "Where is the Documentation" "500" "tertiary" "fa-duotone fa-books"
We expect some may be wondering why we aren't developing documentation within this website in parallel with adding features to the Web UI framework.

The answer is quite simple. We are not expecting anybody to be using Web UI at this point, and a lot of the work we are doing is experimental and subject to frequent changes or entire re-writes.

So to minimize our overall time spent working on documentation, we are waiting for features to become a bit more finalized.

Of course, if you are someone who is interested in getting started with Web UI now then please let us know and we will re-prioritize adding documentation.

```list
GitHub: [Web UI open-source web development framework.](https://github.com/StoicDreams/RustWebUI)

Rust WebUI Docs: [Developer documentation for Web UI.](https://docs.rs/webui)

Crates.io: [Crate information for Web UI.](https://crates.io/crates/webui)
```

```

````
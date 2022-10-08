use webui::*;

/// App home page
pub(crate) fn page_home() -> Html {
    html! {
        <>
            <Paper class="d-flex flex-row flex-wrap">
                <Paper class="flex-grow">
                    {title_secondary!("Welcome to the Web UI documentation and demo website")}
                    <Paper class="pa-3">
                    {paragraphs!(
                        "We are very excited to be sharing this tool with you, as well as our journey into the world of Rust development.",
                        "One of our last major projects at Stoic Dreams was working on a UI framework utilizing the C# Blazor framework, which allows for C# code and Razor pages to be compiled to webassembly, thus allowing for C# native libraries and code to be used for developing front-end web applications.",
                        "This was a very interesting framework for us, as we have always loved the C# language and were very excited by the prospect of having a single language for both front-end and back-end development.",
                        html!{
                            <>
                                {"But Blazor is not without it's major issues, which we will not dive into here. But, you can check out our "}
                                <a href="https://blazorui.stoicdreams.com">{"Blazor UI project"}</a>
                                {" to see for yourself the differences between it and what we're accomplishing here."}
                            </>
                        },
                        "But then we were introduced to Rust. A language that has the power of C and C++ languages, but removes many of the pitfalls. A language with thread and memory safety built in, and no garbage collection. Something we hate about C#.",
                        "And doing more research into the language, we found that it has a robust and rapidly growing community of developers that are passionate about the language and the power and safety it provides us.",
                        "Then as we're digging more into the language we discover it has documentation and testing systems built into the language, that make it much easier to self document code in a manner that is easily consumable, as well as much easier to manage testing, such as the ability to write tests as part of the documentation so they serve as both an example for other developers, as well as a unit test to verify your code is working - which also assures your documentation stays up to date as you make changes.",
                        "So then after much consideration and evaluation, we made the decision to jump in and commit to Rust, re-writing all of our C# projects to be powered by Rust.",
                        "And so far we are very glad we made this decision.",
                        "We have fallen in love with Rust, and for a very simple reason. It's the most productive language we've ever used.",
                        "What do we mean by this? Well, Rust is fast. Super fast. And it makes us very fast to develop and update features.",
                        "And since Rust is so strict about compilation errors, this means that most bugs are caught by the compiler, giving us immediate feedback on not only the problem, but many times helpful solutions to fix the problem.",
                        "This feedback loop being so short, means problems are identified and resolved in much shorter cycles. Which then translates to us being able to create software that works much faster."

                    )}
                    </Paper>
                </Paper>
                <iframe title="Discord Widget" class="discord-widget ml-a" src="https://discord.com/widget?id=972856291909332993&theme=dark" width="350" height="500" allowtransparency="true" frameborder="0" sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"></iframe>
            </Paper>
            <Paper class="d-flex flex-gap2 pa-2">
                <Paper style="max-width:400px">
                    {title_tertiary!("Getting Started")}
                    <Paper class="pa-1">
                        {paragraphs!(
                            "Web UI is very early in development so it may not be ready for you to use in a production application.",
                            "That being said, Web UI is powering this website so obviously it's good enough for us to use in our production websites.",
                            html!{
                                <>
                                    {"We will be updating this website to provide full documentation, but for now you can check out "}
                                    <a href="https://crates.io/crates/webui">{"our Rust-built documentation"}</a>
                                    {"."}
                                </>
                            }
                        )}
                    </Paper>
                </Paper>
            </Paper>
        </>
    }
}

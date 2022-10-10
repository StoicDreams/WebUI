use webui::*;

pub fn feedback_button_info() -> DrawerToggleInfo {
    DrawerToggleInfo::new(
        "Give us your Feedback!".to_owned(),
        || html! {<i class="fa-solid fa-comment" />},
        get_render,
    )
    .set_drawer(Direction::Top)
    .set_on_confirm("Send Feedback".to_string(), handle_confirm)
    .build()
}

fn handle_confirm() -> bool {
    jslog!("Feedback confirmed!");
    true
}

pub(crate) fn get_render() -> Html {
    html! {
        <>
            <Paper class="pa-1">
                {paragraphs!(
                {html!{
                    <>
                        {"You can also come "}
                        <Link title="Web UI at Stoic Dreams Discord server" href="https://discord.com/channels/972856291909332993/1025781071608037466">{"chat with us on the Stoic Dreams discord server."}</Link>
                    </>
                }},
                "Coming Soon!"
                )}
            </Paper>
        </>
    }
}

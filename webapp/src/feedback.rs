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
                <p>{"Coming soon"}</p>
            </Paper>
        </>
    }
}

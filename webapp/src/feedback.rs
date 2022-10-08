use webui::{html, Direction, DrawerToggleInfo, Html, Paper};

pub fn feedback_button_info() -> DrawerToggleInfo {
    DrawerToggleInfo::new(
        "Give us your Feedback!".to_owned(),
        || html! {<i class="fa-solid fa-comment" />},
        get_render,
    )
    .set_drawer(Direction::Top)
    .build()
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

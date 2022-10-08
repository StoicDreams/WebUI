use webui::{Direction, DrawerToggleInfo, Paper};
use yew::{html, Html};

pub fn feedback_button_info() -> DrawerToggleInfo {
    DrawerToggleInfo {
        display: || {
            html! {
                <i class="fa-solid fa-comment" />
            }
        },
        title: "Give Feedback".to_owned(),
        drawer: Direction::Top,
        class: "".to_owned(),
        drawer_content: get_render,
    }
}

pub(crate) fn get_render() -> Html {
    html! {
        <>
            <header>
                <h2>{"Give us your feedback!"}</h2>
                <span class="flex-grow" />
            </header>
            <Paper class="pa-1">
                <p>{"Coming soon"}</p>
            </Paper>
        </>
    }
}

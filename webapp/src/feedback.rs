use std::collections::HashMap;

use webui::*;

pub fn feedback_button_info() -> DrawerToggleInfo {
    DrawerToggleInfo::new(
        "Give us your Feedback!".to_owned(),
        || html! {<i class="fa-solid fa-comment" />},
        get_render_wrapper,
    )
    .set_drawer(Direction::Top)
    .set_on_confirm("Send Feedback".to_string(), handle_confirm)
    .build()
}

fn handle_confirm() -> bool {
    let feedback = GlobalData::get_data::<String>("feedback".to_string());
    match feedback {
        Ok(value) => {
            let mut post_data = HashMap::from([
                ("Message", value)
            ]);
            jslog!("Got feedback: {:?}", post_data);
            match serde_json::to_string(&post_data) {
                Ok(post_body) => {
                    let response = fetch(FetchRequest::new(
                        "https://api.myfi.ws/Feedback".to_string(), 
                        FetchMethod::Post(post_body.to_string()))
                    );
                    true
                },
                Err(error) => {
                    jslog!("Failed to create feedback body: {:?}", error);
                    false
                }
            }
        },
        Err(error) => {
            jslog!("Failed to retrieve feedback: {:?}", error);
            false
        }
    }
}

pub(crate) fn get_render_wrapper() -> Html {
    html! {
        <GetRender />
    }
}

#[function_component(GetRender)]
pub(crate) fn get_render() -> Html {
    let feedback = use_state(|| GlobalData::get_data::<String>("feedback".to_string()).unwrap_or_default().to_string());
    let display = feedback.to_string();
    jslog!("Render Feedback");
    let onchange: fn(String) = |value| {
        jslog!("Feedback OnChange: {}", value);
        _ = GlobalData::set_data("feedback".to_string(), value);
    };
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
                <InputMessage name="Feedback" value={feedback} onchange={onchange} />

                <div>{"Test:"}{display}</div>
            </Paper>
        </>
    }
}

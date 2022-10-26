use std::collections::HashMap;

use webui::{web_sys::HtmlTextAreaElement, *};

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
            if value.is_empty() {
                return true;
            }
            let mut post_data = HashMap::from([("Message", value)]);
            match serde_json::to_string(&post_data) {
                Ok(post_body) => {
                    let result = async_std::task::block_on(async move {
                        let response = fetch(FetchRequest::new(
                            "https://api.myfi.ws/Feedback".to_string(),
                            FetchMethod::Post(post_body.to_string()),
                        ))
                        .await;
                        if response.is_ok() {
                            _ = GlobalData::set_data("feedback".to_string(), "");
                        }
                        response.is_ok()
                    });
                    true
                }
                Err(error) => {
                    jslog!("Failed to create feedback body: {:?}", error);
                    false
                }
            }
        }
        Err(_) => true,
    }
}

pub(crate) fn get_render_wrapper() -> Html {
    html! {
        <GetRender />
    }
}

#[function_component(GetRender)]
pub(crate) fn get_render() -> Html {
    let feedback = use_state(|| {
        GlobalData::get_data::<String>("feedback".to_string())
            .unwrap_or_default()
            .to_string()
    });
    let onchange = {
        let feedback = feedback.clone();
        Callback::from(move |ev: Event| match ev.target() {
            Some(target) => {
                let value = target.unchecked_into::<HtmlTextAreaElement>().value();
                feedback.set(value.to_string());
                _ = GlobalData::set_data("feedback".to_string(), value);
            }
            None => (),
        })
    };
    html! {
        <>
            <Paper class="pa-1 flex-grow d-flex flex-column gap-1">
                {paragraphs!(
                {html!{
                    <>
                        {"You can also come "}
                        <Link title="Web UI at Stoic Dreams Discord server" href="https://discord.com/channels/972856291909332993/1025781071608037466">{"chat with us on the Stoic Dreams discord server."}</Link>
                    </>
                }}
                )}
                <InputMessage class="flex-grow d-flex flex-column" name="Feedback" value={feedback.clone()} onchange={onchange} />
            </Paper>
        </>
    }
}

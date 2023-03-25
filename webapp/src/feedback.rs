use std::{collections::HashMap, rc::Rc};
use webui::{actors::input_state::use_input_state, web_sys::HtmlTextAreaElement, *};
use yew::functional::use_state;

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

const CONFIRM_KEY: &str = "feedback_confirm_result";
const FEEDBACK_KEY: &str = "feedback";

fn handle_confirm() -> bool {
    let input_state = use_input_state(FEEDBACK_KEY, || String::default(), Some(Rc::new(|| {})));
    log(format!(
        "Handle Confirmation Triggered:{}",
        input_state.to_string()
    ));
    let value = input_state.get();
    if value.is_empty() {
        return true;
    }
    let post_data = HashMap::from([("Message", value)]);
    match serde_json::to_string(&post_data) {
        Ok(post_body) => {
            log(format!("Spawning"));
            wasm_bindgen_futures::spawn_local(async move {
                let response = fetch(FetchRequest::new(
                    "https://feedback.myfi.ws/api/new".to_string(),
                    FetchMethod::Post(post_body.to_string()),
                ))
                .await;
                log(format!("Post Fetch:{}", response.is_ok()));
                _ = GlobalData::set_data(
                    FEEDBACK_KEY,
                    &format!("Response:{}\n{:?}", response.is_ok(), response),
                );
                if response.is_ok() {
                    _ = GlobalData::set_data(FEEDBACK_KEY, "");
                }
                let _ = GlobalData::set_data::<bool>(CONFIRM_KEY, response.is_ok());
            });
            // log(format!("Post Spawn"));
            // let result = GlobalData::get_data::<bool>(CONFIRM_KEY).unwrap_or(false);
            true
        }
        Err(error) => {
            jslog!("Failed to create feedback body: {:?}", error);
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
    // let value = GlobalData::get_data::<String>(FEEDBACK_KEY)
    // 	.unwrap_or_default()
    // 	.to_string();
    // let default_value = value.to_owned();
    // let feedback_handler = use_state(|| { default_value });
    // let test = use_state(||false);

    // log(format!("Render:{}", test.clone().to_string()));
    // let test2 = test.clone();
    // let boxx = Box::new(move ||{
    // 	test2.set(true);
    // });
    let count = use_state(|| 0);
    let rrr = Rc::new(move || {
        let increment = *count + 1;
        count.set(increment);
        log(format!("rrr:{}", *count));
    });
    let input_state = use_input_state(FEEDBACK_KEY, || String::default(), Some(rrr));
    let mut input_clone = input_state.clone();
    let callback = Closure::wrap(Box::new(move || {
        jslog!("Callback Test");
        input_clone.set(String::from("Hello"));
    }) as Box<dyn FnMut()>);
    // feedback_handler.set(value);
    let onchange = {
        // let input_state = input_state.clone();
        Callback::from(move |value: String| {
            // let value = target.unchecked_into::<HtmlTextAreaElement>().value();
            jslog!("Change:{}", value.to_string());
            _ = set_timeout(callback.as_ref().unchecked_ref(), 1);
            // input_state.set(value.to_string());
            // (rrr)();
            _ = GlobalData::set_data(FEEDBACK_KEY, value);
        })
    };
    log(format!("Render:{}", input_state.to_string()));
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
                <InputMessage class="flex-grow d-flex flex-column" key={FEEDBACK_KEY} name="Feedback" value={input_state.clone()} onchange={onchange} />
            </Paper>
        </>
    }
}

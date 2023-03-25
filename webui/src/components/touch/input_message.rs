use crate::actors::input_state::InputStateHandler;
use crate::web_sys::HtmlTextAreaElement;
use crate::*;

#[derive(Properties, PartialEq)]
pub struct InputMessageProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub key: String,
    // pub value: UseStateHandle<String>,
    pub value: InputStateHandler<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub cache_id: Option<String>,
    #[prop_or_default]
    pub onchange: Option<Callback<String>>,
}

#[function_component(InputMessage)]
pub fn input_message(props: &InputMessageProps) -> Html {
    let my_id = match &props.cache_id {
        Some(id) => id.to_string(),
        None => interop::get_uuid().to_owned(),
    };
    let classes = &mut Classes::new();
    classes.push("input-message");
    if !props.class.is_empty() {
        classes.push(&props.class);
    }
    let inputref = props.value.clone();
    let oninput = {
        Callback::from(move |ev: InputEvent| match ev.target() {
            Some(target) => {
                let value = target.unchecked_into::<HtmlTextAreaElement>().value();
                // inputref.set(value);
            }
            None => (),
        })
    };
    let changeref = props.value.clone();
    let onchange_handler = props.onchange.clone().unwrap_or(Callback::from(|_| ()));
    let key = props.key.to_owned();
    let onchange = {
        Callback::from(move |ev: Event| match ev.target() {
            Some(target) => {
                let value = target.unchecked_into::<HtmlTextAreaElement>().value();
                // if !key.is_empty() {
                // 	_ = GlobalData::set_data(&key, value.to_string());
                // }
                // changeref.set(value.to_string());
                onchange_handler.emit(value);
            }
            None => (),
        })
    };
    let placeholder = if props.placeholder.is_empty() {
        "Type text here".to_string()
    } else {
        props.placeholder.to_string()
    };
    jslog!("InputMessage Render:{}", props.value.to_string());
    html! {
        <InputField id={my_id.to_owned()}
            name={props.name.to_owned()}
            class={classes.to_string()}
            >
            <div class="auto_message_box">
                <pre><code>{props.value.to_string()}</code></pre>
                <textarea name={props.name.to_owned()}
                    id={my_id.to_owned()}
                    {oninput}
                    {onchange}
                    {placeholder}
                    >{props.value}</textarea>
            </div>
        </InputField>
    }
}

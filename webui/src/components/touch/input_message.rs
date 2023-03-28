use crate::web_sys::HtmlTextAreaElement;
use crate::*;

#[derive(Properties, PartialEq)]
pub struct InputMessageProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub key: String,
    pub value: UseStateHandle<String>,
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
    let count = (*props.value.clone()).len();
    let inputref = props.value.clone();
    let oninput = {
        Callback::from(move |ev: InputEvent| match ev.target() {
            Some(target) => {
                let value = target.unchecked_into::<HtmlTextAreaElement>().value();
                inputref.set(value);
            }
            None => (),
        })
    };
    let onchange_handler = props.onchange.clone().unwrap_or(Callback::from(|_| ()));
    let onchange = {
        Callback::from(move |ev: Event| match ev.target() {
            Some(target) => {
                let value = target.unchecked_into::<HtmlTextAreaElement>().value();
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
    let pre_value: String = props.value.clone().to_string();
    let ta_value: String = props.value.clone().to_string();
    html! {
        <InputField id={my_id.to_owned()}
            name={props.name.to_owned()}
            class={classes.to_string()}
            >
            <div class="auto_message_box">
                <pre><code>{pre_value}</code></pre>
                <textarea name={props.name.to_owned()}
                    id={my_id.to_owned()}
                    value={ta_value}
                    {oninput}
                    {onchange}
                    {placeholder}
                    />
            </div>
            <Paper class="d-flex flex-row">
                <Paper class="flex-grow" />
                <Paper>
                    {format!("Count: {}", count.to_formatted_string(&Locale::en))}
                </Paper>
            </Paper>
        </InputField>
    }
}

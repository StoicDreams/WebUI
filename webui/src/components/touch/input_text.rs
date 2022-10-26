use crate::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct InputTextProps {
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
    pub onchange: Option<Callback<Event>>,
}

#[function_component(InputText)]
pub fn input_message(props: &InputTextProps) -> Html {
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
                let value = target.unchecked_into::<HtmlInputElement>().value();
                inputref.set(value);
            }
            None => (),
        })
    };
    let changeref = props.value.clone();
    let onchange_handler = props.onchange.clone().unwrap_or(Callback::from(|_| ()));
    let onchange = {
        Callback::from(move |ev: Event| match ev.target() {
            Some(target) => {
                let value = target.unchecked_into::<HtmlInputElement>().value();
                changeref.set(value.to_string());
                onchange_handler.emit(ev.clone());
            }
            None => (),
        })
    };
    let placeholder = if props.placeholder.is_empty() {
        "Type text here".to_string()
    } else {
        props.placeholder.to_string()
    };
    html! {
        <InputField id={my_id.to_owned()}
            name={props.name.to_owned()}
            class={classes.to_string()}
            >
            <div class="auto_message_box single-line">
                <pre><code>{props.value.to_string()}</code></pre>
                <input type="text"
                    name={props.key.to_owned()}
                    id={my_id.to_owned()}
                    value={props.value.to_string()}
                    {oninput}
                    {onchange}
                    {placeholder}
                    />
            </div>
        </InputField>
    }
}

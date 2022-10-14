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
    pub field_class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub cache_id: Option<String>,
    #[prop_or_default]
    pub onchange: Option<fn(String)>,
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
        Callback::from(move |ev:InputEvent| {
            match ev.target() {
                Some(target) => {
                    let value = target.unchecked_into::<HtmlTextAreaElement>().value();
                    jslog!("input: {:?}", value);
                    inputref.set(value);
                    },
                None => ()
            }
        })
    };
    let changeref = props.value.clone();
    let onchange_handler = props.onchange.unwrap_or(|_:String|());
    let onchange = {
        Callback::from(move |ev:Event| {
            match ev.target() {
                Some(target) => {
                    let value = target.unchecked_into::<HtmlTextAreaElement>().value();
                    jslog!("change: {:?}", value);
                    changeref.set(value.to_string());
                    onchange_handler(value.to_string());
                },
                None => ()
            }
        })
    };
	html! {
        <InputField id={my_id.to_owned()}
            name={props.name.to_owned()}
            class={props.field_class.to_owned()}
            >
            <div class="auto_message_box">
                <pre><code>{props.value.to_string()}</code></pre>
                <textarea name={props.key.to_owned()}
                    id={my_id.to_owned()}
                    oninput={oninput}
                    onchange={onchange}
                    >{props.value}</textarea>
            </div>
        </InputField>
	}
}


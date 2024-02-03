use crate::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct InputTextProps {
    #[prop_or_default]
    pub t: String,
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
    pub readonly: bool,
    #[prop_or_default]
    pub onchange: Option<Callback<String>>,
    #[prop_or_default]
    pub start_icon: Option<IconOptions>,
    #[prop_or_default]
    pub end_icon: Option<IconOptions>,
    #[prop_or_default]
    pub start_button: Option<ButtonIconInfo>,
    #[prop_or_default]
    pub end_button: Option<ButtonIconInfo>,
}

#[function_component(InputText)]
pub fn input_message(props: &InputTextProps) -> Html {
    let my_id = match &props.cache_id {
        Some(id) => id.to_string(),
        None => interop::get_uuid(),
    };
    let input_type = if props.t.is_empty() {
        "text".to_string()
    } else {
        props.t.to_string()
    };
    let classes = &mut Classes::new();
    classes.push("input-message");
    if !props.class.is_empty() {
        classes.push(&props.class);
    }
    let inputref = props.value.clone();
    let oninput = {
        Callback::from(move |ev: InputEvent| {
            if let Some(target) = ev.target() {
                let value = target.unchecked_into::<HtmlInputElement>().value();
                inputref.set(value);
            }
        })
    };
    let onchange_handler = props.onchange.clone().unwrap_or(Callback::from(|_| ()));
    let onchange = {
        Callback::from(move |ev: Event| {
            if let Some(target) = ev.target() {
                let value = target.unchecked_into::<HtmlInputElement>().value();
                onchange_handler.emit(value);
            }
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
            <div class="d-flex flex-row gap-0 field-group-line">
                {if let Some(start_icon) = &props.start_icon {
                    html!{<Icon
                            icon={start_icon.icon.to_owned()}
                            title={start_icon.title.to_owned()}
                            elevation={start_icon.elevation.to_owned()}
                            color={start_icon.color.to_owned()}
                            style={start_icon.style.to_owned()}
                            />}
                } else { html!() }}
                {if let Some(start_button) = &props.start_button {
                    html!{<ButtonIcon
                            icon={start_button.icon.to_owned()}
                            onclick={start_button.onclick.to_owned()}
                            elevation={start_button.elevation.to_owned()}
                            color={start_button.color.to_owned()}
                            class={start_button.class.to_owned()}
                            title={start_button.title.to_owned()}
                            style={start_button.style.to_owned()}
                            disabled={start_button.disabled.to_owned()}
                            />}
                } else { html!() }}
                <div class="auto_message_box single-line">
                    <pre><code class="language-plaintext">{props.value.to_string()}</code></pre>
                    <input type={input_type}
                        name={props.key.to_owned()}
                        id={my_id.to_owned()}
                        value={props.value.to_string()}
                        readonly={props.readonly}
                        {oninput}
                        {onchange}
                        {placeholder}
                        />
                </div>
                {if let Some(end_icon) = &props.end_icon {
                    html!{<Icon
                            icon={end_icon.icon.to_owned()}
                            title={end_icon.title.to_owned()}
                            elevation={end_icon.elevation.to_owned()}
                            color={end_icon.color.to_owned()}
                            style={end_icon.style.to_owned()}
                            />}
                } else { html!() }}
                {if let Some(end_button) = &props.end_button {
                    html!{<ButtonIcon
                            icon={end_button.icon.to_owned()}
                            onclick={end_button.onclick.to_owned()}
                            elevation={end_button.elevation.to_owned()}
                            color={end_button.color.to_owned()}
                            class={end_button.class.to_owned()}
                            title={end_button.title.to_owned()}
                            style={end_button.style.to_owned()}
                            disabled={end_button.disabled.to_owned()}
                             />}
                } else { html!() }}
            </div>
        </InputField>
    }
}

use crate::*;

#[derive(Properties, PartialEq)]
pub struct InputTextProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub key: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub field_class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub cache_id: Option<String>,
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
    let value = use_state(|| props.value.to_string());
	html! {
        <InputField id={my_id.to_owned()}
            name={props.name.to_owned()}
            class={props.field_class.to_owned()}
            >
            <div class="auto_message_box">
                <code>{value.to_string()}</code>
                <input type="text"
                    name={props.key.to_owned()}
                    id={my_id.to_owned()}
                    value={value.to_string()}
                    />
            </div>
        </InputField>
	}
}


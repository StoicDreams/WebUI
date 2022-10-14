use crate::*;

#[derive(Properties, PartialEq)]
pub struct InputFieldProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(InputField)]
pub fn input_message(props: &InputFieldProps) -> Html {
    let my_id = match &props.id {
        Some(id) => id.to_string(),
        None => interop::get_uuid().to_owned(),
    };
    let classes = &mut Classes::new();
    classes.push("input-field");
    if !props.class.is_empty() {
        classes.push(&props.class);
    }
    html! {
        <div class={classes.to_owned()}>
            <label for={my_id}>{props.name.to_string()}</label>
            {for props.children.iter()}
        </div>
    }
}

use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DynamicComponentProps {
    pub name: String,
}

#[function_component(DynamicComponent)]
pub fn dynamic_component(props: &DynamicComponentProps) -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    if let Some(registry) = contexts.config.component_registry.to_owned() {
        if let Some(component) = registry.get(&props.name) {
            return component(contexts);
        }
    }
    html!(
        <Paper>{format!("Component <{} /> not found!", props.name)}</Paper>
    )
}

use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct DynamicComponentProps {
    pub name: String,
}

/// Used for dynamically loading components declared within Markdown content/files.
///
/// Components must be functions that are passed a single Contexts parameter and return yew Html.
/// Components must also be registered from the AppConfigBuilder.register_component method.
///
/// Markdown Example:
///
/// ```Markdown
/// !<Test>
/// ```
/// Example:
///
/// ```Rust,no_run
/// pub fn render_test(_contexts: Contexts) -> Html {
///     html! {
///         <Test />
///     }
/// }
///
/// fn setup_app_config() -> AppConfig {
///     AppConfig::builder(
///         "Web UI Demo & Documentation".to_owned(),
///         "Stoic Dreams".to_owned(),
///         "https://www.stoicdreams.com".to_owned(),
///         "WebUI.StoicDreams.com".to_owned(),
///     )
///     .set_header_logo_src("Logo.svg".to_owned())
///     .set_nav_routing(nav_menu::get_nav_routing())
///     .set_drawer_toggle_header_left(nav_menu::nav_menu_info())
///     .set_drawer_toggle_header_middle(myfi_feedback_button_info())
///     .set_header_strip_bar(stoic_header_strip_bar)
///     .set_user_info_panel(myfi_info_panel)
///     .set_copyright_start(2022)
///     .register_component("Test", render_test)
///     .build()
/// }
/// ```
///
#[function_component(DynamicComponent)]
pub(crate) fn dynamic_component(props: &DynamicComponentProps) -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    if let Some(registry) = contexts.config.component_registry.to_owned() {
        if let Some(component) = registry.get(&props.name) {
            return component(contexts);
        }
    }
    html!(
        <Paper>{format!("Component <{}> not found!", props.name)}</Paper>
    )
}

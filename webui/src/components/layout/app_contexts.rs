use crate::{function_component, html, AppConfig, Children, ContextProvider, Properties};

/// Properties for app drawer components
#[derive(Properties, PartialEq)]
pub(crate) struct AppContextsProps {
    pub children: Children,
    pub app_config: AppConfig,
}

/// App drawer component
#[function_component(AppContexts)]
pub(crate) fn app_drawer(props: &AppContextsProps) -> Html {
    html! {
        <ContextProvider<AppConfig> context={props.app_config.clone()}>
            { for props.children.iter() }
        </ContextProvider<AppConfig>>
    }
}

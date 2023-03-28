use crate::*;

/// Properties for app drawer components
#[derive(Properties, PartialEq)]
pub(crate) struct AppContextsProps {
    pub children: Children,
    pub app_config: AppConfig,
}

#[function_component(AppContexts)]
pub(crate) fn app_contexts(props: &AppContextsProps) -> Html {
    let app_config = &props.app_config;
    let navigation = use_state(|| {
        let path = interop::get_path().to_lowercase();
        NavigationMessage::PathUpdate(path)
    });
    let drawers = use_state(|| DrawerMessage::None);
    html! {
        <ContextProvider<AppConfig> context={app_config.clone()}>
        <ContextProvider<UseStateHandle<NavigationMessage>> context={navigation.clone()}>
        <ContextProvider<UseStateHandle<DrawerMessage>> context={drawers.clone()}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<DrawerMessage>>>
        </ContextProvider<UseStateHandle<NavigationMessage>>>
        </ContextProvider<AppConfig>>
    }
}

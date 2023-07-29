use crate::prelude::*;

/// Properties for app drawer components
#[derive(Properties, PartialEq)]
pub(crate) struct AppContextsProps {
    pub children: Children,
    pub config: AppConfig,
}

#[function_component(AppContexts)]
pub(crate) fn app_contexts(props: &AppContextsProps) -> Html {
    let app_config = &props.config;
    let navigation = use_state(|| {
        let path = interop::get_path().to_lowercase();
        NavigationMessage::PathUpdate(path)
    });
    let data = use_state(|| None::<String>);
    let drawers = use_state(|| DrawerMessage::None);
    let context = Contexts {
        config: app_config.clone(),
        page_loaded: use_state(|| "".to_string()),
        data,
        nav: navigation,
        drawer: drawers,
    };
    html! {
        <ContextProvider<Contexts> {context}>
            { for props.children.iter() }
        </ContextProvider<Contexts>>
    }
}

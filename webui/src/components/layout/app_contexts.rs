use std::sync::Arc;

use crate::prelude::*;

/// Properties for app drawer components
#[derive(Properties, PartialEq)]
pub(crate) struct AppContextsProps {
    pub children: Children,
    pub config: AppConfig,
    pub state_keys: Vec<String>,
}

#[function_component(AppContexts)]
pub(crate) fn app_contexts(props: &AppContextsProps) -> Html {
    let nav = use_state(|| {
        let path = interop::get_path().to_lowercase();
        NavigationMessage::PathUpdate(path)
    });
    let context = Contexts {
        config: props.config.clone(),
        page_loaded: use_state(|| "".to_string()),
        data: use_state(|| None::<String>),
        nav,
        drawer: use_state(|| DrawerMessage::None),
        user_roles: use_state(|| 0),
        #[cfg(feature = "myfi")]
        user: use_state(|| None::<MyFiUser>),
    };
    html! {
        <ContextProvider<Contexts> {context}>
            { for props.children.iter() }
        </ContextProvider<Contexts>>
    }
}

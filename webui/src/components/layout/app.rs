use super::app_body::AppBody;
use super::app_contexts::*;
use crate::data_types::app_config::AppConfig;
use crate::prelude::*;

/// Properties for AppConfig component
#[derive(Properties, PartialEq)]
pub(crate) struct AppProps {
    pub config: AppConfig,
    pub state_keys: Vec<String>,
}

/// Inner process for starting application/website
pub(crate) fn start_webui_app(app_config: AppConfig, state_keys: Vec<String>) {
    let props = AppProps {
        config: app_config,
        state_keys,
    };
    yew::Renderer::<App>::with_props(props).render();
}
struct App {}

impl Component for App {
    type Message = ();
    type Properties = AppProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {<AppRender config={props.config.to_owned()} state_keys={props.state_keys.to_owned()} />}
    }
}

#[function_component(AppRender)]
fn app_render(props: &AppProps) -> Html {
    let nav: UseStateHandle<NavigationMessage> = use_state(|| {
        let path = interop::get_path().to_lowercase();
        NavigationMessage::PathUpdate(path)
    });
    let mut contexts = Contexts {
        config: props.config.clone(),
        page_loaded: use_state(String::default),
        page_data: use_state(String::default),
        app_data: use_state(HashMap::new),
        nav,
        user_roles: use_state(|| 0),
        #[cfg(feature = "myfi")]
        user: use_state(|| None::<MyFiUser>),
    };
    if let Some(key_list) = &props.config.app_data_keys {
        for key in key_list {
            //contexts.init_data_handler(&key, use_state(String::default));
        }
    };
    let nav = &props.config.nav;
    html! {
        <ContextProvider<Contexts> context={contexts.to_owned()}>
            <crate::loaders::Loaders />
            <webui-app class="page transition out" data-removeclass=".nav|open;.shared|open">
                {if let Some(nav) = &nav {
                    html!{
                        <webui-drawer slot="left" class="nav elevation-20" data-state="slot|fixed" data-moveable="true" data-dockable="true">
                            {nav(&contexts)}
                        </webui-drawer>
                    }
                } else {html!()}}
                <webui-drawer slot="right" class="shared elevation-20" data-stopclick="true" data-moveable="true" data-state="slot|fixed" fixed="true"></webui-drawer>
                <AppHeader />
                <AppBody />
                <AppFooter />
            </webui-app>
        </ContextProvider<Contexts>>
    }
}

#[function_component(AppHeader)]
fn app_header() -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let app_config = contexts.clone().config;
    let nav = &app_config.nav;
    if let Some(header) = app_config.header {
        return html! {
            <header slot="header">
                {if nav.is_some() {
                    html!{
                        <button data-toggleclass=".nav|open" class="elevation-10 pa-1 mx-1">
                            <webui-fa icon="bars"></webui-fa>
                        </button>
                    }
                } else {html!()}}
                {header(&contexts)}
            </header>
        };
    }
    html! {}
}

#[function_component(AppFooter)]
fn app_footer() -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let app_config = contexts.clone().config;
    if let Some(footer) = app_config.footer {
        return html! {
            <footer slot="footer">
                {footer(&contexts)}
                <webui-flex grow="true"></webui-flex>
                if !app_config.hide_powered_by {
                    <webui-poweredby version={crate::VERSION}></webui-poweredby>
                }
            </footer>
        };
    }
    html! {}
}

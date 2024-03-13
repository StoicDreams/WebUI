use super::app_body::AppBody;
use super::app_contexts::*;
use super::app_drawer::AppDrawer;
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
        page_loaded: use_state(|| "".to_string()),
        app_data: HashMap::new(),
        nav,
        drawer: use_state(|| DrawerMessage::None),
        user_roles: use_state(|| 0),
        #[cfg(feature = "myfi")]
        user: use_state(|| None::<MyFiUser>),
    };
    let app_loader = match props.config.app_loader {
        Some(app_loader) => app_loader,
        None => default_app_loader,
    };

    contexts.init_data_handler("page_data", use_state(|| String::default()));
    html! {
        <div id="app" class="page transition out">
            <ContextProvider<Contexts> context={contexts.to_owned()}>
                <crate::loaders::Loaders />
                {app_loader()}
                <AppHeader />
                <AppBody />
                <AppFooter />
                <AppDrawer drawer={Direction::Top} />
                <AppDrawer drawer={Direction::Right} />
                <AppDrawer drawer={Direction::Bottom} />
                <AppDrawer drawer={Direction::Left} pinnable={PinOptions::PinnableWithThinOption} />
            </ContextProvider<Contexts>>
        </div>
    }
}

fn default_app_loader() -> Html {
    html!()
}

#[function_component(AppHeader)]
fn app_header() -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let app_config = contexts.clone().config;
    if let Some(header) = app_config.header {
        return html! {header(contexts.to_owned())};
    }
    html! {}
}

#[function_component(AppFooter)]
fn app_footer() -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let app_config = contexts.clone().config;
    if let Some(footer) = app_config.footer {
        return html! {footer(contexts.to_owned())};
    }
    html! {}
}

use crate::prelude::*;
use yew::{use_state, UseStateHandle};

/// Properties for NavLink component
#[derive(Debug, Properties, PartialEq)]
pub struct AppDrawerButtonProps {
    pub info: Option<DrawerToggleInfo>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub logosrc: Option<String>,
    #[prop_or_default]
    pub logotitle: Option<String>,
    #[prop_or_default]
    pub always_show_logo: bool,
    #[prop_or(1)]
    pub elevation: u8,
}

/// Button that is used to trigger opening one of the four app drawers.
///
/// Left and right app drawers are side panels that pop out with a width dependent on their content.
/// Top and Bottom app drawers act more like dialogs|modals, sliding out and displaying in the center of the page.
#[function_component(AppDrawerButton)]
pub fn app_drawer_button(props: &AppDrawerButtonProps) -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let title_context = contexts.clone();
    let logo_src_handle: UseStateHandle<Option<String>> = use_state(|| None);
    let logo_title_handle: UseStateHandle<String> = use_state(String::default);
    let is_setup = use_state(|| false);
    let drawer_info = &props.info;
    let logo_src = logo_src_handle.deref().to_owned();
    let logo_title = logo_title_handle.deref().to_owned();
    let drawer_info_click = drawer_info.clone();
    if let Some(info) = &drawer_info_click {
        if !*is_setup {
            let options = info.get_options();
            contexts.drawer.set(DrawerMessage::Setup(options));
            is_setup.set(true);
        }
    }
    let contexts_onclick = contexts.clone();
    let setup_onclick = Callback::from(move |_| {
        let drawer_info_click = drawer_info_click.clone();
        if let Some(info) = drawer_info_click {
            let options = info.get_options();
            contexts_onclick
                .drawer
                .set(DrawerMessage::ToggleDrawer(options));
        };
    });
    let children = &props.children;
    let classes = &mut Classes::new();

    if !props.class.is_empty() {
        classes.push(&props.class);
    } else {
        classes.push("btn");
    }
    if let Some(drawer_info) = drawer_info {
        classes.push(&format!("drawer-toggle-{}", drawer_info.drawer));
    }
    let elevation = props.elevation;
    html! {
        <>
            {match drawer_info.clone() {
                Some(drawer_info) => {
                    let title = (drawer_info.title)(title_context);
                    let btn_class = if drawer_info.class.is_empty() {"toggle".to_string()} else {drawer_info.class.to_string()};
                    html! {
                        <Button title={title.to_owned()} class={classes.to_string()}
                            elevation={elevation}
                            onclick={setup_onclick}>
                            <span class={btn_class}>{(drawer_info.display)(contexts.clone())}</span>
                            {match &logo_src {
                                Some(logo) => {
                                    html! {
                                        <img class="pl-1" src={logo.to_string()} title={logo_title.to_owned()} />
                                    }
                                },
                                None => html! {}
                            }}
                            {for children.iter()}
                        </Button>
                    }
                },
                None => html! {
                    if props.always_show_logo {
                        {match &logo_src {
                            Some(logo) => {
                                html! {
                                    <Paper>
                                        <img src={logo.to_string()} title={logo_title.to_owned()} />
                                    </Paper>
                                }
                            },
                            None => html! {}
                        }}
                    }
                }
            }}
        </>
    }
}

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum PageState {
    Hidden,
    TransitionIn,
    TransitionOut,
    Show,
}

#[function_component(AppBody)]
pub(crate) fn app_body() -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let page_state = use_state(|| PageState::Show);
    let nav = contexts.nav.deref().to_owned();
    let path = use_state(|| interop::get_path().to_lowercase());
    let routes = contexts.config.nav_routing.clone();
    let user_roles = contexts.user_roles.deref().to_owned();
    if let NavigationMessage::PathUpdate(new_path) = nav {
        if *path.deref() != new_path {
            contexts.nav.set(NavigationMessage::None);
            contexts.clear_app_data("page_data");
            let page_path = get_path_from_url(&new_path);
            let page_check = get_page_option(&routes, &page_path, &user_roles);
            if page_check.is_some() && *page_state.deref() == PageState::Show {
                contexts.drawer.set(DrawerMessage::Close);
                let page_state = page_state.clone();
                load_page_data(&path, contexts.clone());
                let path_timeout = path.clone();
                set_timeout!(1, {
                    let page_state_out = page_state.clone();
                    let path = path_timeout.clone();
                    let new_path = new_path.clone();
                    page_state_out.set(PageState::TransitionOut);
                    set_timeout!(300, {
                        push_state(&new_path);
                        let page_state_hidden = page_state_out.clone();
                        let path = path.clone();
                        let new_path = new_path.clone();
                        page_state_hidden.set(PageState::Hidden);
                        path.set(new_path);
                        // TODO: This set timeout is where the page freeze is happening, need to figure out why.
                        set_timeout!(100, {
                            let page_state_in = page_state_hidden.to_owned();
                            page_state_in.set(PageState::TransitionIn);
                            set_timeout!(300, {
                                let page_state_show = page_state_in.clone();
                                page_state_show.set(PageState::Show);
                            });
                        });
                    });
                });
            }
        }
    }
    let path = get_path_from_url(&path);
    let page_el = format!("page{}", path.replace('-', "_").replace('/', "__"));
    let main_class = match page_state.deref() {
        PageState::Hidden => "page hidden",
        PageState::TransitionIn => "page transition in",
        PageState::TransitionOut => "page transition out",
        PageState::Show => "",
    };

    load_page_data(&path, contexts.clone());

    let page = path.to_string();
    html! {
        <>
            <main class={main_class}>
                <@{page_el} class="paper">
                    <PageContent {routes} {page} />
                </@>
            </main>
            <Paper id="loading" class="d-flex align-center justify-center">
                <Loading variant={LoadingVariant::Circle} color={Theme::Secondary} size={LOADING_SIZE_LARGE} />
            </Paper>
        </>
    }
}

#[cfg(not(feature = "myfi"))]
fn load_page_data(_path: &str, _contexts: Contexts) {}

#[cfg(feature = "myfi")]
fn load_page_data(path: &str, contexts: Contexts) {
    if let Some(data) = contexts.get_data_handler("page_data") {
        let path = path.to_owned();
        let last_fetched = contexts.page_loaded.deref().as_str();
        if last_fetched == path {
            return;
        }
        contexts.page_loaded.set(path.clone());
        spawn_async!({
            let fetched = get_myfi_page_data(&path).await;
            data.set(fetched);
        });
    }
}

#[derive(Properties, PartialEq)]
struct PageNotFoundProps {
    path: String,
}

fn page_not_found(path: &str) -> Html {
    html! {
        <MarkdownContent markdown={format!(r#"
            `````sideimage "right" "https://cdn.myfi.ws/v/Vecteezy/404-error-illustration-exclusive-design-inspiration.svg"
            ````paper "d-flex flex-column align-center"
            ```paper "f7"
            **`{}` Not Found:**
            ```
            The page you are looking for could not be found!
            ````
            `````
            "#, path)} />
    }
}

#[derive(Properties, PartialEq)]
pub struct PageContentProps {
    pub routes: Vec<NavRoute>,
    pub page: String,
}

#[function_component(PageContent)]
fn page_content(props: &PageContentProps) -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let user_roles = contexts.user_roles.deref().to_owned();
    match use_get_page(&props.routes, &props.page, &user_roles) {
        yew::suspense::SuspensionResult::Ok(link_info) => {
            let page = link_info.page;
            html! {<>{page(contexts)}</>}
        }
        yew::suspense::SuspensionResult::Err(_err) => {
            jslog!("Get page failed!");
            match contexts.config.page_not_found {
                Some(page) => html! {page(&props.page)},
                None => html! {page_not_found(&props.page)},
            }
        }
    }
}

#[hook]
fn use_get_page(
    routes: &[NavRoute],
    page: &str,
    user_roles: &u32,
) -> yew::suspense::SuspensionResult<NavLinkInfo> {
    match get_page_option(routes, page, user_roles) {
        Some(info) => Ok(info),
        None => {
            let (s, handle) = yew::suspense::Suspension::new();
            get_page_failure(move || {
                handle.resume();
            });
            Err(s)
        }
    }
}

fn get_page_failure<F: FnOnce()>(handler: F) {
    handler();
}

fn get_page_option(routes: &[NavRoute], page: &str, user_roles: &u32) -> Option<NavLinkInfo> {
    for route in routes {
        match route {
            NavRoute::NavLink(link_info) => {
                if link_info.path.to_lowercase() == page.to_lowercase()
                    && (link_info.role == 0 || link_info.role & user_roles != 0)
                {
                    return Option::Some(link_info.to_owned());
                }
            }
            NavRoute::NavGroup(group_info) => {
                if group_info.children.is_empty() {
                    continue;
                }
                if let Option::Some(link_info) =
                    get_page_option(&group_info.children, page, user_roles)
                {
                    return Option::Some(link_info);
                }
            }
        }
    }
    Option::None
}

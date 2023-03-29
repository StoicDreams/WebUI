use yew::suspense::SuspensionResult;

use crate::*;

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
    let _ = match nav {
        NavigationMessage::PathUpdate(new_path) => {
            if path.deref().to_owned() != new_path {
                contexts.nav.set(NavigationMessage::None);
                if *page_state.deref() == PageState::Show {
                    let page_state = page_state.clone();
                    let path = path.clone();
                    set_timeout!(1, move || {
                        let page_state = page_state.clone();
                        let path = path.clone();
                        let new_path = new_path.clone();
                        page_state.set(PageState::TransitionOut);
                        set_timeout!(300, move || {
                            let page_state = page_state.clone();
                            let path = path.clone();
                            let new_path = new_path.clone();
                            page_state.set(PageState::Hidden);
                            path.set(String::from(new_path));
                            set_timeout!(100, move || {
                                let page_state = page_state.clone();
                                page_state.set(PageState::TransitionIn);
                                set_timeout!(300, move || {
                                    let page_state = page_state.clone();
                                    page_state.set(PageState::Show);
                                });
                            });
                        });
                    });
                }
            }
        }
        _ => (),
    };
    let page_el = format!("page{}", path.replace("-", "_").replace("/", "__"));
    let main_class = match page_state.deref() {
        PageState::Hidden => "page hidden",
        PageState::TransitionIn => "page transition in",
        PageState::TransitionOut => "page transition out",
        PageState::Show => "",
    };

    let routes = contexts.config.nav_routing;
    let page = path.deref().to_string();
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

#[function_component(PageNotFound)]
fn page_not_found() -> Html {
    html! {
        <SideImage image_pos={LeftOrRight::Right} src="https://cdn.myfi.ws/v/Vecteezy/404-error-illustration-exclusive-design-inspiration.svg">
            {paragraphs!(
                "The page you are looking for could not be found."
            )}
        </SideImage>
    }
}

#[derive(Properties, PartialEq)]
pub struct PageContentProps {
    pub routes: Vec<NavRoute>,
    pub page: String,
}

#[function_component(PageContent)]
fn page_content(props: &PageContentProps) -> Html {
    match use_get_page(props.routes.to_owned(), &props.page) {
        SuspensionResult::Ok(link_info) => {
            let page = link_info.page;
            html! {<>{page()}</>}
        }
        SuspensionResult::Err(_err) => {
            jslog!("Get page failed!");
            html! {<PageNotFound />}
        }
    }
}

#[hook]
fn use_get_page(routes: Vec<NavRoute>, page: &str) -> suspense::SuspensionResult<NavLinkInfo> {
    match get_page_option(routes, page) {
        Some(info) => Ok(info),
        None => {
            let (s, handle) = suspense::Suspension::new();
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

fn get_page_option(routes: Vec<NavRoute>, page: &str) -> Option<NavLinkInfo> {
    for route in routes {
        match route {
            NavRoute::NavLink(link_info) => {
                if link_info.path.to_lowercase() == page.to_lowercase() {
                    return Option::Some(link_info);
                }
            }
            NavRoute::NavGroup(group_info) => {
                if group_info.children.len() == 0 {
                    continue;
                }
                if let Option::Some(link_info) = get_page_option(group_info.children, page) {
                    return Option::Some(link_info);
                }
            }
        }
    }
    Option::None
}

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
    let navigation = use_context::<UseStateHandle<NavigationMessage>>()
        .expect("Context NavigationMessage not found");
    let app_config = use_context::<AppConfig>().expect("Context AppConfig not found");
    let page_state = use_state(|| PageState::Show);
    let nav = navigation.deref().to_owned();
    let path = use_state(|| interop::get_path().to_lowercase());
    let _ = match nav {
        NavigationMessage::PathUpdate(new_path) => {
            if path.deref().to_owned() != new_path {
                navigation.set(NavigationMessage::None);
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
    jslog!("body path:{}", page_el);
    html! {
        <>
            <main class={main_class}>
                <@{page_el} class="paper">
                    {(get_page_content(app_config.nav_routing, &path))()}
                </@>
            </main>
            <Paper id="loading" class="d-flex align-center justify-center">
                <Loading variant={LoadingVariant::Circle} color={Theme::Secondary} size={LOADING_SIZE_LARGE} />
            </Paper>
        </>
    }
}

fn page_not_found() -> Html {
    html! {
        <SideImage image_pos={LeftOrRight::Right} src="https://cdn.myfi.ws/v/Vecteezy/404-error-illustration-exclusive-design-inspiration.svg">
            {paragraphs!(
                "The page you are looking for could not be found."
            )}
        </SideImage>
    }
}

fn get_page_content(routes: Vec<NavRoute>, page: &str) -> fn() -> Html {
    let page = get_page(routes, page);
    match page {
        Option::Some(link_info) => link_info.page,
        Option::None => page_not_found,
    }
}

fn get_page(routes: Vec<NavRoute>, page: &str) -> Option<NavLinkInfo> {
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
                if let Option::Some(link_info) = get_page(group_info.children, page) {
                    return Option::Some(link_info);
                }
            }
        }
    }
    Option::None
}

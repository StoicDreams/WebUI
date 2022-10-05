use crate::pages::{about::PageAbout, home::PageHome};
use yew::{html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

pub fn body_html() -> Html {
    html! {
        <BrowserRouter>
            <Switch::<Route> render={Switch::render(page_routes)} />
        </BrowserRouter>
    }
}

/// Page route handling
pub fn page_routes(route: &Route) -> Html {
    match route {
        Route::Home => html! { <PageHome />},
        Route::About => html! { <PageAbout />},
    }
}

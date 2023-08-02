use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NextPageProps {
    pub display: Option<String>,
    pub url: String,
}

#[function_component(NextPageButton)]
pub fn next_page_button(_props: &NextPageProps) -> Html {
    #[cfg(not(feature = "nextpagebutton"))]
    {
        return html! {};
    }
    #[cfg(feature = "nextpagebutton")]
    {
        let contexts = use_context::<Contexts>().expect("Contexts not found");
        let app_config = contexts.config;
        let display = match _props.display.to_owned() {
            Some(display) => display,
            None => match app_config.get_nav_from_path(&_props.url) {
                Some(nav) => nav.name,
                None => "Not Found".to_string(),
            },
        };
        let href = _props.url.to_owned();
        html!(
            <>
                <Paper class="flex-grow" />
                <Paper class="d-flex flex-column align-center justify-center ma-5 pa-5">
                    <Link class="btn theme-info" {href}>
                        <span>{format!("Continue to {}", display)}</span>
                        <Avatar class="d-inline ml-2" icon="fa-duotone fa-right" />
                    </Link>
                </Paper>
            </>
        )
    }
}

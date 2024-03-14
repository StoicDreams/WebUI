use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NextPageProps {
    pub display: Option<String>,
    pub url: String,
    #[prop_or(true)]
    pub snap_bottom: bool,
}

#[function_component(NextPageButton)]
pub fn next_page_button(props: &NextPageProps) -> Html {
    #[cfg(not(feature = "nextpagebutton"))]
    {
        return html! {};
    }
    #[cfg(feature = "nextpagebutton")]
    {
        let contexts = use_context::<Contexts>().expect("Contexts not found");
        let app_config = contexts.config.clone();
        let display = match props.display.to_owned() {
            Some(display) => display,
            None => match app_config.get_nav_from_path(&props.url, &contexts) {
                Some(nav) => nav.name,
                None => "Not Found".to_string(),
            },
        };
        let href = props.url.to_owned();
        html!(
            <>
                if props.snap_bottom {
                    <Paper class="flex-grow" />
                }
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

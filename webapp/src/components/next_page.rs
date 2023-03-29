use crate::*;

#[derive(Properties, PartialEq)]
pub(crate) struct NextPageProps {
    pub display: Option<String>,
    pub url: String,
}

#[function_component(NextPage)]
pub(crate) fn next_page(props: &NextPageProps) -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let display = match props.display.to_owned() {
        Some(display) => display,
        None => match contexts.config.get_nav_from_path(&props.url) {
            Some(nav) => nav.name,
            None => "Not Found".to_string(),
        },
    };
    let href = props.url.to_owned();
    html!(
        <>
            <Paper class="d-flex flex-column align-center justify-center mt-5 pt-5 pb-5">
                <Link class="btn theme-info" {href}>
                    <span>{format!("Continue to {}", display)}</span>
                    <Avatar class="d-inline ml-2" icon="fa-duotone fa-right" />
                </Link>
            </Paper>
            <Paper class="d-flex flex-column align-center justify-center mt-5">
                <JasperLink />
            </Paper>
        </>
    )
}

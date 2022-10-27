use webui::*;

/// Properties for Paper component
#[derive(Properties, PartialEq)]
pub struct JasperLinkProps {
    #[prop_or_default]
    pub display: Option<String>,
}

#[function_component(JasperLink)]
pub fn jasper_link(props: &JasperLinkProps) -> Html {
    html!(
        <>
            <Paper class="ma-3" />
            <Paper class="mt-a mb-5">
                <Quote class="pa-3" color={Theme::Success} elevation={ELEVATION_STANDARD}>
                    {match props.display.to_owned() {
                        Some(display) => html!(display),
                        None => html!({"Content on this page was created with the help of Jasper.ai."})
                    }}
                    {" "}
                    <Link href="https://jasper.ai/unlimited?fpr=stoicdreams">
                        {"Visit Jasper.ai to learn more about this awesome AI tool."}
                    </Link>
                </Quote>
            </Paper>
        </>
    )
}

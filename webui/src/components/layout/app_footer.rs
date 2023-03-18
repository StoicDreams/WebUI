use crate::{function_component, html, use_context, AppConfig, AppDrawerButton, Paper};

/// App footer component
#[function_component(AppFooter)]
pub(crate) fn app_footer() -> Html {
    let app_config = use_context::<AppConfig>().expect("no app config found");
    let left_drawer_info = app_config.footer_left_drawer_toggle.clone();
    let bottom_drawer_info = app_config.footer_bottom_drawer_toggle.clone();
    let right_drawer_info = app_config.footer_right_drawer_toggle.clone();

    html! {
        <footer>
            <AppDrawerButton info={left_drawer_info.clone()} />
            <Paper class="flex-grow" />
            <Paper>
                {format!("© {} {} All Rights Reserved", "2023", app_config.company_name)}
            </Paper>
            <AppDrawerButton info={bottom_drawer_info.clone()} />
            <Paper class="flex-grow" />
            if !app_config.hide_powered_by {
                <Paper>
                    <sup>{"Powered by "}</sup>
                    <a title="Web UI version 0.3.25" href="https://webui.stoicdreams.com">{"Web UI"}</a>
                </Paper>
            }
            <AppDrawerButton info={right_drawer_info.clone()} />
        </footer>
    }
}

use chrono::{Datelike, Utc};
use rust_decimal::prelude::ToPrimitive;

use crate::{function_component, html, use_context, AppDrawerButton, Contexts, Html, Paper};

/// App footer component
#[function_component(AppFooter)]
pub(crate) fn app_footer() -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let app_config = contexts.config;
    let left_drawer_info = app_config.footer_left_drawer_toggle.clone();
    let bottom_drawer_info = app_config.footer_bottom_drawer_toggle.clone();
    let right_drawer_info = app_config.footer_right_drawer_toggle.clone();
    let copy_end = Utc::now().year().to_i16().unwrap_or(2022);
    let copy_start = app_config.copyright_year_start.unwrap_or(copy_end);
    let copy_display = if copy_start == copy_end {
        copy_start.to_string()
    } else {
        format!("{}-{}", copy_start, copy_end)
    };
    html! {
        <footer>
            <AppDrawerButton info={left_drawer_info.clone()} />
            <Paper class="flex-grow" />
            <Paper>
                {format!("© {} {} All Rights Reserved", copy_display, app_config.company_name)}
            </Paper>
            <AppDrawerButton info={bottom_drawer_info.clone()} />
            <Paper class="flex-grow" />
            if !app_config.hide_powered_by {
                <Paper>
                    <sup>{"Powered by "}</sup>
                    <a title="Web UI version 0.6.13" href="https://webui.stoicdreams.com">{"Web UI"}</a>
                </Paper>
            }
            <AppDrawerButton info={right_drawer_info.clone()} />
        </footer>
    }
}

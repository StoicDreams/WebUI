use chrono::{Datelike, Utc};
use rust_decimal::prelude::ToPrimitive;

use crate::{function_component, html, use_context, Contexts, Html, Paper};

/// App footer component
pub(crate) fn default_app_footer(contexts: &Contexts) -> Html {
    let app_config = contexts.config.clone();
    let copy_end = Utc::now().year().to_i16().unwrap_or(2022);
    let copy_start = app_config.copyright_year_start.unwrap_or(copy_end);
    let copy_display = if copy_start == copy_end {
        copy_start.to_string()
    } else {
        format!("{}-{}", copy_start, copy_end)
    };
    html! {
        <>
            <webui-flex grow="true"></webui-flex>
            <webui-paper>
                {format!("© {} {} All Rights Reserved", copy_display, app_config.company_name)}
            </webui-paper>
        </>
    }
}

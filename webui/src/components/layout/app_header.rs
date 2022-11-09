use crate::*;

/// App footer component
#[function_component(AppHeader)]
pub(crate) fn app_footer() -> Html {
    let app_config = use_context::<AppConfig>().expect("no app config found");
    let left_drawer_info = app_config.header_left_drawer_toggle.clone();
    let top_drawer_info = app_config.header_top_drawer_toggle.clone();
    let right_drawer_info = app_config.header_right_drawer_toggle.clone();

    html! {
        <header>
            <AppDrawerButton info={left_drawer_info.clone()}
                class="logo"
                logosrc="/Logo.svg"
                logotitle={format!("{} Logo", app_config.company_name.to_owned())}
                />
            <h1 class="flex-grow">{ app_config.app_name.clone() }</h1>
            <AppDrawerButton info={top_drawer_info.clone()} />
            {app_config.header_strip_bar.unwrap_or(empty_html)()}
            {app_config.user_info_panel.unwrap_or(empty_html)()}
            <AppDrawerButton info={right_drawer_info.clone()} />
        </header>
    }
}

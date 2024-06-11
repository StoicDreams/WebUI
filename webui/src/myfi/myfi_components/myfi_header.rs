use crate::prelude::*;

/// App header component
pub fn myfi_app_header(contexts: &Contexts) -> Html {
    let app_config = contexts.clone().config;
    let mut strip_bar = None::<fn(contexts: &Contexts) -> Html>;
    let mut show_discord = false;
    #[cfg(feature = "stoic")]
    {
        strip_bar = Some(stoic_header_strip_bar);
        show_discord = true;
    }
    html! {
        <>
            <h1>{ app_config.app_name.clone() }</h1>
            <h2 class="flex-grow" data-subsribe="page-title"></h2>
            {if let Some(strip_bar) = &strip_bar {
                html!{strip_bar(contexts)}
            }else{html!()}}
            <webui-feedback title="Provide us your feedback!" data-post="https://api.myfi.ws/feedback/new" data-json-name="message">
                {if show_discord {
                    html!{
                        <p>
                            {"You can also come "}
                            <a href="https://discord.com/channels/972856291909332993/1025781071608037466">{"chat with us on the Stoic Dreams discord server."}</a>
                        </p>
                    }
                }else{html!()}}
            </webui-feedback>
            <webui-alerts title="View My Alerts" data-title="My Alerts" data-toggleclass=".shared|open"></webui-alerts>
            {myfi_info_panel(contexts)}
        </>
    }
}

use web_sys::KeyboardEvent;
use yew::AttrValue;

use crate::prelude::*;

const WRAPPER_STYLES: &str = "min-width:300px;";

/// The info panel for myfi account and other services.
pub fn myfi_info_panel(contexts: &Contexts) -> Html {
    let user = contexts.user.deref().to_owned();
    if let Some(user) = user {
        let class = if user.roles > 0 {
            "btn theme-success"
        } else {
            ""
        }
        .to_string();
        return html! {
            <AppDrawerButton info={drawer_toggle_info(contexts)} {class}>
                <span>{user.display_name}</span>
            </AppDrawerButton>
        };
    }
    html! {
        <Paper class="d-inlineblock">
            <Loading variant={LoadingVariant::Circle} size={LOADING_SIZE_MEDIUM} color={Theme::Info} />
        </Paper>
    }
}

fn drawer_toggle_info(_contexts: &Contexts) -> DrawerToggleInfo {
    drawer!(
        "Account Services",
        html! {<webui-fa icon="user" family="duotone" />},
        get_render_wrapper,
        Direction::Right
    )
    .hide_close_x_button()
    .hide_cancel_button()
    .set_on_confirm("Close", handle_confirm)
    .build()
}

pub(crate) fn get_render_wrapper(contexts: &Contexts) -> Html {
    let user_state = contexts.clone().user;
    let user_state = user_state.deref();

    if let Some(user) = user_state {
        if user.roles > 0 {
            return render_with_user(contexts, user);
        }
        return render_without_user();
    }
    html! {
        <Paper class="d-inlineblock">
            <Loading variant={LoadingVariant::Circle} size={LOADING_SIZE_LARGE} color={Theme::Info} />
        </Paper>
    }
}

fn render_without_user() -> Html {
    html! {
        <Paper class="d-flex flex-column" style={WRAPPER_STYLES}>
            <DisplayLoginSignup />
        </Paper>
    }
}

#[function_component(DisplayLoginSignup)]
fn display_login_signup() -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    if let Some(user) = contexts.user.deref() {
        if let Some(site_id) = &user.site_id {
            return html! {<DisplaySigninLink site_id={site_id.to_string()} />};
        }
        return html! {
            <Paper>
                <p>{"This application is not currently configured for Stoic Dreams account services."}</p>
            </Paper>
        };
    }
    html! {
        <Paper class="d-flex flex-column justify-center">
            <Loading variant={LoadingVariant::Circle} size={LOADING_SIZE_MEDIUM} color={Theme::Info} />
        </Paper>
    }
}

#[derive(PartialEq, Properties, Clone)]
struct DisplaySigninLinkOptions {
    site_id: AttrValue,
}

#[function_component(DisplaySigninLink)]
fn display_signin_link(props: &DisplaySigninLinkOptions) -> Html {
    let contexts = use_context::<Contexts>().expect("Failed to load contexts");
    let target = if IS_TAURI_APP { "_blank" } else { "_self" };
    let show_code = use_state(|| false);
    let code_input = use_state(String::default);
    let onclick = {
        let is_app = IS_TAURI_APP;
        let show_code = show_code.clone();
        Callback::from(move |_| {
            if is_app {
                show_code.set(true);
            }
        })
    };
    let href = if IS_TAURI_APP {
        format!(
            "https://www.stoicdreams.com/signin?siteid={}&app={}",
            props.site_id, contexts.config.app_name
        )
    } else {
        format!(
            "https://www.stoicdreams.com/signin?siteid={}",
            props.site_id
        )
    };
    html! {
        <Paper style="max-width:400px;" class="d-flex flex-column gap-2">
            <Paper class="d-flex gap-0 field-group-line">
                <Link href={href} {onclick} target={target} class="btn theme-primary flex-grow">{"Sign In with Stoic Dreams"}</Link>
                {if IS_TAURI_APP && !*show_code {
                    let onclick = {
                        let show_code = show_code.clone();
                        Callback::from(move |_| {
                            show_code.set(true);
                        })
                    };
                    html!{
                        <Button class="btn theme-secondary flex-grow" {onclick}>{"Show Sign-In Code Input"}</Button>
                    }
                }else{html!()}}
            </Paper>
            {if *show_code {
                let contexts = contexts.clone();
                let code_input = code_input.clone();
                let start_icon = IconOptions {
                    icon: FaIcon::duotone("key"),
                    color: Theme::Secondary,
                    ..Default::default()
                };
                let onclick = {
                    let contexts = contexts.clone();
                    let code_input = code_input.clone();
                    Callback::from(move |_| {
                        let key = code_input.deref();
                        if key.is_empty() {
                            alert!(contexts, "Missing Input", "No Sign-In Auth code was entered!");
                        } else {
                            jslog!("Navigate to triggered {}", key);
                            nav_to!(contexts, format!("/sdauth?key={}", key));
                        }
                    })
                };
                html!{
                    <Paper class="field-group-line grow-open">
                        <InputText value={code_input.clone()} {start_icon}
                            end_button={html_nested! {
                                <Button color={Theme::Primary} {onclick} title="Confirm your sign-in code after entering.">{"Confirm"}</Button>
                            }} />
                    </Paper>
                }
            }else{html!()}}
            <Quote color={Theme::Info}>{"Stoic Dreams Account Services is provided securely through www.stoicdreams.com. Clicking this Sign-In button will redirect you to www.stoicdreams.com where you can sign-in to your Stoic Dreams account and choose what personal information, such as your full name and email, is shared with this site."}</Quote>
        </Paper>
    }
}

fn render_with_user(contexts: &Contexts, user: &MyFiUser) -> Html {
    let onclick = {
        let contexts_signout = contexts.clone();
        Callback::from(move |_| {
            sign_out(&contexts_signout);
        })
    };
    html! {
        <Paper class="d-flex flex-column" style={WRAPPER_STYLES}>
            {title_primary!(&format!("Hello {}!", user.display_name.to_owned()))}
            <Paper class="flex-grow"></Paper>
            <Button onclick={onclick}>{"Sign Out"}</Button>
        </Paper>
    }
}

fn sign_out(contexts: &Contexts) {
    let confirm_signout_this_app = {
        let contexts_signout = contexts.clone();
        Callback::from(move |_| {
            contexts_signout.user.set(None);
            contexts_signout.user_roles.set(0);
            myfi_sign_out(&contexts_signout, SignoutScope::ThisApp);
        })
    };
    let confirm_signout_this_browser = {
        let contexts_signout = contexts.clone();
        Callback::from(move |_| {
            contexts_signout.user.set(None);
            contexts_signout.user_roles.set(0);
            myfi_sign_out(&contexts_signout, SignoutScope::ThisBrowser);
        })
    };
    let confirm_signout_this_all_devices = {
        let contexts_signout = contexts.clone();
        Callback::from(move |_| {
            contexts_signout.user.set(None);
            contexts_signout.user_roles.set(0);
            myfi_sign_out(&contexts_signout, SignoutScope::AllDevices);
        })
    };
    let render_confirmation = {
        let confirm_signout_this_app = confirm_signout_this_app.clone();
        let confirm_signout_sd_acount = confirm_signout_this_browser.clone();
        move |_contexts: &Contexts| {
            html! {
                <>
                    <Paper class="flex-grow" />
                    <Paper class="d-flex flex-row flex-wrap gap-2">
                    <Button onclick={confirm_signout_this_all_devices.to_owned()} color={Theme::Danger}>{"All Devices"}</Button>
                        {if !IS_TAURI_APP {
                            html!{<Button onclick={confirm_signout_sd_acount.to_owned()} color={Theme::Warning}>{"All Websites"}</Button>}
                        } else {html!()}}
                        <Button onclick={confirm_signout_this_app.to_owned()} color={Theme::Success}>{"Sign Out"}</Button>
                    </Paper>
                </>
            }
        }
    };
    let markdown = if IS_TAURI_APP {
        r#"Would you like to sign out of just this application or sign out of all Stoic Dreams account services?
        Selecting `Sign Out` will sign you out of this application only.
        Selecting `All Devices` will sign you out of all Stoic Dreams services across all devices and browsers."#
    } else {
        r#"Would you like to sign out of just this website or all websites?
        Selecting `Sign Out` will sign you out of this website only.
        Selecting `All Websites` will sign you out of all websites that use Stoic Dreams account services within this browser.
        Selecting `All Devices` will sign you out of all Stoic Dreams services across all devices and browsers."#
    };
    // confirm if user wants to sign out of just this website or all websites
    dialog!(
        contexts,
        "Sign Out Options",
        {
            html! {
                <Paper class="d-flex flex-column gap-1">
                    <MarkdownContent markdown={markdown} />
                </Paper>
            }
        },
        render_confirmation
    );
}

fn handle_confirm(_contexts: &Contexts) -> bool {
    true
}

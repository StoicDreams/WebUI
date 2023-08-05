use web_sys::KeyboardEvent;

use crate::prelude::*;

const WRAPPER_STYLES: &str = "min-width:300px;";

/// The info panel for myfi account and other services.
pub fn myfi_info_panel(contexts: Contexts) -> Html {
    let user = contexts.user.deref().deref().to_owned();
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

fn drawer_toggle_info(_contexts: Contexts) -> DrawerToggleInfo {
    DrawerToggleInfo::builder(
        |_contexts: Contexts| String::from("Account Services"),
        |_contexts: Contexts| html! {<i class="fa-duotone fa-user" />},
        DynContextsHtml::new(get_render_wrapper),
    )
    .set_drawer(Direction::Right)
    .hide_close_x_button()
    .hide_cancel_button()
    .set_on_confirm("Close", handle_confirm)
    .build()
}

pub(crate) fn get_render_wrapper(contexts: Contexts) -> Html {
    let user_state = contexts.clone().user;
    let user_state = user_state.deref();

    if let Some(user) = user_state.deref() {
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
            let href = format!("https://www.stoicdreams.com/signin?siteid={site_id}",);
            return html! {
                <Paper>
                    <Link href={href} target="_self" class="btn theme-primary">{"Sign In or Create Account"}</Link>
                </Paper>
            };
        }
        return html! {
            <Paper>
                <p>{"This website is not currently configured for Stoic Dreams account services."}</p>
            </Paper>
        };
    }
    html! {
        <Paper class="d-flex flex-column justify-center">
            <Loading variant={LoadingVariant::Circle} size={LOADING_SIZE_MEDIUM} color={Theme::Info} />
        </Paper>
    }
}

fn render_with_user(contexts: Contexts, user: &MyFiUser) -> Html {
    let onclick = {
        let contexts_signout = contexts.clone();
        Callback::from(move |_| {
            sign_out(contexts_signout.clone());
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

fn sign_out(contexts: Contexts) {
    let confirm_signout_this_website = {
        let contexts_signout = contexts.clone();
        Callback::from(move |_| {
            contexts_signout.user.set(None);
            myfi_sign_out(contexts_signout.clone());
        })
    };
    let confirm_signout_sd_acount = {
        let contexts_signout = contexts.clone();
        Callback::from(move |_| {
            contexts_signout.user.set(None);
            myfi_sign_out(contexts_signout.clone());
        })
    };
    // confirm if user wants to sign out of just this website or all websites
    dialog!(contexts, "Sign Out Options", {
        html! {
            <Paper class="d-flex flex-column gap-1">
                <p>{"Would you like to sign out of just this website or all websites?"}</p>
                <p>{"Selecting 'All Websites' will sign you out of all websites that use Stoic Dreams account services."}</p>
                <p>{"Selecting 'Just This Website' will sign you out of this website only."}</p>
                <Paper class="d-flex flex-row flex-wrap gap-2">
                    <Button onclick={confirm_signout_this_website.to_owned()}>{"Just This Website"}</Button>
                    <Button onclick={confirm_signout_sd_acount.to_owned()}>{"All Websites"}</Button>
                </Paper>
            </Paper>
        }
    });
}

fn handle_confirm(_contexts: Contexts) -> bool {
    true
}

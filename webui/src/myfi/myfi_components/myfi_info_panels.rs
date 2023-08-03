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

fn drawer_toggle_info(contexts: Contexts) -> DrawerToggleInfo {
    DrawerToggleInfo::builder(
        |contexts: Contexts| String::from("Account Services"),
        |contexts: Contexts| html! {<i class="fa-duotone fa-user" />},
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
    }
    render_without_user()
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
    let tab_keys = vec![String::from("Sign In"), String::from("Create Acount")];
    html! {
        <Paper>
            <TabbedContent tabs={tab_keys} class="d-flex flex-column gap-1">
                <SignIn />
                <SignUp />
            </TabbedContent>
        </Paper>
    }
}

#[function_component(SignUp)]
fn sign_up() -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    html! {
        <>
            {title_primary!(&format!("Create a new {} account!", get_company_singular()))}
            <p>{"Coming Soon!"}</p>
        </>
    }
}

#[function_component(SignIn)]
fn sign_in() -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let alert = use_state(|| "".to_string());
    let is_submitting = use_state(|| false);
    let submit_form = {
        let contexts = contexts.clone();
        let email = email.clone();
        let password = password.clone();
        let alert = alert.clone();
        let is_submitting = is_submitting.clone();
        move || {
            is_submitting.set(true);
            alert.set(String::default());
            let email = email.deref().to_owned();
            let password = password.deref().to_owned();
            if let Some(error) = validate_email(&email) {
                alert.set(error);
                is_submitting.set(false);
                return;
            }
            if let Some(error) = validate_password(&password) {
                alert.set(error);
                is_submitting.set(false);
                return;
            }
            myfi_sign_in(
                contexts.clone(),
                &email,
                &password,
                alert.clone(),
                is_submitting.clone(),
            )
        }
    };
    let submit = {
        let submit_form = submit_form.clone();
        Callback::from(move |_| submit_form.clone()())
    };
    let form_detect_enter = {
        let submit_form = submit_form.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                submit_form();
            }
        })
    };
    html! {
        <>
            {if is_submitting.deref().to_owned() {
                html!{<Loading variant={LoadingVariant::Circle} color={Theme::Primary} size={60} />}
            } else {
                html!{
                    <>
                        {title_primary!(&format!("Sign in to your {} account!", get_company_singular()))}
                        <form class="d-flex flex-column gap-1" name="myfi-sign-in-form" autocomplete="on" onkeyup={form_detect_enter}>
                            <InputText name="Email" value={email.clone()} />
                            <InputText t="password" name="Password" value={password.clone()} />
                        </form>
                        <Button onclick={submit}>{"Sign In"}</Button>
                        {if !alert.deref().to_owned().is_empty() {
                            html!{<Alert color={Theme::Danger}>{alert.deref().to_owned()}</Alert>}
                        } else {
                            html!{}
                        }}
                    </>
                }
            }}
        </>
    }
}

fn render_with_user(contexts: Contexts, user: &MyFiUser) -> Html {
    let onclick = {
        let contexts_signout = contexts.clone();
        Callback::from(move |_| {
            myfi_sign_out(contexts_signout.clone());
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

fn handle_confirm(_contexts: Contexts) -> bool {
    true
}

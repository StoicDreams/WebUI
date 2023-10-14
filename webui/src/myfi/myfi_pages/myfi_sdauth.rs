use crate::prelude::*;

pub(crate) fn page_sdauth(_contexts: Contexts) -> Html {
    set_title("Stoic Dreams Account Authentication");
    html! {
        <RenderPage />
    }
}

/// Get key used for storing MyFi client auth key in user storage.
pub(crate) fn get_myfi_auth_token_key() -> String {
    String::from("stoic_dreams_auth_token")
}

#[function_component(RenderPage)]
fn render_page() -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let auth_key = query_url("key");
    if auth_key.is_none() {
        return html! {
            <>
                <MyFiStorageConsent />
                <Paper class="d-flex flex-column justify-left align-left">
                    <Quote color={Theme::Success}>
                        {"You are currently signed in with your Stoic Dreams account."}
                    </Quote>
                </Paper>
                <NextPageButton url="/" snap_bottom={false} />
            </>
        };
    }
    let page_messages = use_state(|| {
        String::from(
            r#"
```quote "info"
Validating account.
```
"#,
        )
    });
    let displayed_markdown = page_messages.deref().to_owned();
    if !displayed_markdown.contains("Validating account.") {
        return html! {
            <>
                <MyFiStorageConsent />
                <Paper class="d-flex flex-column justify-left align-left">
                    <MarkdownContent markdown={displayed_markdown} />
                </Paper>
            </>
        };
    }
    let pmthread = page_messages.to_owned();
    let auth_key = auth_key.to_owned();
    spawn_async!({
        match auth_key {
            Some(key) => {
                set_user_storage_data(get_myfi_auth_token_key(), key);
                let user_state = contexts.clone().user;
                match myfi_get_my_info(user_state).await {
                    true => {
                        push_state("/sdauth");
                        pmthread.set(String::from(
                            r#"
```quote "success"
Sign-in to Stoic Dreams account successful.
```
"#,
                        ));
                    }
                    false => {
                        pmthread.set(String::from(
                            r#"
```quote "danger"
Sign-in to Stoic Dreams account failed. Key is invalid or may have expired.
```
"#,
                        ));
                        set_user_storage_data(get_myfi_auth_token_key(), String::default());
                    }
                }
            }
            None => {
                pmthread.set(String::from(
                    r#"
```quote "danger"
Expected key was not found for account authentication.
```
"#,
                ));
            }
        }
    });
    html! {
        <>
            <Loading variant={LoadingVariant::StripedBar} color={Theme::Primary} size={LOADING_SIZE_XLARGE} />
            <Paper class="d-flex flex-column justify-left align-left">
                <MarkdownContent markdown={displayed_markdown} />
            </Paper>
        </>
    }
}

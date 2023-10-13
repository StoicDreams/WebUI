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
    let page_messages = use_state(|| {
        String::from(
            r#"
```quote "info"
Validating account.
```
"#,
        )
    });
    let pmthread = page_messages.to_owned();
    if page_messages.contains("Validating account.") {
        spawn_async!({
            match query_url("key") {
                Some(key) => {
                    set_user_storage_data(get_myfi_auth_token_key(), key);
                    let user_state = contexts.clone().user;
                    match myfi_get_my_info(user_state).await {
                        true => {
                            pmthread.set(String::from(
                                r#"
```quote "success"
Sign-in to Stoic Dreams account successful.
```
```paper
[Continue to Home](/)
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
    }
    let displayed_markdown = page_messages.deref().to_owned();
    if displayed_markdown.contains("Validating") {
        return html! {
            <>
                <MyFiStorageConcent />
                <Paper class="d-flex flex-grow flex-column justify-left align-left">
                    <MarkdownContent markdown={displayed_markdown} />
                </Paper>
                <Paper />
            </>
        };
    }
    html! {
        <>
            <MyFiStorageConcent />
            <Paper class="d-flex flex-grow flex-column justify-left align-left">
                <MarkdownContent markdown={displayed_markdown} />
            </Paper>
        </>
    }
}

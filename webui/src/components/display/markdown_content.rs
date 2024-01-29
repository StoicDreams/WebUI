use crate::prelude::*;
use std::collections::HashMap;

/// Get the default tags for markdown content
pub fn get_markdown_tags() -> HashMap<String, String> {
    let mut tags = HashMap::<String, String>::new();
    tags.insert(
        "COMPANY_SINGULAR".to_string(),
        get_company_singular().to_string(),
    );
    tags.insert(
        "COMPANY_PLURAL".to_string(),
        get_company_plural().to_string(),
    );
    tags.insert("APP_NAME".to_string(), get_app_name().to_string());
    tags.insert("DOMAIN".to_string(), get_domain().to_string());
    tags
}

/// Properties for Image component
#[derive(Properties, PartialEq)]
pub struct MarkdownContentProps {
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub markdown: Option<String>,
    #[prop_or_default]
    pub tags: Option<HashMap<String, String>>,
}

/// Component for loading and displaying site content from markdown files
///
/// Basic example displaying from url
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: Contexts) -> Html {
///     html! {
///         <MarkdownContent href="/d/en-us/example.md"/>
///     }
/// }
/// ```
///
/// Apply elevetation
///
/// Basic example displaying from passed in value
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: Contexts) -> Html {
///     html! {
///         <MarkdownContent markdown="# Hello World" />
///     }
/// }
/// ```
#[function_component(MarkdownContent)]
pub fn markdown_content(props: &MarkdownContentProps) -> Html {
    let is_loaded = use_state(|| false);
    let is_loading = use_state(|| false);
    let cached_href = use_state(String::default);
    let markdown = use_state(String::default);
    let href = props.href.to_owned().unwrap_or_default();
    if *is_loaded && *cached_href != href {
        is_loaded.set(false);
        return html!(<Loading size={LOADING_SIZE_LARGE} />);
    }
    if let Some(md) = props.markdown.to_owned() {
        if !*is_loaded {
            let md = match &props.tags {
                Some(tags) => replace_tags(&md, tags),
                None => md,
            };
            let md = trim_left_padding(&md);
            is_loaded.set(true);
            markdown.set(md);
        }
    };
    if !*is_loaded || (*markdown).is_empty() {
        if *is_loading {
            return html!(<Loading size={LOADING_SIZE_LARGE} />);
        }
        if let Some(href) = props.href.to_owned() {
            is_loading.set(true);
            let md = markdown;
            if *cached_href != href {
                cached_href.set(href.to_owned());
            }
            let tags = props.tags.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = fetch(FetchRequest::new(href, FetchMethod::Get)).await;
                if !response.is_ok() {
                    md.set("Failed to load content.".to_string());
                    is_loaded.set(true);
                    is_loading.set(false);
                    return;
                }
                match response.get_result() {
                    Some(body) => {
                        if body.starts_with("<!DOCTYPE") {
                            md.set("Content is invalid type.".to_string());
                            is_loaded.set(true);
                            is_loading.set(false);
                            return;
                        }
                        let body = match &tags {
                            Some(tags) => replace_tags(&body, tags),
                            None => body,
                        };
                        md.set(body);
                        is_loaded.set(true);
                        is_loading.set(false);
                    }
                    None => {
                        md.set("Failed to load content body.".to_string());

                        is_loaded.set(true);
                        is_loading.set(false);
                    }
                }
            });
            return html!(<Loading size={LOADING_SIZE_LARGE} />);
        }
    }

    if (*markdown).is_empty() {
        return html!(<Loading size={LOADING_SIZE_LARGE} />);
    }

    html! {
        {render_markdown(&markdown)}
    }
}

pub fn markdown_to_html(markdown: &str) -> Html {
    render_markdown(markdown)
}

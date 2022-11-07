use std::{cmp::Ordering, collections::HashMap};
use crate::*;

mod children;
mod code_segments;
mod helpers;
mod lines;
mod line_content;
mod line_segments;
mod line_types;
mod list;
mod markdown_segments;
mod table;
mod tags;

use children::*;
use code_segments::*;
use helpers::*;
use lines::*;
use line_content::*;
use line_segments::*;
use line_types::*;
use list::*;
use markdown_segments::*;
use tags::*;
use table::*;

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
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		<MarkdownContent href="/d/en-us/example.md"/>
/// 	}
/// }
/// ```
///
/// Apply elevetation
///
/// Basic example displaying from passed in value
/// ```rust
/// use webui::*;
///
/// fn page() -> Html {
/// 	html! {
/// 		<MarkdownContent markdown="# Hello World" />
/// 	}
/// }
/// ```
#[function_component(MarkdownContent)]
pub fn site_content(props: &MarkdownContentProps) -> Html {
    let is_loaded = use_state(|| false);
    let is_loading = use_state(|| false);
    let cached_href = use_state(|| String::default());
    // let markdown = use_state(|| String::default());
    let markdown = use_state(|| Vec::<(String, MarkdownSegments)>::new());
    let href = props.href.to_owned().unwrap_or_default();
    if *is_loaded && *cached_href != href {
        is_loaded.set(false);
        return html!(<Loading size={LOADING_SIZE_LARGE} />);
    }
    match props.markdown.to_owned() {
        Some(md) => {
            let md = match &props.tags {
                Some(tags) => replace_tags(&md.clone(), tags),
                None => md.clone(),
            };
            markdown.set(parse_markdown(&md));
        }
        None => {}
    };
    if !*is_loaded || (*markdown).is_empty() {
        if *is_loading {
            return html!(<Loading size={LOADING_SIZE_LARGE} />);
        }
        match props.href.to_owned() {
            Some(href) => {
                is_loading.set(true);
                let md = markdown.clone();
                if *cached_href != href {
                    cached_href.set(href.to_owned());
                }
                let tags = props.tags.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response = fetch(FetchRequest::new(href, FetchMethod::Get)).await;
                    if !response.is_ok() {
                        md.set(parse_markdown("Failed to load content."));
                        is_loaded.set(true);
                        is_loading.set(false);
                        return;
                    }
                    match response.get_result() {
                        Some(body) => {
                            if body.starts_with("<!DOCTYPE") {
                                md.set(parse_markdown("Content is invalid type."));
                                is_loaded.set(true);
                                is_loading.set(false);
                                return;
                            }
                            let body = match &tags {
                                Some(tags) => replace_tags(&body.clone(), tags),
                                None => body.clone(),
                            };
                            md.set(parse_markdown(&body));
                            is_loaded.set(true);
                            is_loading.set(false);
                        }
                        None => {
                            md.set(parse_markdown("Failed to load content body."));

                            is_loaded.set(true);
                            is_loading.set(false);
                        }
                    }
                });
                return html!(<Loading size={LOADING_SIZE_LARGE} />);
            }
            None => {}
        }
    }

    if (*markdown).is_empty() {
        return html!(<Loading size={LOADING_SIZE_LARGE} />);
    }

    html! {
        {start_render_children(&*markdown)}
    }
}

pub fn markdown_to_html(markdown: &str) -> Html {
    let markdown = parse_markdown(markdown);
    html!(
        <>
            {start_render_children(&markdown)}
        </>
    )
}

fn parse_markdown(markdown: &str) -> Vec<(String, MarkdownSegments)> {
    let mut lines = Vec::new();
    for line in markdown.lines() {
        lines.push(get_line_type(line));
    }
    lines.to_owned()
}

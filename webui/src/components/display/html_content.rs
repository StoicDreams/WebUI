use web_sys::HtmlElement;

use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HtmlContentProps {
    pub html: String,
}

/// A component for displaying HTML content directly from a string.
///
/// Example:
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: Contexts) -> Html {
///     html! {
///         <HtmlContent html={r#"
///             <h1>Hello World</h1>
///             <p>This is a paragraph</p>
///             "#} />
///     }
/// }
/// ```
#[function_component(HtmlContent)]
pub fn html_content(props: &HtmlContentProps) -> Html {
    let window = web_sys::window().expect("Missing Window");
    let document = window.document().expect("Missing Document");
    let el = document
        .create_element("div")
        .expect("Failed to create div for html_content");
    el.set_inner_html(&props.html.clone());
    let collection = el.child_nodes();
    let mut nodes = vec![];
    for elem in 0..collection.length() {
        if let Some(item) = collection.item(elem) {
            nodes.push(item);
        }
    }
    html! {nodes.iter().map(|node|{
        html!{Html::VRef(node.to_owned())}
    }).collect::<Html>()}
}

#[derive(Properties, PartialEq)]
pub struct StyleProps {
    pub styles: String,
}

/// A component for rendering a style tag for inline styles.
#[function_component(StyleContent)]
pub fn style_content(props: &StyleProps) -> Html {
    let window = web_sys::window().expect("Missing Window");
    let document = window.document().expect("Missing Document");
    let el = document
        .create_element("style")
        .expect("Failed to create style");
    el.set_attribute("rel", "style");
    el.set_inner_html(&props.styles.clone());
    Html::VRef(el.into())
}

#[derive(Properties, PartialEq)]
pub struct ScriptProps {
    pub script: String,
    pub delay: usize,
}

/// A component for rendering a script tag for running inline javascript.
///
/// The given script will be delayed by the desired amount before running, and the script tag will be destroyed at the end of the run.
#[function_component(JavaScriptContent)]
pub fn javascript_content(props: &ScriptProps) -> Html {
    let window = web_sys::window().expect("Missing Window");
    let document = window.document().expect("Missing Document");
    let el = document
        .create_element("script")
        .expect("Failed to create script");
    el.set_attribute("type", "text/javascript");
    let id = format!("JS{}", newid().to_string());
    el.set_attribute("id", &id);
    let script = format!(
        r#"
setTimeout(async()=>{{
    try {{
        {}
    }} catch (ex) {{
        console.error('Failed inline script', ex);
    }}
    document.getElementById(`{}`).remove();
}}, {})
"#,
        props.script, id, props.delay
    );
    el.set_inner_html(&script);
    Html::VRef(el.into())
}

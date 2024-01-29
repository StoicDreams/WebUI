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
        .expect("Failed to create div");
    el.set_inner_html(&props.html.clone());
    el.set_class_name("paper d-flex align-center justify-center");
    _ = el.set_attribute("style", "height: auto;width: auto;");
    html! {
        <div class="paper d-flex align-center justify-center">
            {Html::VRef(el.into())}
        </div>
    }
}

/// A component for rendering raw html markup inside of a span.
///
/// This is intended for smaller text segments, such as a single line.
#[function_component(SpanHtmlContent)]
pub fn span_html_content(props: &HtmlContentProps) -> Html {
    let window = web_sys::window().expect("Missing Window");
    let document = window.document().expect("Missing Document");
    let el = document
        .create_element("span")
        .expect("Failed to create span");
    el.set_inner_html(&props.html.clone());
    Html::VRef(el.into())
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

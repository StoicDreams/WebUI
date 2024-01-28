use markdown::mdast;

use crate::prelude::*;

pub fn render_markdown(markdown: &str) -> Html {
    match markdown::to_mdast(markdown, &markdown::ParseOptions::gfm()) {
        Ok(node) => {
            transpose_node_to_markdownitem(&node)
        }
        Err(err) => {
            html!{err}
        }
    }
}

fn transpose_node_to_markdownitem(node: &mdast::Node) -> Html {
    match node {
        mdast::Node::Root(root) => handle_root(root),
        mdast::Node::BlockQuote(blockquote) => handle_blockquote(blockquote),
        mdast::Node::FootnoteDefinition(footnote_definition) => handle_footnote_definition(footnote_definition),
        mdast::Node::MdxJsxFlowElement(mdx_jsx_flow_element) => handle_mdx_jsx_flow_element(mdx_jsx_flow_element),
        mdast::Node::List(list) => handle_list(list),
        mdast::Node::MdxjsEsm(mdx_js_esm) => handle_mdx_js_esm(mdx_js_esm),
        mdast::Node::Toml(toml) => handle_toml(toml),
        mdast::Node::Yaml(yaml) => handle_yaml(yaml),
        mdast::Node::Break(line_break) => handle_line_break(line_break),
        mdast::Node::InlineCode(inline_code) => handle_inline_code(inline_code),
        mdast::Node::InlineMath(inline_math) => handle_inline_math(inline_math),
        mdast::Node::Delete(delete) => handle_delete(delete),
        mdast::Node::Emphasis(emphasis) => handle_emphasis(emphasis),
        mdast::Node::MdxTextExpression(mdx_text_expressions) => handle_mdx_text_expressions(mdx_text_expressions),
        mdast::Node::FootnoteReference(footnote_reference) => handle_footnote_reference(footnote_reference),        
        mdast::Node::Html(html) => handle_html(html),
        mdast::Node::Image(image) => handle_image(image),
        mdast::Node::ImageReference(image_reference) => handle_image_reference(image_reference),
        mdast::Node::MdxJsxTextElement(mdx_jsx_text_element) => handle_mdx_jsx_text_element(mdx_jsx_text_element),
        mdast::Node::Link(link) => handle_link(link),
        mdast::Node::LinkReference(link_reference) => handle_link_reference(link_reference),
        mdast::Node::Strong(strong) => handle_strong(strong),
        mdast::Node::Text(text) => handle_text(text),
        mdast::Node::Code(code) => handle_code(code),
        mdast::Node::Math(math) => handle_math(math),
        mdast::Node::MdxFlowExpression(mdx_flow_expr) => handle_mdx_flow_expr(mdx_flow_expr),
        mdast::Node::Heading(heading) => handle_heading(heading),
        mdast::Node::Table(table) => handle_table(table),
        mdast::Node::ThematicBreak(thematic_break) => handle_thematic_break(thematic_break),
        mdast::Node::TableRow(table_row) => handle_table_row(table_row),
        mdast::Node::TableCell(table_cell) => handle_table_cell(table_cell),
        mdast::Node::ListItem(list_item) => handle_list_item(list_item),
        mdast::Node::Definition(definition) => handle_definition(definition),
        mdast::Node::Paragraph(paragraph) => handle_paragraph(paragraph),
    }
}

fn handle_children(children: &Vec<mdast::Node>) -> Html {
    html!{
        children.iter().map(|node|{
            {transpose_node_to_markdownitem(node)}
        }).collect::<Html>()
    }
}

fn handle_root(root: &mdast::Root) -> Html {
    html!{handle_children(&root.children)}
}

fn handle_blockquote(blockquote: &mdast::BlockQuote) -> Html {
    html!{format!("Unhandled blockquote:{:?} {}", blockquote, blockquote.children.len())}
}

fn handle_footnote_definition(footnote_definition: &mdast::FootnoteDefinition) -> Html {
    html!{format!("Unhandled footnote_definition:{:?} {}", footnote_definition, footnote_definition.children.len())}
}

fn handle_mdx_jsx_flow_element(mdx_jsx_flow_element: &mdast::MdxJsxFlowElement) -> Html {
    html!{format!("Unhandled mdx_jsx_flow_element:{:?} {}", mdx_jsx_flow_element, mdx_jsx_flow_element.children.len())}
}

fn handle_list(list: &mdast::List) -> Html {
    html!{format!("Unhandled list:{:?} {}", list, list.children.len())}
}

fn handle_mdx_js_esm(mdx_js_esm: &mdast::MdxjsEsm) -> Html {
    html!{format!("Unhandled mdx_js_esm:{:?} {}", mdx_js_esm, mdx_js_esm.value)}
}

fn handle_toml(toml: &mdast::Toml) -> Html {
    html!{format!("Unhandled toml:{:?} {}", toml, toml.value)}
}

fn handle_yaml(yaml: &mdast::Yaml) -> Html {
    html!{format!("Unhandled yaml:{:?} {}", yaml, yaml.value)}
}

fn handle_line_break(line_break: &mdast::Break) -> Html {
    html!{format!("Unhandled line_break:{:?}", line_break)}
}

fn handle_inline_code(inline_code: &mdast::InlineCode) -> Html {
    html!{format!("Unhandled inline_code:{:?} {}", inline_code, inline_code.value)}
}

fn handle_inline_math(inline_math: &mdast::InlineMath) -> Html {
    html!{format!("Unhandled inline_math:{:?} {}", inline_math, inline_math.value)}
}

fn handle_delete(delete: &mdast::Delete) -> Html {
    html!{format!("Unhandled delete:{:?} {}", delete, delete.children.len())}
}

fn handle_emphasis(emphasis: &mdast::Emphasis) -> Html {
    html!{format!("Unhandled emphasis:{:?} {}", emphasis, emphasis.children.len())}
}

fn handle_mdx_text_expressions(mdx_text_expressions: &mdast::MdxTextExpression) -> Html {
    html!{format!("Unhandled mdx_text_expressions:{:?} {}", mdx_text_expressions, mdx_text_expressions.value)}
}

fn handle_footnote_reference(footnote_reference: &mdast::FootnoteReference) -> Html {
    html!{format!("Unhandled footnote_reference:{:?}", footnote_reference)}
}

fn handle_html(html: &mdast::Html) -> Html {
    html!{format!("Unhandled html:{:?} {}", html, html.value)}
}

fn handle_image(image: &mdast::Image) -> Html {
    html!{format!("Unhandled image:{:?}", image)}
}

fn handle_image_reference(image_reference: &mdast::ImageReference) -> Html {
    html!{format!("Unhandled image_reference:{:?}", image_reference)}
}

fn handle_mdx_jsx_text_element(mdx_jsx_text_element: &mdast::MdxJsxTextElement) -> Html {
    html!{format!("Unhandled mdx_jsx_text_element:{:?} {}", mdx_jsx_text_element, mdx_jsx_text_element.children.len())}
}

fn handle_link(link: &mdast::Link) -> Html {
    html!{format!("Unhandled link:{:?}", link)}
}

fn handle_link_reference(link_reference: &mdast::LinkReference) -> Html {
    html!{format!("Unhandled link_reference:{:?}", link_reference)}
}

fn handle_strong(strong: &mdast::Strong) -> Html {
    html!{format!("Unhandled strong:{:?}", strong)}
}

fn handle_text(text: &mdast::Text) -> Html {
    html!{<>{&text.value}</>}
}

fn handle_code(code: &mdast::Code) -> Html {
    let meta = match &code.meta {
        Some(class) => class.split(' ').collect::<Vec<&str>>(),
        None => vec![]
    };
    let class = if meta.len() > 0 { meta[0] } else { "" };
    let style = if meta.len() > 0 { meta[0] } else { "" };
    match &code.lang {
        Some(lang) => {
            match lang.to_lowercase() {

                _ => {
                    let lang_class = format!("language-{}", lang.to_lowercase());
                    let class = classes!(lang_class, class);
                    html!(<pre languagetag={lang.to_string()} class={class.to_string()} style={style.to_owned()}>
                        <code class={lang_class}>{&code.value}</code>
                    </pre>)
                }
            }
        },
        None => {            
            let class = classes!(class);
            html!(<pre class={class.to_string()} style={style.to_owned()}>
                <code>{&code.value}</code>
            </pre>)
        }
    }
}

fn handle_math(math: &mdast::Math) -> Html {
    html!{format!("Unhandled math:{:?}", math)}
}

fn handle_mdx_flow_expr(mdx_flow_expr: &mdast::MdxFlowExpression) -> Html {
    html!{format!("Unhandled mdx_flow_expr:{:?}", mdx_flow_expr)}
}

fn handle_heading(heading: &mdast::Heading) -> Html {
    match heading.depth {
        1 => html!{<h1>{handle_children(&heading.children)}</h1>},
        2 => html!{<h2>{handle_children(&heading.children)}</h2>},
        3 => html!{<h3>{handle_children(&heading.children)}</h3>},
        4 => html!{<h4>{handle_children(&heading.children)}</h4>},
        5 => html!{<h5>{handle_children(&heading.children)}</h5>},
        _ => html!{<h6>{handle_children(&heading.children)}</h6>}
    }
}

fn handle_table(table: &mdast::Table) -> Html {
    html!{format!("Unhandled table:{:?}", table)}
}

fn handle_thematic_break(thematic_break: &mdast::ThematicBreak) -> Html {
    html!{format!("Unhandled thematic_break:{:?}", thematic_break)}
}

fn handle_table_row(table_row: &mdast::TableRow) -> Html {
    html!{format!("Unhandled table_row:{:?}", table_row)}
}

fn handle_table_cell(table_cell: &mdast::TableCell) -> Html {
    html!{format!("Unhandled table_cell:{:?}", table_cell)}
}

fn handle_list_item(list_item: &mdast::ListItem) -> Html {
    html!{format!("Unhandled list_item:{:?}", list_item)}
}

fn handle_definition(definition: &mdast::Definition) -> Html {
    html!{format!("Unhandled definition:{:?}", definition)}
}

fn handle_paragraph(paragraph: &mdast::Paragraph) -> Html {
    html!{format!("Unhandled paragraph:{:?}", paragraph)}
}

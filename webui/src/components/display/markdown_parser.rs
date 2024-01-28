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
    let url = link.url.to_owned();
    let title = link.title.to_owned().unwrap_or_default();
    html!{<Link href={url} title={title}>{handle_children(&link.children)}</Link>}
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

fn get_meta(meta: &Vec<&str>, pos: usize) -> String {
    if pos == 0 { return String::default(); }
    if meta.len() < pos { return String::default(); }
    let mut value = meta[pos-1].to_string();
    if value.starts_with('"') {
        value = value[1..].to_string();
    }
    if value.ends_with('"') {
        value = value[..value.len()-1].to_string();
    }
    value
}

fn handle_code(code: &mdast::Code) -> Html {
    let meta = match &code.meta {
        Some(class) => class.split("\" \"").collect::<Vec<&str>>(),
        None => vec![]
    };
    match &code.lang {
        Some(lang) => {
            match lang.to_lowercase().as_str() {
                "automax" | "automaxcontent" => {
                    let class = get_meta(&meta, 1);
                    let style = get_meta(&meta, 2);
                    let class = classes!(CLASSES_AUTO_MAXCONTENT, "gap-2", class).to_string();
                    html!(<Paper class={class} style={style} elevation={ELEVATION_STANDARD}>
                        {render_markdown(&code.value)}
                    </Paper>)
                },
                "cards" => {
                    let class = get_meta(&meta, 1);
                    let style = get_meta(&meta, 2);
                    let class = classes!(CLASSES_CARD_CONTAINER, class).to_string();
                    let style = style.to_string();
                    html!(<Cards {class} {style}>
                        {render_markdown(&code.value)}
                    </Cards>)
                },
                "card" => {
                    let title = get_meta(&meta, 1);
                    let width = u16::from_str(&get_meta(&meta, 2)).unwrap_or(800u16);
                    let theme = get_meta(&meta, 3);
                    let avatar = get_meta(&meta, 4);
                    let link = get_meta(&meta, 5);
                    let link_title = get_meta(&meta, 6);
                    let theme = get_theme(theme.as_str());
                    html!(<Card title={title.to_owned()}
                        width={width}
                        theme={theme}
                        avatar={avatar.to_owned()}
                        link={link.to_owned()}
                        link_title={link_title.to_owned()}
                        elevation={ELEVATION_STANDARD}
                        >
                        {render_markdown(&code.value)}
                    </Card>)
                },
                "list" => {
                    let is_ordered = !get_meta(&meta, 1).is_empty();
                    let class = get_meta(&meta, 2);
                    let style = get_meta(&meta, 3);
                    let markdown = code.value.to_owned();
                    let lines = markdown.lines().filter(|&x| !x.is_empty()).collect::<Vec<&str>>();
                    html!(<List is_ordered={is_ordered} class={class} style={style}>
                        {lines.iter().map(|line|{
                            html!(<li>{render_markdown(line)}</li>)
                        }).collect::<Html>()}
                    </List>)
                },
                "sideimage" => {
                    let image_pos = get_meta(&meta, 1);
                    let src = get_meta(&meta, 2);
                    let class = get_meta(&meta, 3);
                    let style = get_meta(&meta, 4);
                    let image_pos = match image_pos.as_str() {
                        "right" => LeftOrRight::Right,
                        _ => LeftOrRight::Left,
                    };
                    html!(<SideImage {image_pos} class={class.to_owned()} src={src.to_owned()} style={style.to_owned()}>
                        <Paper>{render_markdown(&code.value)}</Paper>
                    </SideImage>)
                },
                _ => {
                    let class = get_meta(&meta, 1);
                    let style = get_meta(&meta, 2);
                    let lang_class = format!("language-{}", lang.to_lowercase());
                    let class = classes!(lang_class, class);
                    html!(<pre languagetag={lang.to_string()} class={class.to_string()} style={style.to_owned()}>
                        <code class={lang_class}>{&code.value}</code>
                    </pre>)
                }
            }
        },
        None => {            
            html!(<pre class="language-plaintext">
                <code class="langugae-plaintext">{&code.value}</code>
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
        1 => html!{<h1 class={format!("theme-primary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h1>},
        2 => html!{<h2 class={format!("theme-secondary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h2>},
        3 => html!{<h3 class={format!("theme-tertiary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h3>},
        4 => html!{<h4 class={format!("theme-tertiary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h4>},
        5 => html!{<h5 class={format!("theme-tertiary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h5>},
        _ => html!{<h6 class={format!("theme-tertiary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h6>}
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
    html!{<p>{handle_children(&paragraph.children)}</p>}
}

pub(super) fn get_theme(theme: &str) -> Theme {
    match theme {
        "active" => Theme::Active,
        "background" => Theme::Background,
        "black" => Theme::Black,
        "white" => Theme::White,
        "secondary" => Theme::Secondary,
        "tertiary" => Theme::Tertiary,
        "info" => Theme::Info,
        "success" => Theme::Success,
        "warning" => Theme::Warning,
        "danger" => Theme::Danger,
        "title" => Theme::Title,
        "inherit" => Theme::None,
        _ => Theme::Primary,
    }
}

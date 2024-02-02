use super::*;
use crate::prelude::*;
use markdown::mdast::{self, AlignKind};
use regex::Regex;

pub fn render_markdown(markdown: &str) -> Html {
    match markdown::to_mdast(markdown, &markdown::ParseOptions::gfm()) {
        Ok(node) => transpose_node_to_markdownitem(&node),
        Err(err) => {
            html! {err}
        }
    }
}

fn transpose_node_to_markdownitem(node: &mdast::Node) -> Html {
    match node {
        mdast::Node::Root(root) => handle_root(root),
        mdast::Node::BlockQuote(blockquote) => handle_blockquote(blockquote),
        mdast::Node::FootnoteDefinition(footnote_definition) => {
            handle_footnote_definition(footnote_definition)
        }
        mdast::Node::MdxJsxFlowElement(mdx_jsx_flow_element) => {
            handle_mdx_jsx_flow_element(mdx_jsx_flow_element)
        }
        mdast::Node::List(list) => handle_list(list),
        mdast::Node::MdxjsEsm(mdx_js_esm) => handle_mdx_js_esm(mdx_js_esm),
        mdast::Node::Toml(toml) => handle_toml(toml),
        mdast::Node::Yaml(yaml) => handle_yaml(yaml),
        mdast::Node::Break(line_break) => handle_line_break(line_break),
        mdast::Node::InlineCode(inline_code) => handle_inline_code(inline_code),
        mdast::Node::InlineMath(inline_math) => handle_inline_math(inline_math),
        mdast::Node::Delete(delete) => handle_delete(delete),
        mdast::Node::Emphasis(emphasis) => handle_emphasis(emphasis),
        mdast::Node::MdxTextExpression(mdx_text_expressions) => {
            handle_mdx_text_expressions(mdx_text_expressions)
        }
        mdast::Node::FootnoteReference(footnote_reference) => {
            handle_footnote_reference(footnote_reference)
        }
        mdast::Node::Html(html) => handle_html(html),
        mdast::Node::Image(image) => handle_image(image),
        mdast::Node::ImageReference(image_reference) => handle_image_reference(image_reference),
        mdast::Node::MdxJsxTextElement(mdx_jsx_text_element) => {
            handle_mdx_jsx_text_element(mdx_jsx_text_element)
        }
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
    html! {
        children.iter().map(|node|{
            {transpose_node_to_markdownitem(node)}
        }).collect::<Html>()
    }
}

fn handle_root(root: &mdast::Root) -> Html {
    html! {handle_children(&root.children)}
}

fn handle_blockquote(blockquote: &mdast::BlockQuote) -> Html {
    html! {<blockquote>{handle_children(&blockquote.children)}</blockquote>}
}

fn handle_footnote_definition(footnote_definition: &mdast::FootnoteDefinition) -> Html {
    let label = match footnote_definition.label.to_owned() {
        Some(label) => label,
        None => footnote_definition.identifier.to_owned(),
    };
    html! {<Paper id={footnote_definition.identifier.to_owned()}>{handle_children(&footnote_definition.children)}</Paper>}
}

fn handle_mdx_jsx_flow_element(mdx_jsx_flow_element: &mdast::MdxJsxFlowElement) -> Html {
    html! {<p>{format!("Unhandled mdx_jsx_flow_element:{:?} {}", mdx_jsx_flow_element, mdx_jsx_flow_element.children.len())}</p>}
}

fn handle_list(list: &mdast::List) -> Html {
    if list.ordered {
        let start = list.start.unwrap_or_else(|| 1);
        html! {<ol start={start.to_string()}>{handle_children(&list.children)}</ol>}
    } else {
        html! {<ul>{handle_children(&list.children)}</ul>}
    }
}

fn handle_mdx_js_esm(mdx_js_esm: &mdast::MdxjsEsm) -> Html {
    html! {<p>{format!("Unhandled mdx_js_esm:{:?} {}", mdx_js_esm, mdx_js_esm.value)}</p>}
}

fn handle_toml(toml: &mdast::Toml) -> Html {
    html!(<pre class="language-toml">
        <code class="langugae-toml">{&toml.value}</code>
    </pre>)
}

fn handle_yaml(yaml: &mdast::Yaml) -> Html {
    html!(<pre class="language-yaml">
        <code class="langugae-yaml">{&yaml.value}</code>
    </pre>)
}

fn handle_line_break(line_break: &mdast::Break) -> Html {
    html! {<br />}
}

fn handle_inline_code(inline_code: &mdast::InlineCode) -> Html {
    html! {<code>{&inline_code.value}</code>}
}

fn handle_inline_math(inline_math: &mdast::InlineMath) -> Html {
    html! {<p>{format!("Unhandled inline_math:{:?} {}", inline_math, inline_math.value)}</p>}
}

fn handle_delete(delete: &mdast::Delete) -> Html {
    html! {<del>{handle_children(&delete.children)}</del>}
}

fn handle_emphasis(emphasis: &mdast::Emphasis) -> Html {
    html! {<em>{handle_children(&emphasis.children)}</em>}
}

fn handle_mdx_text_expressions(mdx_text_expressions: &mdast::MdxTextExpression) -> Html {
    html! {format!("Unhandled mdx_text_expressions:{:?} {}", mdx_text_expressions, mdx_text_expressions.value)}
}

fn handle_footnote_reference(footnote_reference: &mdast::FootnoteReference) -> Html {
    let label = match footnote_reference.label.to_owned() {
        Some(label) => label,
        None => footnote_reference.identifier.to_owned(),
    };
    html! {<a href={format!("#{}", footnote_reference.identifier)}>{format!("[{}]", label)}</a>}
}

fn handle_html(html: &mdast::Html) -> Html {
    if let Some(letter) = html.value.chars().nth(1) {
        if html.value.starts_with("<") && html.value.ends_with("/>") && letter.is_uppercase() {
            let mut chars = html.value.chars();
            chars.next();
            chars.next_back();
            chars.next_back();
            let name = chars.as_str().trim().to_string();
            return html!(<DynamicComponent name={name} />);
        }
        if html.value.starts_with("<") && html.value.ends_with(">") && letter.is_uppercase() {
            let mut chars = html.value.chars();
            chars.next();
            chars.next_back();
            let name = chars.as_str().to_string();
            return html!(<DynamicComponent name={name} />);
        }
    }
    html! {<SpanHtmlContent html={html.value.to_owned()} />}
}

fn handle_image(image: &mdast::Image) -> Html {
    let alt = image.alt.to_owned();
    let title = image.title.to_owned().unwrap_or(alt.to_owned());
    html! {<img src={image.url.to_owned()} alt={image.alt.to_owned()} title={title} />}
}

fn handle_image_reference(image_reference: &mdast::ImageReference) -> Html {
    let alt = image_reference.alt.to_owned();
    html! {<img data-imgref={image_reference.identifier.to_owned()} alt={alt} />}
}

fn handle_mdx_jsx_text_element(mdx_jsx_text_element: &mdast::MdxJsxTextElement) -> Html {
    html! {<p>{format!("Unhandled mdx_jsx_text_element:{:?} {}", mdx_jsx_text_element, mdx_jsx_text_element.children.len())}</p>}
}

fn handle_link(link: &mdast::Link) -> Html {
    let mut url = link.url.to_owned();
    let title = link.title.to_owned().unwrap_or_default();
    // The markdown parser has an issue with subject/body variables that contain spaces, so to fix this issue users can use underscores instead of spaces and this process will revert them back to spaces.
    if url.starts_with("mailto:") && url.contains("?") {
        let mut skip = true;
        url = url
            .split("?")
            .map(move |segment| {
                if skip {
                    skip = false;
                    return segment.to_owned();
                }
                segment.replace("_", " ").to_owned()
            })
            .collect::<Vec<String>>()
            .join("?");
    }
    html! {<Link href={url} title={title}>{handle_children(&link.children)}</Link>}
}

fn handle_link_reference(link_reference: &mdast::LinkReference) -> Html {
    let label = link_reference
        .to_owned()
        .label
        .unwrap_or(link_reference.identifier.to_owned());
    html! {<a target="_blank" data-linkref={link_reference.identifier.to_owned()}>{handle_children(&link_reference.children)}</a>}
}

fn handle_strong(strong: &mdast::Strong) -> Html {
    html! {<strong>{handle_children(&strong.children)}</strong>}
}

fn handle_text(text: &mdast::Text) -> Html {
    handle_text_enhancements(&text.value)
}

fn get_meta(meta: &Vec<&str>, pos: usize) -> String {
    if pos == 0 {
        return String::default();
    }
    if meta.len() < pos {
        return String::default();
    }
    let mut value = meta[pos - 1].to_string();
    if value.starts_with('"') {
        value = value[1..].to_string();
    }
    if value.ends_with('"') {
        value = value[..value.len() - 1].to_string();
    }
    value
}

fn handle_code(code: &mdast::Code) -> Html {
    let meta = match &code.meta {
        Some(class) => class.split("\" \"").collect::<Vec<&str>>(),
        None => vec![],
    };
    match &code.lang {
        Some(lang) => match lang.to_lowercase().as_str() {
            "automax" | "automaxcontent" => {
                let class = get_meta(&meta, 1);
                let style = get_meta(&meta, 2);
                let class = classes!(CLASSES_AUTO_MAXCONTENT, "gap-2", class).to_string();
                html!(<Paper class={class} style={style} elevation={ELEVATION_STANDARD}>
                        {render_markdown(&code.value)}
                    </Paper>)
            }
            "cards" => {
                let class = get_meta(&meta, 1);
                let style = get_meta(&meta, 2);
                let class = classes!(CLASSES_CARD_CONTAINER, class).to_string();
                let style = style.to_string();
                html!(<Cards {class} {style}>
                        {render_markdown(&code.value)}
                    </Cards>)
            }
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
            }
            "list" => {
                let is_ordered = !get_meta(&meta, 1).is_empty();
                let class = get_meta(&meta, 2);
                let style = get_meta(&meta, 3);
                let markdown = code.value.to_owned();
                let lines = markdown
                    .lines()
                    .filter(|&x| !x.is_empty())
                    .collect::<Vec<&str>>();
                html!(<List is_ordered={is_ordered} class={class} style={style}>
                        {lines.iter().map(|line|{
                            html!(<li>{render_markdown(line)}</li>)
                        }).collect::<Html>()}
                    </List>)
            }
            "maxauto" | "maxcontentauto" => {
                let class = get_meta(&meta, 1);
                let style = get_meta(&meta, 2);
                let class = classes!(CLASSES_MAXCONTENT_AUTO, "gap-2", class);
                html!(<Paper class={class.to_string()} style={style.to_owned()} elevation={ELEVATION_STANDARD}>
                        {render_markdown(&code.value)}
                    </Paper>)
            }
            "paper" => {
                let class = get_meta(&meta, 1);
                let style = get_meta(&meta, 2);
                html!(<Paper class={class.to_owned()} style={style.to_owned()}>
                        {render_markdown(&code.value)}
                    </Paper>)
            }
            "quote" => {
                let theme = get_meta(&meta, 1);
                let cite = get_meta(&meta, 2);
                let class = get_meta(&meta, 3);
                let style = get_meta(&meta, 4);
                let theme = get_theme(theme.as_str());
                html!(<Quote color={theme} cite={cite.to_string()} class={class.to_owned()} style={style.to_owned()}>
                        {render_markdown(&code.value)}
                    </Quote>)
            }
            "section" => {
                let class = get_meta(&meta, 1);
                let style = get_meta(&meta, 2);
                let class = classes!(CLASSES_PAGE_SECTION, class);
                html!(<Paper class={class.to_string()} style={style.to_owned()} elevation={ELEVATION_STANDARD}>
                        {render_markdown(&code.value)}
                    </Paper>)
            }
            "sidebyside" => {
                let class = get_meta(&meta, 1);
                let style = get_meta(&meta, 2);
                let class = classes!(CLASSES_SIDE_BY_SIDE, "gap-2", class);
                html!(<Paper class={class.to_string()} style={style.to_owned()} elevation={ELEVATION_STANDARD}>
                    {render_markdown(&code.value)}
                    </Paper>)
            }
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
            }
            _ => {
                let class = get_meta(&meta, 1);
                let style = get_meta(&meta, 2);
                let lang_class = format!("language-{}", lang.to_lowercase());
                let class = classes!(lang_class, class);
                html!(<pre languagetag={lang.to_string()} class={class.to_string()} style={style.to_owned()}>
                        <code class={lang_class}>{&code.value}</code>
                    </pre>)
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
    html! {<p>{format!("Unhandled math:{:?}", math)}</p>}
}

fn handle_mdx_flow_expr(mdx_flow_expr: &mdast::MdxFlowExpression) -> Html {
    html! {<p>{format!("Unhandled mdx_flow_expr:{:?}", mdx_flow_expr)}</p>}
}

fn handle_heading(heading: &mdast::Heading) -> Html {
    match heading.depth {
        1 => {
            html! {<h1 class={format!("theme-primary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h1>}
        }
        2 => {
            html! {<h2 class={format!("theme-secondary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h2>}
        }
        3 => {
            html! {<h3 class={format!("theme-tertiary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h3>}
        }
        4 => {
            html! {<h4 class={format!("theme-tertiary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h4>}
        }
        5 => {
            html! {<h5 class={format!("theme-tertiary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h5>}
        }
        _ => {
            html! {<h6 class={format!("theme-tertiary {}", TITLE_CLASSES)}>{handle_children(&heading.children)}</h6>}
        }
    }
}

fn handle_table(table: &mdast::Table) -> Html {
    let mut styles: Vec<String> = vec![];
    let mut count = 0;
    let id = format!("T{}", newid().to_string().replace("-", ""));
    for kind in &table.align {
        count += 1;
        let align = match kind {
            AlignKind::Center => "center",
            AlignKind::Left => "left",
            AlignKind::Right => "right",
            AlignKind::None => "left",
        };
        let style = format!(
            "table#{} tr > td:nth-child({}) {{text-align:{};}}",
            id, count, align
        );
        styles.push(style);
    }
    html! {
        <>
            <StyleContent styles={styles.join("\n")} />
            <table id={id}>{handle_children(&table.children)}</table>
        </>
    }
}

fn handle_thematic_break(thematic_break: &mdast::ThematicBreak) -> Html {
    html! {<hr />}
}

fn handle_table_row(table_row: &mdast::TableRow) -> Html {
    html! {<tr>{handle_children(&table_row.children)}</tr>}
}

fn handle_table_cell(table_cell: &mdast::TableCell) -> Html {
    html! {<td><div>{handle_children(&table_cell.children)}</div></td>}
}

fn handle_list_item(list_item: &mdast::ListItem) -> Html {
    if let Some(checked) = list_item.checked {
        return html!(
            <li>
            if checked {
                <i class="fa-regular fa-square-check theme-success"></i>
            } else {
                <i class="fa-regular fa-square theme-shade"></i>
            }
            {handle_children(&list_item.children)}
            </li>
        );
    }
    html!(<li>{handle_children(&list_item.children)}</li>)
}

fn handle_definition(definition: &mdast::Definition) -> Html {
    let url = definition.url.to_owned();
    let id = definition.identifier.to_owned();
    let title = definition.title.to_owned().unwrap_or(url.to_owned());
    let label = definition.label.to_owned().unwrap_or(title.to_owned());
    let script = format!(
        r#"
        const url = `{}`;
        const id = `{}`;
        const title = `{}`;
        const label = `{}`;
        document.querySelectorAll(`[data-linkref="${{id}}"]`).forEach(a => {{
            a.setAttribute('href', url);
            a.setAttribute('title', title);
        }});
        document.querySelectorAll(`[data-imgref="${{id}}"]`).forEach(img => {{
            img.setAttribute('src', url);
        }});
        "#,
        url, id, title, label
    );
    html! {<JavaScriptContent script={script} delay={10} />}
} //<a data-linkref="gh-checks"><img data-imgref="gh-image" alt="Task Proxy GitHub Actions"></a>

fn handle_paragraph(paragraph: &mdast::Paragraph) -> Html {
    html! {<div>{handle_children(&paragraph.children)}</div>}
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

fn handle_text_enhancements(text: &str) -> Html {
    let mut html = insert_emojis(text);
    html = replace_start_end_deliminators(&text, "==", "<mark>", "</mark>");
    html = replace_start_end_deliminators(&text, "^", "<sup>", "</sup>");
    html = parse_for_icon_refs(&html);
    html! {<SpanHtmlContent html={html} />}
}

fn parse_for_icon_refs(text: &str) -> String {
    let re = Regex::new(r"\!\[([A-Za-z-_ ]*)\]\(([A-Za-z-_ ]+)\)").unwrap();
    let result = re.replace(text, "<i class=\"$2\" title=\"$1\"></i>");
    result.to_string()
}

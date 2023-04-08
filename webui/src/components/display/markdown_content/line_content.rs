use super::*;

pub(super) fn render_line_content(
    is_running: &mut bool,
    line: &str,
    line_type: &MarkdownSegments,
    index: &mut u32,
    lines: &mut Vec<(String, String, MarkdownSegments)>,
) -> Html {
    html!(
        <>
        {match line_type {
            MarkdownSegments::AutoMaxContent(class, style) => {
                *index += 1;
                let class = classes!(CLASSES_AUTO_MAXCONTENT, "gap-2", class);
                html!(<Paper class={class.to_string()} style={style.to_owned()} elevation={ELEVATION_STANDARD}>
                    {render_children(index, lines)}
                </Paper>)
            },
            MarkdownSegments::Cards(class, style) => {
                *index += 1;
                let class = classes!(CLASSES_CARD_CONTAINER, class).to_string();
                let style = style.to_string();
                html!(<Cards {class} {style}>
                    {render_children(index, lines)}
                </Cards>)
            },
            MarkdownSegments::Card(title, width, theme, avatar, link, link_title) => {
                *index += 1;
                let theme = get_theme(theme.as_str());
                html!(<Card title={title.to_owned()}
                    width={*width}
                    theme={theme}
                    avatar={avatar.to_owned()}
                    link={link.to_owned()}
                    link_title={link_title.to_owned()}
                    elevation={ELEVATION_STANDARD}
                    >
                    {render_children(index, lines)}
                </Card>)
            },
            MarkdownSegments::Code(name, class, style) => {
                *index += 1;
                let lang_class = format!("language-{}", name.to_lowercase());
                let class = classes!(lang_class, class);
                html!(<pre languagetag={name.to_string()} class={class.to_string()} style={style.to_owned()}>
                    <code class={lang_class}>{render_code_segments(index, lines)}</code>
                </pre>)
            },
            MarkdownSegments::EndSection => {
                *index += 1;
                *is_running = false;
                html!()
            },
            MarkdownSegments::List(is_ordered, class, style) => {
                *index += 1;
                html!(<List is_ordered={*is_ordered} class={class.to_owned()} style={style.to_owned()}>
                    {render_list(index, lines)}
                </List>)
            },
            MarkdownSegments::Loading(variant, color, size, percent, class, style) => {
                *index += 1;
                let variant = variant.to_owned();
                let color = color.to_owned();
                let size = size.to_owned();
                let class = classes!(CLASSES_CARD_CONTAINER, class).to_string();
                let style = style.to_string();
                let percent = percent.to_owned();
                html!(
                    <Paper>
                        <Loading {variant} {color} {percent} {size} {class} {style} />
                    </Paper>
                )
            },
            MarkdownSegments::MaxContentAuto(class, style) => {
                *index += 1;
                let class = classes!(CLASSES_MAXCONTENT_AUTO, "gap-2", class);
                html!(<Paper class={class.to_string()} style={style.to_owned()} elevation={ELEVATION_STANDARD}>
                    {render_children(index, lines)}
                </Paper>)
            },
            MarkdownSegments::PageSection(class, style) => {
                *index += 1;
                let class = classes!(CLASSES_PAGE_SECTION, class);
                html!(<Paper class={class.to_string()} style={style.to_owned()} elevation={ELEVATION_STANDARD}>
                    {render_children(index, lines)}
                </Paper>)
            },
            MarkdownSegments::Paper(class, style) => {
                *index += 1;
                html!(<Paper class={class.to_owned()} style={style.to_owned()}>
                    {render_children(index, lines)}
                </Paper>)
            },
            MarkdownSegments::Paragraph => {
                *index += 1;
                html!(<p>{render_line(line)}</p>)
            },
            MarkdownSegments::Quote(theme, cite, class, style) => {
                *index += 1;
                let theme = get_theme(theme.as_str());
                html!(<Quote color={theme} cite={cite.to_string()} class={class.to_owned()} style={style.to_owned()}>
                    {render_children(index, lines)}
                </Quote>)
            },
            MarkdownSegments::SideBySide(class, style) => {
                *index += 1;
                let class = classes!(CLASSES_SIDE_BY_SIDE, "gap-2", class);
                html!(<Paper class={class.to_string()} style={style.to_owned()} elevation={ELEVATION_STANDARD}>
                    {render_children(index, lines)}
                </Paper>)
            },
            MarkdownSegments::SideImage(image_pos, src, class, style) => {
                *index += 1;
                let image_pos = match image_pos.as_str() {
                    "right" => LeftOrRight::Right,
                    _ => LeftOrRight::Left,
                };
                html!(<SideImage {image_pos} class={class.to_owned()} src={src.to_owned()} style={style.to_owned()}>
                    <Paper>{render_children(index, lines)}</Paper>
                </SideImage>)
            },
            MarkdownSegments::Table(_columns, _class, _style) => {
                *index += 1;
                /* TODO: Complete me
                let class = classes!(CLASSES_CARD_CONTAINER, class).to_string();
                let style = style.to_string();
                let mut table_columns = Vec::new();
                for column in columns {
                    table_columns.push(TableColumns::<HashMap<u8, String>>::new(
                        "Project".to_string(),
                        |data| {
                            let value = data.get_key_value(&0);
                            html! ()
                        },
                    ));
                }
                */
                // let mut data = Vec::new();
                // data.push(HashMap<String, String>::new());
                html!(
                    // {Table::<HashMap<String, String>>::new().render(data)}
                )
            },
            MarkdownSegments::Title(level) => {
                *index += 1;
                match level {
                    1 => html!(title_primary!(line)),
                    2 => html!(title_secondary!(line)),
                    _ => html!(title_tertiary!(line)),
                }
            },
        }}
        </>
    )
}

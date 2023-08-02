use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabbedContentProps {
    #[prop_or_default]
    pub tabs: Vec<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub elevation: u8,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(TabbedContent)]
pub fn tabbed_content(props: &TabbedContentProps) -> Html {
    let wrapper_classes = &mut Classes::new();
    wrapper_classes.push("tabbed-content d-flex flex-column gap-0");
    let content_classes = &mut Classes::new();
    content_classes.push("tabbed-content-content");
    if !props.class.is_empty() {
        content_classes.push(&props.class);
    }
    if props.elevation > 0 {
        wrapper_classes.push(format!("elevation-{}", props.elevation));
    }
    let show_index = use_state(|| 1);
    let mut index = 0;
    let mut tab_index = 0;
    jslog!("Tabs: {}", *show_index);
    html!(
        <Paper class={wrapper_classes.to_string()}>
            <Paper class="tabbed-content-buttons d-flex flex-row gap-0">
                { for props.tabs.iter().map(|tab|{
                    tab_index += 1;
                    let classes = &mut Classes::new();
                    classes.push("btn tab-button");
                    if tab_index == *show_index {
                        classes.push("theme-active");
                    }
                    let onclick = {
                        let show_index = show_index.clone();
                        let tab_index = tab_index;
                        Callback::from(move |_| {
                            show_index.set(tab_index);
                        })
                    };
                    html!{
                        <Button class={classes.to_string()} onclick={onclick}>{tab}</Button>
                    }
                })}
            </Paper>
            <Paper class={content_classes.to_string()}>
                { for props.children.iter().filter(|_|{
                    index += 1;
                    index==*show_index
                }) }
            </Paper>
        </Paper>
    )
}

use crate::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct DropdownOption {
    pub value: String,
    pub display: fn() -> Html,
}

impl DropdownOption {
    pub fn new(value: &str, display: fn() -> Html) -> Self {
        DropdownOption {
            value: value.to_string(),
            display,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    pub options: Vec<DropdownOption>,
    pub selected: UseStateHandle<String>,
    #[prop_or_default]
    pub t: String,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub key: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or(Theme::None)]
    pub color: Theme,
    #[prop_or_default]
    pub cache_id: Option<String>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub onchange: Option<Callback<String>>,
    #[prop_or_default]
    pub start_icon: Option<IconOptions>,
    #[prop_or_default]
    pub end_icon: Option<IconOptions>,
}

#[function_component(Dropdown)]
pub fn input_dropdown(props: &DropdownProps) -> Html {
    if props.options.len() == 0 {
        return html!();
    }
    let is_open = use_state(|| false);

    let selected_value = props.selected.deref().to_owned();
    let selected_option = match props
        .options
        .iter()
        .find(|value| value.value == selected_value)
    {
        Some(value) => value,
        None => props.options.first().unwrap(),
    };

    if selected_value != selected_option.value {
        props.selected.set(selected_option.value.to_owned());
        return html!();
    }

    // Callback to toggle the dropdown
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let selected = props.selected.to_owned();
    html! {
        <div class={classes!("dropdown", props.class, if *is_open { "show" } else { "" })}>
            <Button color={props.color.clone()} class="btn dropdown-toggle" onclick={onclick}>
                {if let Some(start_icon) = &props.start_icon {
                    html!{<Icon
                            icon={start_icon.icon.to_owned()}
                            title={start_icon.title.to_owned()}
                            elevation={start_icon.elevation.to_owned()}
                            color={start_icon.color.to_owned()}
                            style={start_icon.style.to_owned()}
                            />}
                } else { html!() }}
                {(selected_option.display)()}
                {if let Some(end_icon) = &props.end_icon {
                    html!{<Icon
                            icon={end_icon.icon.to_owned()}
                            title={end_icon.title.to_owned()}
                            elevation={end_icon.elevation.to_owned()}
                            color={end_icon.color.to_owned()}
                            style={end_icon.style.to_owned()}
                            />}
                } else { html!() }}
            </Button>
            { if *is_open {
                html!{<ul class="dropdown-menu">
                    {props.options.iter().map(|option|{
                        let onclick = {
                            let selected = selected.to_owned();
                            let value = option.value.to_owned();
                            let is_open = is_open.clone();
                            Callback::from(move |_| {
                                selected.set(value.to_owned());
                                is_open.set(false);
                            })
                        };
                        let classes = classes!(if option.value == *selected { "selected" } else { "" });
                        html!{<li class={classes} {onclick}>{(option.display)()}</li>}
                    }).collect::<Html>()}
                </ul>}
            } else { html!() }}
        </div>
    }
}

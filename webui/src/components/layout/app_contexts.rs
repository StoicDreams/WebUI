use crate::*;

/// Properties for app drawer components
#[derive(Properties, PartialEq)]
pub(crate) struct AppContextsProps {
    pub children: Children,
    pub app_config: AppConfig,
}

#[derive(Clone, PartialEq)]
pub struct Agents {}

pub(crate) struct AppContexts {
    agents: Agents,
}

impl Component for AppContexts {
    type Message = AppStateReceiverMessage;
    type Properties = AppContextsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { agents: Agents {} }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppStateReceiverMessage::AppStateMessage(request) => (),
            AppStateReceiverMessage::None => (),
        };
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
            <ContextProvider<AppConfig> context={props.app_config.clone()}>
            <ContextProvider<Agents> context={self.agents.clone()}>
                { for props.children.iter() }
            </ContextProvider<Agents>>
            </ContextProvider<AppConfig>>
        }
    }
}

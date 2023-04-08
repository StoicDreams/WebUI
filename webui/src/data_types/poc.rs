use crate::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
struct PocInfo {
    id: Uuid,
    content: Rc<dyn Fn() -> Html>,
}

impl PocInfo {
    fn new<F>(content: F) -> Self
    where
        F: Fn() -> Html + 'static,
    {
        Self {
            id: newid(),
            content: Rc::new(content),
        }
    }
}

impl HtmlRunner for PocInfo {
    fn run(&self) -> Html {
        let unbox = self.content.as_ref();
        html!(<>{unbox()}</>)
    }
}

impl PartialEq for PocInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl core::fmt::Debug for PocInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PocInfo")
            .field("id", &self.id)
            .field("content", &"dyn Fn() -> Html")
            .finish()
    }
}

#[derive(Debug, Properties, PartialEq)]
struct PocProps {
    pub children: Children,
    pub info: PocInfo,
}

#[function_component(PocComponent)]
fn poc_component(props: &PocProps) -> Html {
    let content = &props.info;
    html! {
        <>
            { for props.children.iter() }
            {content.run()}
        </>
    }
}

#[function_component(PocApp)]
fn poc_app() -> Html {
    // An external variable that we want to pass into a method that builds html
    let message = "This is an external message!";
    // A parameter object that we pass in an Html method that can output our external message
    let info = PocInfo::new(move || html!(message));
    html! {
        <PocComponent {info}>
            {"Hello World!"}
        </PocComponent>
    }
}

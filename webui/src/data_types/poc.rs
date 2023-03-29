use crate::prelude::*;

struct PocInfo<'a> {
    id: Uuid,
    content: Box<dyn Fn() -> Html + 'a>,
}

impl<'a> PocInfo<'a> {
    fn new<F>(content: F) -> Self
    where
        F: Fn() -> Html + 'a,
    {
        Self {
            id: newid(),
            content: Box::new(content),
        }
    }

    fn run(&self) -> Html {
        let unbox = &*self.content.as_ref();
        html!(<>{unbox()}</>)
    }
}

impl<'a> PartialEq for PocInfo<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> core::fmt::Debug for PocInfo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PocInfo")
            .field("id", &self.id)
            .field("content", &"dyn Fn() -> Html")
            .finish()
    }
}

#[derive(Debug, Properties, PartialEq)]
struct PocProps<'a> {
    pub children: Children,
    pub info: PocInfo<'a>,
}

#[function_component(PocComponent)]
fn poc_component(props: &PocProps<'static>) -> Html {
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

// use std::rc::Rc;
use crate::prelude::*;

pub trait HtmlRunner {
    fn run(&self) -> Html;
}

#[derive(Clone)]
pub struct DynHtml {
    id: Uuid,
    content: Rc<dyn Fn() -> Html>,
}

impl DynHtml {
    pub fn new<F>(content: F) -> Self
    where
        F: Fn() -> Html + 'static,
    {
        Self {
            id: newid(),
            content: Rc::new(content),
        }
    }
}

impl HtmlRunner for DynHtml {
    fn run(&self) -> Html {
        let unbox = self.content.as_ref();
        html!(<>{unbox()}</>)
    }
}

impl PartialEq for DynHtml {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl core::fmt::Debug for DynHtml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynHtml")
            .field("id", &self.id)
            .field("content", &"dyn Fn() -> Html")
            .finish()
    }
}

impl Default for DynHtml {
    fn default() -> Self {
        Self {
            id: Default::default(),
            content: Rc::new(|| html!()),
        }
    }
}

#[derive(Clone)]
pub struct DynHtmlLT<'a> {
    id: Uuid,
    content: Rc<dyn Fn() -> Html + 'a>,
}

impl<'a> DynHtmlLT<'a> {
    pub fn new<F>(content: F) -> Self
    where
        F: Fn() -> Html + 'a,
    {
        Self {
            id: newid(),
            content: Rc::new(content),
        }
    }
}

impl<'a> HtmlRunner for DynHtmlLT<'a> {
    fn run(&self) -> Html {
        let unbox = self.content.as_ref();
        html!(<>{unbox()}</>)
    }
}

impl<'a> PartialEq for DynHtmlLT<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> core::fmt::Debug for DynHtmlLT<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynHtmlLT")
            .field("id", &self.id)
            .field("content", &"dyn Fn() -> Html")
            .finish()
    }
}

impl<'a> Default for DynHtmlLT<'a> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            content: Rc::new(|| html!()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    pub(crate) fn hello_world() -> Html {
        html! (
            <Paper class={CLASSES_PAGE_SECTION}>
                <PocApp />
            </Paper>
        )
    }

    #[test]
    fn test_hello_world() {
        _ = hello_world();
    }

    #[derive(Debug, Properties, PartialEq)]
    struct TestProps {
        pub children: Children,
        pub info: DynHtml,
    }

    #[function_component(TestComponent)]
    fn test_component(props: &TestProps) -> Html {
        let content = &props.info;
        html! {
            <>
                { for props.children.iter() }
                {content.run()}
            </>
        }
    }

    #[function_component(PocApp)]
    fn test_app() -> Html {
        // An external variable that we want to pass into a method that builds html
        let message = "This is an external message!";
        // A parameter object that we pass in an Html method that can output our external message
        let info = DynHtml::new(move || html!(message));
        html! {
            <TestComponent {info}>
                {"Hello World!"}
            </TestComponent>
        }
    }
}

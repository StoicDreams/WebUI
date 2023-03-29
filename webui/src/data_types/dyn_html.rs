use crate::prelude::*;

#[derive(Clone)]
struct DynHtml<'a> {
    id: Uuid,
    content: Rc<dyn Fn() -> Html + 'a>,
}

impl<'a> DynHtml<'a> {
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

impl<'a> PartialEq for DynHtml<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> core::fmt::Debug for DynHtml<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynHtml")
            .field("id", &self.id)
            .field("content", &"dyn Fn() -> Html")
            .finish()
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
        assert!(true);
    }

	#[derive(Debug, Properties, PartialEq)]
	struct TestProps<'a> {
		pub children: Children,
		pub info: DynHtml<'a>,
	}

	#[function_component(TestComponent)]
	fn test_component(props: &TestProps<'static>) -> Html {
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

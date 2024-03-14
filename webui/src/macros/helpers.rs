use crate::prelude::*;

#[macro_export]
macro_rules! define_fn_struct {
    ($name:ident, $returns:ty, { $($parameter:ident : $ty:ty),* }) => {
        #[derive(Clone)]
        pub struct $name {
            id: Uuid,
            callback: Arc<dyn Fn($( $ty ),*) -> $returns>,
        }

        impl $name {
            pub fn new<F>(callback: F) -> Self
            where
                F: Fn($( $ty ),*) -> $returns + 'static,
            {
                Self {
                    id: newid(),
                    callback: Arc::new(callback),
                }
            }
            pub fn run(&self, $( $parameter : $ty ),*) -> $returns {
                let unbox = self.callback.as_ref();
                unbox($( $parameter ),*)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("DynContextsHtml")
                    .field("id", &self.id)
                    .field("callback", &"dyn Fn($( $parameter ),*) -> $returns")
                    .finish()
            }
        }
    };
}

/// Helper macro to define a struct that derives Debug, Clone, Deserialize, Serialize, Default, and PartialEq and can be instantiated with a new(...) initializer method.
#[macro_export]
macro_rules! define_form {
    ($name:ident, { $($field:ident : $ty:ty),* }) => {
        #[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
        pub struct $name {
            $(pub $field : $ty),*,
        }

        impl $name {
            #[allow(clippy::too_many_arguments)]
            pub fn new($( $field : $ty ),*) -> Self {
                Self {
                    $( $field ),*,
                }
            }
        }
    }
}

/// Helper macro for rendering markdown content without using the MarkdownContent boilerplate.
#[macro_export]
macro_rules! markdown {
    ($markdown:expr) => {
        html!(<MarkdownContent markdown={$markdown} />)
    }
}

/// Helper macro for loading and rendering a markdown file without using the MarkdownContent boilerplate.
#[macro_export]
macro_rules! markdownfile {
    ($href:expr) => {
        html!(<MarkdownContent href={$href} />)
    }
}

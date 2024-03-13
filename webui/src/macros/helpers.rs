use crate::prelude::*;

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

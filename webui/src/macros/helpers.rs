use crate::prelude::*;

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

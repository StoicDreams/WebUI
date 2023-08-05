use crate::prelude::*;

/// Macro for opening an alert dialog.
#[macro_export]
macro_rules! alert {
    ( $contexts:expr, $title:expr, $html:expr) => {
        $contexts.drawer.set(
            Dialog::alert(
                |_| $title.to_string(),
                DynContextsHtml::new(move |_| html!($html)),
            )
            .message(),
        );
    };
}

/// Macro for opening an regular dialog.
#[macro_export]
macro_rules! dialog {
    ( $contexts:expr, $title:expr, $html:expr) => {
        $contexts.drawer.set(
            Dialog::new(
                |_| $title.to_string(),
                DynContextsHtml::new(move |_| html!($html)),
            )
            .message(),
        );
    };
}

use crate::prelude::*;

/// Macro for spawning an async process
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

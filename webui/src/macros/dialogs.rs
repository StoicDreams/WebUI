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

/// Macro for opening an regular dialog.
#[macro_export]
macro_rules! drawer {
    ( $title:expr, $btn_display:expr, $content:expr) => {
        DrawerToggleInfo::builder(
            |_contexts: Contexts| String::from($title),
            |_contexts: Contexts| $btn_display,
            DynContextsHtml::new($content),
        )
    };
    ( $title:expr, $btn_display:expr, $content:expr, $direction: expr) => {
        DrawerToggleInfo::builder(
            |_contexts: Contexts| String::from($title),
            |_contexts: Contexts| $btn_display,
            DynContextsHtml::new($content),
        )
        .set_drawer($direction)
    };
}

#[macro_export]
macro_rules! drawer_contexts {
    ( $title:expr, $btn_display:expr, $content:expr) => {
        DrawerToggleInfo::builder($title, $btn_display, DynContextsHtml::new($content))
    };
}

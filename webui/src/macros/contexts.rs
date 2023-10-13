use crate::prelude::*;

/// Macro for navigating to a new page.
///
/// Example
/// ```rust,ignore
/// nav_to!(contexts, "/");
/// ```
#[macro_export]
macro_rules! nav_to {
    ( $contexts:expr, $local_path:expr) => {
        let navigation = $contexts.nav.clone();
        let mymessage = NavigationMessage::PathUpdate(String::from($local_path.to_owned()));
        navigation.set(mymessage);
    };
}

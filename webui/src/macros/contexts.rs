use crate::prelude::*;

/// Macro for navigating to a new page.
#[macro_export]
macro_rules! nav_to {
    ( $contexts:expr, $local_path:expr) => {
        let navigation = $contexts.nav.clone();
        let mymessage = NavigationMessage::PathUpdate($local_path.clone());
        navigation.set(mymessage);
    };
}

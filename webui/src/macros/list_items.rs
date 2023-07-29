/// Macro for expanding string arguments into list items
///
/// example
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: Contexts) -> Html {
///     html! {
///         {list_items!(
///             "This is the first list item.",
///             "This is the second list item. With multiple sentences."
///         )}
///     }
/// }
/// ```
#[macro_export]
macro_rules! list_items {
    ( $($x:expr ),* ) => {
        html! {
            <>
            $(
                <li>{$x}</li>
            )*
            </>
        }
    };
}

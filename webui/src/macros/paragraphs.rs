/// Macro for expanding string arguments into paragraphs
///
/// example
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: &Contexts) -> Html {
///     html! {
///         {paragraphs!(
///             "This is the first paragraph.",
///             "This is the second paragraph. With multiple sentences."
///         )}
///     }
/// }
/// ```
#[macro_export]
macro_rules! paragraphs {
    ( $($x:expr ),* ) => {
        html! {
            <>
            $(
                <p>{$x}</p>
            )*
            </>
        }
    };
}

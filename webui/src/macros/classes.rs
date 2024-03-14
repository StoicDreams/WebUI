/// Macro for consolidating classes into single string
///
/// example
/// ```rust
/// use webui::prelude::*;
///
/// fn page(contexts: &Contexts) -> Html {
///     let classes = classes!(CLASSES_PAGE_SECTION, CLASSES_SIDE_BY_SIDE);
///
///     html! {
///         <div class={classes.to_string()}>
///             <div>{"Left"}</div>
///             <div>{"Right"}</div>
///         </div>
///     }
/// }
/// ```
#[macro_export]
macro_rules! classes {
    ( $($x:expr ),* ) => {
        {
            let classes = &mut Classes::new();
            $(
                classes.push({$x.to_string()});
            )*
            classes.to_owned()
        }
    };
}

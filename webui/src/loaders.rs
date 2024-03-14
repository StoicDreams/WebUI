use crate::prelude::*;

/// Process for loading data at start of application/website.
/// Loads are only applied once.
#[function_component(Loaders)]
pub(crate) fn loaders() -> Html {
    let data_loaded = use_state(|| 0u8);
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    if *data_loaded & 2 == 0 {
        data_loaded.clone().set(2);
        spawn_async!({
            #[cfg(feature = "myfi")]
            {
                let is_loaded = *data_loaded;
                let contexts = contexts.clone();
                if is_loaded & 1 == 0 {
                    data_loaded.set(1 | *data_loaded);
                    myfi_loader(&contexts).await;
                }
            }
        });
    }
    html!()
}

#[cfg(feature = "myfi")]
pub(crate) async fn myfi_loader(contexts: &Contexts) {
    let context = contexts.clone();
    let user_state = context.user.clone();
    let roles_state = context.user_roles.clone();
    myfi_get_my_info(user_state, roles_state).await;
}

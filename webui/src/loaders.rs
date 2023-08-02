use crate::prelude::*;

#[function_component(Loaders)]
pub(crate) fn loaders() -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    #[cfg(feature = "myfi")]
    {
        myfi_loader(contexts);
    }
    html!()
}

#[cfg(feature = "myfi")]
pub(crate) fn myfi_loader(contexts: Contexts) -> Html {
    let context = contexts.clone();
    spawn_async!({
        myfi_get_session().await;
        let user_state = context.user.clone();
        myfi_get_my_info(user_state).await;
    });
    html!()
}

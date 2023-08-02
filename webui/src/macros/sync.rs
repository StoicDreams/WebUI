/// Macro for running a method after a timeout
#[macro_export]
macro_rules! set_timeout {
    ( $t:expr, $x:expr ) => {
        let f: Box<dyn FnMut()> = Box::new(move || {
            $x;
        });
        let callback = Closure::wrap(f);
        _ = set_timeout(callback.as_ref().unchecked_ref(), $t);
        callback.forget();
    };
}

/// Macro for spawning an async process
#[macro_export]
macro_rules! spawn_async {
    ( $x:expr ) => {
        wasm_bindgen_futures::spawn_local(async move {
            {
                $x
            }
        });
    };
}

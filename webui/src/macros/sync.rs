/// Macro for running a method after a timeout
#[macro_export]
macro_rules! set_timeout {
    ( $t:expr, $x:expr ) => {
        let callback = Closure::wrap(Box::new($x) as Box<dyn FnMut()>);
        _ = set_timeout(callback.as_ref().unchecked_ref(), $t);
        callback.forget();
    };
}

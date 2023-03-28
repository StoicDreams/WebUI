use crate::GlobalData;
use std::{fmt::Debug, ops::Deref, rc::Rc};

pub fn get_input_state<T, F>(key: &str, default_handler: F) -> InputStateHandler<T>
where
    T: for<'a> serde::Deserialize<'a>,
    T: serde::Serialize,
    T: Debug,
    T: Clone,
    T: PartialEq,
    F: FnOnce() -> T,
{
    use_input_state(key, default_handler, None)
}

pub fn use_input_state<T, F>(
    key: &str,
    default_handler: F,
    change_trigger: Option<Rc<dyn Fn(&T) -> ()>>,
) -> InputStateHandler<T>
where
    T: for<'a> serde::Deserialize<'a>,
    T: serde::Serialize,
    T: Debug,
    T: Clone,
    T: PartialEq,
    F: FnOnce() -> T,
{
    let value = match GlobalData::get_data::<T>(&key) {
        Ok(val) => val,
        Err(_) => default_handler(),
    };
    let _ = GlobalData::set_data::<T>(&key, value.clone());
    InputStateHandler {
        key: key.to_string(),
        change_trigger: change_trigger.unwrap_or(Rc::new(|_| {})),
        value,
    }
}

// #[derive(Debug, PartialEq, Clone)]
#[derive(Clone)]
pub struct InputStateHandler<T>
where
    T: for<'a> serde::Deserialize<'a>,
    T: serde::Serialize,
    T: Debug,
    T: Clone,
    T: PartialEq,
{
    key: String,
    value: T,
    change_trigger: Rc<dyn Fn(&T) -> ()>,
}

impl<T> PartialEq for InputStateHandler<T>
where
    T: for<'a> serde::Deserialize<'a>,
    T: serde::Serialize,
    T: Debug,
    T: Clone,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> InputStateHandler<T>
where
    T: for<'a> serde::Deserialize<'a>,
    T: serde::Serialize,
    T: Debug,
    T: Clone,
    T: PartialEq,
{
    pub fn set(&mut self, value: T) -> () {
        let _ = GlobalData::set_data::<T>(&self.key, value.clone());
        (&self.change_trigger)(&value);
    }
    pub fn get(&self) -> T {
        match GlobalData::get_data(&self.key) {
            Ok(value) => value,
            Err(_) => {
                let _ = GlobalData::set_data::<T>(&self.key, self.value.clone());
                self.value.clone()
            }
        }
    }
    pub fn to_string(&self) -> String {
        // let test = use_force_update();
        format!("{:?}", self.get())
    }
}

impl<T> Deref for InputStateHandler<T>
where
    T: for<'a> serde::Deserialize<'a>,
    T: serde::Serialize,
    T: Debug,
    T: Clone,
    T: PartialEq,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self).value
    }
}

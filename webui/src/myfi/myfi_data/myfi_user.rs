use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyFiUser {
    pub id: String,
    pub display_name: String,
    pub roles: u64,
}

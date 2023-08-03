use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyFiUser {
    pub id: Option<String>,
    pub display_name: String,
    pub roles: u64,
}

impl Default for MyFiUser {
    fn default() -> Self {
        Self {
            id: None,
            display_name: "Guest".to_string(),
            roles: 0,
        }
    }
}

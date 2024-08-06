use serde::{Deserialize, Serialize};

pub mod dao_def;
pub mod do_def;
pub mod logic_def;
pub mod svc_def;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct EntityField {
    pub name: String,
    pub sname: String,
    pub comment: String,
    pub type_name: String,
}

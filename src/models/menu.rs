use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MenuDinamicoItem {
    pub parent_id: i32,
    pub parent_name: String,
    pub modulo_id: i32,
    pub modulo_name: String,
    pub bitagregar: bool,
    pub biteditar: bool,
    pub biteliminar: bool,
    pub bitconsulta: bool,
}
pub mod perfil;
pub mod usuario;
pub mod permisos;
pub mod menu;

pub use perfil::*;
pub use usuario::*;
pub use permisos::*;

pub use crate::middleware::auth::Claims;
pub mod modulo;
pub use modulo::*;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct MisPermisos {
    pub strnombremodulo: String,
    pub bitconsulta: Option<bool>,
    pub bitagregar: Option<bool>,
    pub biteditar: Option<bool>,
    pub biteliminar: Option<bool>,
}
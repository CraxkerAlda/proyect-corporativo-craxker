use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PermisosModulo {
    pub idmodulo: i32,
    pub strnombremodulo: String,
    pub bitagregar: bool,
    pub biteditar: bool,
    pub bitconsulta: bool,
    pub biteliminar: bool,
    pub bitdetalle: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuDinamico {
    pub nombre_menu: String,
    pub modulos: Vec<PermisosModulo>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PermisosPerfil {
    pub id: i32,
    pub idmodulo: i32,
    pub idperfil: i32,
    pub bitagregar: bool,
    pub biteditar: bool,
    pub bitconsulta: bool,
    pub biteliminar: bool,
    pub bitdetalle: bool,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarPermiso {
    pub idmodulo: i32,
    pub idperfil: i32,
    pub bitagregar: bool,
    pub biteditar: bool,
    pub bitconsulta: bool,
    pub biteliminar: bool,
    pub bitdetalle: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MisPermisos {
    pub strnombremodulo: String,
    pub bitconsulta: Option<bool>,
    pub bitagregar: Option<bool>,
    pub biteditar: Option<bool>,
    pub biteliminar: Option<bool>,
}
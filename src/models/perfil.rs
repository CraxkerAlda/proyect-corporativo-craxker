use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Perfil {
    pub id: i32,
    pub strnombreperfil: String,
    pub bitadministrador: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NuevoPerfil {
    pub strnombreperfil: String,
    pub bitadministrador: bool,
}
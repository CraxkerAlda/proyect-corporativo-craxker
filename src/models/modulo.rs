use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Modulo {
    pub id: i32,
    pub strnombremodulo: String,
}

#[derive(Debug, Deserialize)]
pub struct NuevoModulo {
    pub strnombremodulo: String,
}
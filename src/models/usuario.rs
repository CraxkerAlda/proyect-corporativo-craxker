use serde::{Deserialize};

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: i32,
    pub strnombreusuario: String,
    pub idperfil: i32,
    pub strpwd: String,
    pub idestadousuario: i16,
    pub strcorreo: String,
    pub strnumerocelular: Option<String>,
    pub strimagenpath: String,
    pub fechacreacion: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct RegistroUsuario {
    pub strnombreusuario: String,
    pub idperfil: i32,
    pub strpwd: String,
    pub strcorreo: String,
    pub strnumerocelular: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub nombre: String,
    pub perfil_id: i32,
    pub imagen: String,
}
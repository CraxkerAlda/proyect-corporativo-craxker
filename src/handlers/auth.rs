use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use bcrypt::verify;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::models::{Usuario, Claims, LoginResponse};

#[derive(Deserialize)]
pub struct LoginPayload {
    pub usuario: String,
    pub password: String,
    pub captcha_token: String,
}

pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    if !validar_recaptcha(&payload.captcha_token).await {
        return (StatusCode::BAD_REQUEST, "Captcha inválido o expirado").into_response();
    }

    let user_row = sqlx::query_as!(
        Usuario,
        r#"SELECT 
            id, strnombreusuario, idperfil, strpwd, 
            idestadousuario as "idestadousuario!", 
            strcorreo, strnumerocelular, 
            strimagenpath as "strimagenpath!", 
            fechacreacion as "fechacreacion!" 
        FROM usuarios WHERE strnombreusuario = $1"#,
        payload.usuario
    )
    .fetch_optional(&pool)
    .await;

    match user_row {
        Ok(Some(user)) => {
            if user.idestadousuario != 1 {
                return (StatusCode::FORBIDDEN, "Usuario inactivo, contacte al administrador").into_response();
            }

            let valid_pass = verify(&payload.password, &user.strpwd).unwrap_or(false);

            if valid_pass {
                let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET no configurado");
                let expiracion = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as usize + 3600; 

                let claims = Claims {
                    sub: user.strnombreusuario.clone(),
                    id_user: user.id,
                    id_perfil: user.idperfil,
                    exp: expiracion,
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(secret.as_ref()),
                ).unwrap_or_default();

                Json(LoginResponse {
                    token,
                    nombre: user.strnombreusuario,
                    perfil_id: user.idperfil,
                    imagen: user.strimagenpath,
                }).into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "Contraseña incorrecta").into_response()
            }
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, "El usuario no existe").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error en la base de datos").into_response(),
    }
}

async fn validar_recaptcha(token: &str) -> bool {
    let secret_key = std::env::var("RECAPTCHA_SECRET_KEY").unwrap_or_else(|_| "".into());
    
    if secret_key.is_empty() {
        println!("ERROR: RECAPTCHA_SECRET_KEY no configurada en .env");
        return false;
    }

    let client = reqwest::Client::new();
    let res = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&[
            ("secret", secret_key),
            ("response", token.to_string()),
        ])
        .send()
        .await;

    match res {
        Ok(response) => {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                return json["success"].as_bool().unwrap_or(false);
            }
            false
        },
        Err(_) => false,
    }
}
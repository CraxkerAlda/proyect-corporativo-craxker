use axum::{
    extract::{Request, State, FromRequestParts},
    http::{header, StatusCode}, 
    middleware::Next,
    response::IntoResponse,
};

use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,     
    pub id_user: i32,    
    pub id_perfil: i32,   
    pub exp: usize,    
}

#[derive(Clone, Copy)]
pub enum Accion {
    Agregar,
    Editar,
    Consulta,
    Eliminar,
    Detalle,
}

pub async fn auth_middleware(
    State(pool): State<PgPool>,
    modulo_req: String,
    accion_req: Accion,
    req: Request,
    next: Next,
) -> impl IntoResponse { 
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    let token = match auth_header {
        Some(t) => t,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_default();
    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let claims = token_data.claims;
    let columna_bit = match accion_req {
        Accion::Agregar => "bitAgregar",
        Accion::Editar => "bitEditar",
        Accion::Consulta => "bitConsulta",
        Accion::Eliminar => "bitEliminar",
        Accion::Detalle => "bitDetalle",
    };

    let query = format!(
        "SELECT {} FROM permisos_perfil pp 
         JOIN modulos m ON pp.idModulo = m.id 
         WHERE pp.idPerfil = $1 AND m.strNombreModulo = $2",
        columna_bit
    );

    let tiene_permiso: bool = sqlx::query_scalar(&query)
        .bind(claims.id_perfil)
        .bind(&modulo_req)
        .fetch_optional(&pool)
        .await
        .unwrap_or(Some(false))
        .unwrap_or(false);

    if tiene_permiso {
        next.run(req).await.into_response()
    } else {
        StatusCode::FORBIDDEN.into_response()
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        let token = auth_header.ok_or(StatusCode::UNAUTHORIZED)?;
        let secret = std::env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        ).map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(token_data.claims)
    }
}
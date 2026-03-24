use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::{Perfil, NuevoPerfil};

pub async fn listar_perfiles(
    State(pool): State<PgPool>
) -> impl IntoResponse {
    let resultado = sqlx::query_as!(
        Perfil,
        "SELECT id, strnombreperfil, bitadministrador as \"bitadministrador!\" FROM perfiles ORDER BY strnombreperfil ASC"
    )
    .fetch_all(&pool)
    .await;

    match resultado {
        Ok(perfiles) => (StatusCode::OK, Json(perfiles)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response(),
    }
}

pub async fn crear_perfil(
    State(pool): State<PgPool>,
    Json(payload): Json<NuevoPerfil>, 
) -> impl IntoResponse {
    let resultado = sqlx::query!(
        "INSERT INTO perfiles (strnombreperfil, bitadministrador) VALUES ($1, $2) RETURNING id",
        payload.strnombreperfil.trim().to_uppercase(), 
        payload.bitadministrador
    )
    .fetch_one(&pool)
    .await;

    match resultado {
        Ok(_) => (StatusCode::CREATED, "Perfil creado correctamente").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error DB: {}", e)).into_response(),
    }
}

pub async fn actualizar_perfil(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<NuevoPerfil>,
) -> impl IntoResponse {
    let resultado = sqlx::query!(
        "UPDATE perfiles SET strnombreperfil = $1, bitadministrador = $2 WHERE id = $3",
        payload.strnombreperfil.trim().to_uppercase(),
        payload.bitadministrador,
        id
    )
    .execute(&pool)
    .await;

    match resultado {
        Ok(res) if res.rows_affected() > 0 => (StatusCode::OK, "Perfil actualizado").into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "Perfil no encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al actualizar").into_response(),
    }
}

pub async fn eliminar_perfil(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let resultado = sqlx::query!("DELETE FROM perfiles WHERE id = $1", id)
        .execute(&pool)
    .await;

    match resultado {
        Ok(res) if res.rows_affected() > 0 => StatusCode::OK.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => (StatusCode::CONFLICT, "No se puede eliminar: el perfil está en uso").into_response(),
    }
}
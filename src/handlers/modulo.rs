use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::{Modulo, NuevoModulo};

pub async fn listar_modulos(State(pool): State<PgPool>) -> impl IntoResponse {
    let res = sqlx::query_as!(Modulo, "SELECT id, strnombremodulo FROM modulos ORDER BY id ASC")
        .fetch_all(&pool)
        .await;

    match res {
        Ok(m) => (StatusCode::OK, Json(m)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al consultar módulos").into_response(),
    }
}

pub async fn crear_modulo(State(pool): State<PgPool>, Json(payload): Json<NuevoModulo>) -> impl IntoResponse {
    let nombre_formateado = payload.strnombremodulo.to_uppercase();
    let nombre_final = nombre_formateado.trim();

    let res = sqlx::query!(
        "INSERT INTO modulos (strnombremodulo) VALUES ($1)",
        nombre_final
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, "Módulo registrado").into_response(),
        Err(_) => (StatusCode::BAD_REQUEST, "El módulo ya existe o hay un error").into_response(),
    }
}

pub async fn eliminar_modulo(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let res = sqlx::query!("DELETE FROM modulos WHERE id = $1", id)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => (StatusCode::CONFLICT, "No se puede eliminar: tiene permisos asociados").into_response(),
    }
}

pub async fn listar_modulos_json(
    State(pool): State<PgPool>
) -> impl IntoResponse {
    let resultado = sqlx::query_as!(
        crate::models::Modulo,
        "SELECT id, strnombremodulo FROM modulos ORDER BY strnombremodulo ASC"
    )
    .fetch_all(&pool)
    .await;

    match resultado {
        Ok(m) => (StatusCode::OK, Json(m)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al cargar menú").into_response(),
    }
}


pub async fn actualizar_modulo(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<NuevoModulo>,
) -> impl IntoResponse {
    let nombre_final = payload.strnombremodulo.to_uppercase();
    let nombre_final = nombre_final.trim();

    let res = sqlx::query!(
        "UPDATE modulos SET strNombreModulo = $1 WHERE id = $2",
        nombre_final,
        id
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            println!("Error SQL: {:?}", e);
            (StatusCode::BAD_REQUEST, "Error al actualizar módulo").into_response()
        }
    }
}
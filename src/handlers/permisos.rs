use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::middleware::auth::Claims;
use crate::models::{PermisosPerfil, ActualizarPermiso}; 

pub async fn obtener_permisos_por_perfil(
    State(pool): State<PgPool>,
    Path(id_perfil): Path<i32>,
) -> impl IntoResponse {
    let res = sqlx::query_as!(
        PermisosPerfil,
        r#"SELECT id, idmodulo, idperfil, 
            bitagregar as "bitagregar!", 
            biteditar as "biteditar!", 
            bitconsulta as "bitconsulta!", 
            biteliminar as "biteliminar!", 
            bitdetalle as "bitdetalle!" 
        FROM permisos_perfil WHERE idperfil = $1"#,
        id_perfil
        )
        .fetch_all(&pool)
        .await;

    match res {
        Ok(permisos) => (StatusCode::OK, Json(permisos)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al consultar permisos").into_response(),
    }
}

pub async fn guardar_permiso(
    State(pool): State<PgPool>,
    Json(payload): Json<ActualizarPermiso>,
) -> impl IntoResponse {
    let res = sqlx::query!(
        r#"
        INSERT INTO permisos_perfil (idmodulo, idperfil, bitagregar, biteditar, bitconsulta, biteliminar, bitdetalle)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (idmodulo, idperfil) 
        DO UPDATE SET 
            bitagregar = EXCLUDED.bitagregar,
            biteditar = EXCLUDED.biteditar,
            bitconsulta = EXCLUDED.bitconsulta,
            biteliminar = EXCLUDED.biteliminar,
            bitdetalle = EXCLUDED.bitdetalle
        "#,
        payload.idmodulo,
        payload.idperfil,
        payload.bitagregar,
        payload.biteditar,
        payload.bitconsulta,
        payload.biteliminar,
        payload.bitdetalle
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::OK, "Permiso actualizado correctamente").into_response(),
        Err(e) => {
            println!("Error SQL: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al guardar permisos").into_response()
        }
    }
}


pub async fn obtener_mis_permisos(
    State(pool): State<PgPool>,
    claims: Claims,
) -> impl IntoResponse {
    let permisos = sqlx::query_as!(
        crate::models::MisPermisos,
        r#"SELECT m.strNombreModulo, pp.bitConsulta, pp.bitAgregar, pp.bitEditar, pp.bitEliminar 
           FROM permisos_perfil pp
           JOIN modulos m ON pp.idModulo = m.id
           WHERE pp.idPerfil = $1"#,
        claims.id_perfil
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(permisos)
}
use crate::middleware::auth::Claims;
use crate::models::menu::MenuDinamicoItem; 
use axum::{extract::State, response::IntoResponse, Json, http::StatusCode};
use sqlx::PgPool;


pub async fn obtener_menu_dinamico(
    State(pool): State<PgPool>,
    claims: Claims,
) -> impl IntoResponse {
    
    let res = sqlx::query_as!(
        MenuDinamicoItem,
        r#"
        SELECT DISTINCT
            m_parent.idMenu as "parent_id!",
            CASE 
                WHEN m_parent.idMenu = 1 THEN 'Seguridad'
                WHEN m_parent.idMenu = 2 THEN 'Principal 1'
                WHEN m_parent.idMenu = 3 THEN 'Principal 2'
            END as "parent_name!",
            mod.id as "modulo_id!",
            mod.strNombreModulo as "modulo_name!",
            pp.bitAgregar as "bitagregar!", 
            pp.bitEditar as "biteditar!", 
            pp.bitEliminar as "biteliminar!", 
            pp.bitConsulta as "bitconsulta!"
        FROM menu m_parent
        JOIN modulos mod ON m_parent.idModulo = mod.id
        JOIN permisos_perfil pp ON pp.idModulo = mod.id
        WHERE pp.idPerfil = $1 
          AND (pp.bitAgregar = TRUE OR pp.bitEditar = TRUE OR pp.bitEliminar = TRUE OR pp.bitConsulta = TRUE)
        ORDER BY m_parent.idMenu, mod.strNombreModulo
        "#,
        claims.id_perfil
    )
    .fetch_all(&pool)
    .await;

    match res {
        Ok(rows) => (StatusCode::OK, Json(rows)).into_response(),
        Err(e) => {
            println!("Error en menu dinamico: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error al cargar menu").into_response()
        }
    }
}
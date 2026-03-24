use axum::{
    extract::State,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use sqlx::PgPool;
use crate::views::error_view::vista_error_personalizada;
use crate::views::layout::layout_maestro;
use crate::views::usuario_view::{vista_registro_usuario, vista_tabla_usuarios, vista_editar_usuario};
use crate::views::perfil_view::{vista_tabla_perfiles, vista_editar_perfil, vista_nuevo_perfil};


pub struct UsuarioConPerfil {
    pub id: i32,
    pub strnombreusuario: String,
    pub strcorreo: String,
    pub strnumerocelular: Option<String>,
    pub strimagenpath: Option<String>,
    pub strnombreperfil: String,
}

pub async fn dashboard_inicio() -> Html<String> {
    let contenido = r##"
        <div class="breadcrumb">Inicio > Dashboard</div>
        <div style="background: white; padding: 30px; border-radius: 12px; box-shadow: 0 2px 10px rgba(0,0,0,0.05);">
            <h1 style="color: var(--primary-blue); margin-top: 0;">Panel de Control</h1>
            <p>Bienvenido al sistema de gestión de Craxker Design Hub.</p>
            <hr style="border: 0; border-top: 1px solid #eee; margin: 20px 0;">
            <p style="color: #666;">Seleccione un módulo en la barra superior para comenzar.</p>
        </div>
    "##.to_string();
    Html(layout_maestro("Dashboard", contenido))
}

pub async fn vista_nuevo_usuario(State(pool): State<PgPool>) -> Html<String> {
    let perfiles = sqlx::query_as!(
        crate::models::Perfil,
        "SELECT id, strnombreperfil, bitadministrador as \"bitadministrador!\" FROM perfiles ORDER BY strnombreperfil ASC"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let contenido = vista_registro_usuario(perfiles);
    Html(layout_maestro("Nuevo Usuario", contenido))
}

pub async fn vista_lista_perfiles(State(pool): State<PgPool>) -> Html<String> {
    let perfiles = sqlx::query_as!(
        crate::models::Perfil,
        "SELECT id, strnombreperfil, bitadministrador as \"bitadministrador!\" FROM perfiles ORDER BY id ASC"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let contenido = vista_tabla_perfiles(perfiles);
    Html(layout_maestro("Gestión de Perfiles", contenido))
}

pub async fn vista_lista_usuarios(State(pool): State<PgPool>) -> Html<String> {
    let usuarios = sqlx::query_as!(
        UsuarioConPerfil,
        r#"
        SELECT u.id, u.strnombreusuario, u.strcorreo, u.strnumerocelular, 
               u.strimagenpath, p.strnombreperfil
        FROM usuarios u
        JOIN perfiles p ON u.idperfil = p.id
        ORDER BY u.id DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let contenido = vista_tabla_usuarios(usuarios);
    Html(layout_maestro("Lista de Usuarios", contenido))
}

pub async fn vista_editar_usuario_handler(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Html<String> {
    let usuario = sqlx::query_as!(
        crate::models::Usuario,
        "SELECT id, strnombreusuario, idperfil, strpwd, 
                idestadousuario as \"idestadousuario!\", 
                strcorreo, strnumerocelular, 
                strimagenpath as \"strimagenpath!\", 
                fechacreacion as \"fechacreacion!\" 
         FROM usuarios WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    if let Some(u) = usuario {
        let perfiles = sqlx::query_as!(
            crate::models::Perfil,
            "SELECT id, strnombreperfil, bitadministrador as \"bitadministrador!\" FROM perfiles ORDER BY strnombreperfil ASC"
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let contenido = vista_editar_usuario(u, perfiles);
        Html(layout_maestro("Editar Usuario", contenido))
    } else {
        Html(layout_maestro("Error", "<h1>Usuario no encontrado</h1>".to_string()))
    }
}


pub async fn vista_editar_perfil_handler(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Html<String> {
    let perfil = sqlx::query_as!(
        crate::models::Perfil,
        "SELECT id, strnombreperfil, bitadministrador as \"bitadministrador!\" FROM perfiles WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    if let Some(p) = perfil {
        let contenido = vista_editar_perfil(p);
        Html(layout_maestro("Editar Perfil", contenido))
    } else {
        Html(layout_maestro("Error", "<h1>Perfil no encontrado</h1>".to_string()))
    }
}

pub async fn vista_nuevo_perfil_form() -> Html<String> {
    Html(layout_maestro("Nuevo Perfil", vista_nuevo_perfil()))
}

pub async fn vista_login_handler() -> Html<String> {
    let site_key = std::env::var("RECAPTCHA_SITE_KEY")
        .unwrap_or_else(|_| "LLAVE_NO_CONFIGURADA".to_string());
    
    Html(crate::views::auth_view::vista_login(&site_key))
}


pub async fn vista_lista_modulos(State(pool): State<PgPool>) -> Html<String> {
    let modulos = sqlx::query_as!(
        crate::models::Modulo,
        "SELECT id, strnombremodulo FROM modulos ORDER BY id ASC"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Html(layout_maestro("Gestión de Módulos", crate::views::modulo_view::vista_tabla_modulos(modulos)))
}

pub async fn vista_nuevo_modulo_form() -> Html<String> {
    Html(layout_maestro("Nuevo Módulo", crate::views::modulo_view::vista_nuevo_modulo()))
}

pub async fn vista_editar_modulo_handler(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Html<String> {
    let modulo = sqlx::query_as!(
        crate::models::Modulo,
        "SELECT id, strnombremodulo FROM modulos WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    if let Some(m) = modulo {
        Html(layout_maestro("Editar Módulo", crate::views::modulo_view::vista_editar_modulo(m)))
    } else {
        Html(layout_maestro("Error", "<h1>Módulo no encontrado</h1>".to_string()))
    }
}


pub async fn vista_permisos_perfil(State(pool): State<PgPool>) -> Html<String> {
    let perfiles = sqlx::query_as!(
        crate::models::Perfil,
        "SELECT id, strnombreperfil, bitadministrador as \"bitadministrador!\" FROM perfiles ORDER BY strnombreperfil ASC"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let modulos = sqlx::query_as!(
        crate::models::Modulo,
        "SELECT id, strnombremodulo FROM modulos ORDER BY strnombremodulo ASC"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let contenido = crate::views::permisos_view::vista_matriz_permisos(perfiles, modulos);
    Html(layout_maestro("Configuración de Permisos", contenido))
}


pub async fn pantalla_estatica(req: axum::extract::Request) -> Html<String> {
    let path = req.uri().path();
    let nombre_modulo = path.split('/').last().unwrap_or("Módulo").to_uppercase();
    
    let titulo_display = nombre_modulo.replace('_', " ");

    let contenido = format!(r##"
        <div class="breadcrumb">
            <a href="/dashboard"><i class="fas fa-home"></i> Inicio</a>
            <i class="fas fa-chevron-right separator" style="margin: 0 10px; font-size: 0.8rem; color: #cbd5e1;"></i>
            <span>Módulos Principales</span>
            <i class="fas fa-chevron-right separator" style="margin: 0 10px; font-size: 0.8rem; color: #cbd5e1;"></i>
            <span class="current" style="color: var(--primary-blue); font-weight: 700;">{titulo}</span>
        </div>

        <div class="card-table" style="background: white; border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.08); padding: 30px; margin-top: 10px;">
            <div class="table-header" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 25px; border-bottom: 1px solid #f1f5f9; padding-bottom: 20px;">
                <div>
                    <h2 style="color: var(--primary-blue); margin:0; font-size: 1.5rem; letter-spacing: -0.5px;">Gestión de {titulo}</h2>
                    <p style="color: #64748b; font-size: 0.85rem; margin-top: 5px;">
                        <i class="fas fa-info-circle"></i> Módulo de demostración con arquitectura de datos estáticos
                    </p>
                </div>
                <button class="btn-new-pro" onclick="alert('Funcionalidad no disponible en modo demo')" 
                        style="background: var(--accent-green); color: white; border: none; padding: 12px 24px; border-radius: 8px; font-weight: 600; font-size: 0.9rem; display: inline-flex; align-items: center; gap: 8px; cursor: pointer; transition: 0.3s; box-shadow: 0 4px 10px rgba(40, 167, 69, 0.2);">
                    <i class="fas fa-plus-circle"></i> Nuevo Registro
                </button>
            </div>

            <div class="table-responsive" style="overflow-x: auto;">
                <table class="craxker-table" style="width: 100%; border-collapse: collapse;">
                    <thead>
                        <tr style="background: #f8fafc; border-bottom: 2px solid #e2e8f0;">
                            <th style="text-align: left; padding: 15px; color: var(--primary-blue); font-size: 0.85rem; text-transform: uppercase; letter-spacing: 0.5px;">ID</th>
                            <th style="text-align: left; padding: 15px; color: var(--primary-blue); font-size: 0.85rem; text-transform: uppercase; letter-spacing: 0.5px;">Descripción Demo</th>
                            <th style="text-align: left; padding: 15px; color: var(--primary-blue); font-size: 0.85rem; text-transform: uppercase; letter-spacing: 0.5px;">Fecha Registro</th>
                            <th style="text-align: left; padding: 15px; color: var(--primary-blue); font-size: 0.85rem; text-transform: uppercase; letter-spacing: 0.5px;">Estado</th>
                            <th style="text-align: center; padding: 15px; color: var(--primary-blue); font-size: 0.85rem; text-transform: uppercase; letter-spacing: 0.5px;">Acciones</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr style="border-bottom: 1px solid #f1f5f9; transition: 0.2s;">
                            <td style="padding: 15px; color: #64748b; font-weight: 600;">#101</td>
                            <td style="padding: 15px; color: #475569;">Dato de prueba para visualización de {titulo}</td>
                            <td style="padding: 15px; color: #64748b;">21/03/2026</td>
                            <td style="padding: 15px;">
                                <span style="background: #eef2ff; color: #4338ca; padding: 4px 10px; border-radius: 6px; font-size: 0.75rem; font-weight: 700;">ACTIVO</span>
                            </td>
                            <td style="padding: 15px; text-align: center; white-space: nowrap;">
                                <button class="btn-action edit" onclick="alert('Modo Demo')" style="background: #eef2ff; color: var(--primary-blue); border: none; padding: 8px 12px; border-radius: 6px; cursor: pointer; margin-right: 5px;"><i class="fas fa-edit"></i></button>
                                <button class="btn-action delete" onclick="alert('Modo Demo')" style="background: #fff1f2; color: #e11d48; border: none; padding: 8px 12px; border-radius: 6px; cursor: pointer;"><i class="fas fa-trash"></i></button>
                            </td>
                        </tr>
                        <tr style="border-bottom: 1px solid #f1f5f9; transition: 0.2s;">
                            <td style="padding: 15px; color: #64748b; font-weight: 600;">#102</td>
                            <td style="padding: 15px; color: #475569;">Segundo registro de ejemplo para validación de diseño</td>
                            <td style="padding: 15px; color: #64748b;">20/03/2026</td>
                            <td style="padding: 15px;">
                                <span style="background: #e6fffa; color: #2c7a7b; padding: 4px 10px; border-radius: 6px; font-size: 0.75rem; font-weight: 700;">PROCESADO</span>
                            </td>
                            <td style="padding: 15px; text-align: center; white-space: nowrap;">
                                <button class="btn-action edit" onclick="alert('Modo Demo')" style="background: #eef2ff; color: var(--primary-blue); border: none; padding: 8px 12px; border-radius: 6px; cursor: pointer; margin-right: 5px;"><i class="fas fa-edit"></i></button>
                                <button class="btn-action delete" onclick="alert('Modo Demo')" style="background: #fff1f2; color: #e11d48; border: none; padding: 8px 12px; border-radius: 6px; cursor: pointer;"><i class="fas fa-trash"></i></button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            
            <div style="margin-top: 25px; padding: 15px; background: #fffbeb; border-left: 4px solid #f59e0b; border-radius: 4px;">
                <p style="margin: 0; color: #92400e; font-size: 0.85rem; font-weight: 500;">
                    <i class="fas fa-exclamation-triangle"></i> <strong>Nota de Evaluación:</strong> Este módulo es una vista estática diseñada para validar la estructura del menú y la respuesta del sistema de permisos. No requiere persistencia en base de datos.
                </p>
            </div>
        </div>

        <script>
            document.addEventListener('DOMContentLoaded', async () => {{
                // Protegemos el módulo con la lógica de seguridad global
                // Esto hará que si el usuario no tiene bitConsulta para este módulo, lo mande a error-403
                await aplicarPermisosAcciones('{mod_db}');
            }});
        </script>
    "##, titulo = titulo_display, mod_db = nombre_modulo);

    Html(layout_maestro(&titulo_display, contenido))
}

pub async fn handler_403() -> impl IntoResponse {
    let html = vista_error_personalizada(
        "403", 
        "Restringido", 
        "Tu nivel de acceso actual no permite el ingreso a este módulo de diseño. Solicita privilegios al administrador de Craxker Hub.", 
        "fa-fingerprint"
    );
    (axum::http::StatusCode::FORBIDDEN, Html(html))
}

pub async fn handler_404() -> impl IntoResponse {
    let html = vista_error_personalizada(
        "404", 
        "Perdido", 
        "El recurso que buscas ha sido movido del espacio de trabajo o nunca existió en nuestro HUB de diseño.", 
        "fa-ghost"
    );
    (axum::http::StatusCode::NOT_FOUND, Html(html))
}

pub async fn handler_500() -> impl IntoResponse {
    let html = vista_error_personalizada(
        "500", 
        "Falla Crítica del Sistema", 
        "Hubo un problema inesperado al procesar tu solicitud en la base de datos. Por favor, intenta más tarde o contacta al soporte técnico de Craxker Design Hub.", 
        "fa-exclamation-triangle"
    );
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Html(html))
}
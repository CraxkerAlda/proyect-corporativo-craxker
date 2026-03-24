use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
};
use dotenvy::dotenv;
use middleware::auth::{Accion, auth_middleware};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod middleware;
mod handlers;
mod models;
mod views;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL no definida");
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("127.0.0.1:{}", port).parse::<SocketAddr>().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");


let api_usuarios = Router::<sqlx::PgPool>::new()
        .route(
            "/",
            get(handlers::usuario::listar_usuarios).layer(from_fn_with_state(
                pool.clone(),
                |state, req, next| auth_middleware(state, "USUARIOS".into(), Accion::Consulta, req, next),
            )),
        )
        .route(
            "/registrar",
            post(handlers::usuario::registrar_usuario).layer(from_fn_with_state(
                pool.clone(),
                |state, req, next| auth_middleware(state, "USUARIOS".into(), Accion::Agregar, req, next),
            )),
        )
        .route(
            "/{id}",
            delete(handlers::usuario::eliminar_usuario).layer(from_fn_with_state(
                pool.clone(),
                |state, req, next| auth_middleware(state, "USUARIOS".into(), Accion::Eliminar, req, next),
            )),
        )
        .route(
            "/{id}",
            put(handlers::usuario::actualizar_usuario).layer(from_fn_with_state(
                pool.clone(),
                |state, req, next| auth_middleware(state, "USUARIOS".into(), Accion::Editar, req, next),
            )),
        );


    let api_perfiles = Router::<sqlx::PgPool>::new()
        .route("/", get(handlers::perfil::listar_perfiles).layer(from_fn_with_state(
            pool.clone(),
            |state, req, next| auth_middleware(state, "PERFIL".into(), Accion::Consulta, req, next)
        )))
        .route("/", post(handlers::perfil::crear_perfil).layer(from_fn_with_state(
            pool.clone(),
            |state, req, next| auth_middleware(state, "PERFIL".into(), Accion::Agregar, req, next)
        )))
        .route("/{id}", put(handlers::perfil::actualizar_perfil).layer(from_fn_with_state(
            pool.clone(),
            |state, req, next| auth_middleware(state, "PERFIL".into(), Accion::Editar, req, next)
        )))
        .route("/{id}", delete(handlers::perfil::eliminar_perfil).layer(from_fn_with_state(
            pool.clone(),
            |state, req, next| auth_middleware(state, "PERFIL".into(), Accion::Eliminar, req, next)
        )))
        .route("/activos", get(handlers::perfil::listar_perfiles));

    let api_modulos = Router::<sqlx::PgPool>::new()
        .route("/", get(handlers::modulo::listar_modulos).layer(from_fn_with_state(
            pool.clone(),
            |state, req, next| auth_middleware(state, "MODULOS".into(), Accion::Consulta, req, next)
        )))
        .route("/menu", get(handlers::modulo::listar_modulos_json))
        .route("/", post(handlers::modulo::crear_modulo).layer(from_fn_with_state(
            pool.clone(),
            |state, req, next| auth_middleware(state, "MODULOS".into(), Accion::Agregar, req, next)
        )))
        .route("/{id}", delete(handlers::modulo::eliminar_modulo).layer(from_fn_with_state(
            pool.clone(),
            |state, req, next| auth_middleware(state, "MODULOS".into(), Accion::Eliminar, req, next)
        )))
        .route("/{id}", put(handlers::modulo::actualizar_modulo).layer(from_fn_with_state(
            pool.clone(),
            |state, req, next| auth_middleware(state, "MODULOS".into(), Accion::Editar, req, next)
        )));

    let api_permisos = Router::<sqlx::PgPool>::new()
        .route("/{id_perfil}", get(handlers::permisos::obtener_permisos_por_perfil).layer(from_fn_with_state(
            pool.clone(),
            |state, req, next| auth_middleware(state, "PERMISOS_PERFIL".into(), Accion::Consulta, req, next)
        )))
        .route("/", post(handlers::permisos::guardar_permiso).layer(from_fn_with_state(
            pool.clone(),
            |state, req, next| auth_middleware(state, "PERMISOS_PERFIL".into(), Accion::Editar, req, next)
        )))
        .route("/mis-permisos", get(handlers::permisos::obtener_mis_permisos));

    let app = Router::new()
        .route("/", get(|| async { axum::response::Redirect::to("/login") }))        
        .nest_service("/uploads", ServeDir::new("static/uploads"))
        
        .route("/login", get(handlers::vistas::vista_login_handler))
        .route("/api/auth/login", post(handlers::auth::login_handler))

        .route("/dashboard", get(handlers::vistas::dashboard_inicio))
        
        .route("/vistas/usuarios", get(handlers::vistas::vista_lista_usuarios))
        .route("/vistas/usuario", get(handlers::vistas::vista_nuevo_usuario))
        .route("/vistas/usuario/editar/{id}", get(handlers::vistas::vista_editar_usuario_handler))

        .route("/vistas/perfiles", get(handlers::vistas::vista_lista_perfiles))
        .route("/vistas/perfiles/nuevo", get(handlers::vistas::vista_nuevo_perfil_form))
        .route("/vistas/perfiles/editar/{id}", get(handlers::vistas::vista_editar_perfil_handler))

        .route("/vistas/modulo", get(handlers::vistas::vista_lista_modulos))
        .route("/vistas/modulo/nuevo", get(handlers::vistas::vista_nuevo_modulo_form))
        .route("/vistas/modulo/editar/{id}", get(handlers::vistas::vista_editar_modulo_handler))

        .route("/vistas/permisos", get(handlers::vistas::vista_permisos_perfil))
        
        .route("/dinamico", get(handlers::menu::obtener_menu_dinamico))

        .route("/vistas/principal_1_1", get(handlers::vistas::pantalla_estatica))
        .route("/vistas/principal_1_2", get(handlers::vistas::pantalla_estatica))
        .route("/vistas/principal_2_1", get(handlers::vistas::pantalla_estatica))
        .route("/vistas/principal_2_2", get(handlers::vistas::pantalla_estatica))

        .nest("/api/usuarios", api_usuarios)
        .nest("/api/modulos", api_modulos)
        .nest("/api/perfiles", api_perfiles)
        .nest("/api/permisos", api_permisos)
        
        .route("/error-403", get(handlers::vistas::handler_403))

        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(5 * 1024 * 1024))
        .fallback(handlers::vistas::handler_404) 
        .with_state(pool);

    println!("Servidor en http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
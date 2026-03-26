use axum::{
    extract::{Multipart, State, Path},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::path::Path as StdPath;

pub async fn registrar_usuario(
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut nombre_usuario = String::new();
    let mut id_perfil = 0;
    let mut password = String::new();
    let mut correo = String::new();
    let mut celular = String::new();
    let mut imagen_path = String::from("/uploads/default.png");

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "usuario" => nombre_usuario = field.text().await.unwrap_or_default(),
            "perfil" => id_perfil = field.text().await.unwrap_or_default().parse::<i32>().unwrap_or(0),
            "password" => password = field.text().await.unwrap_or_default(),
            "correo" => correo = field.text().await.unwrap_or_default(),
            "celular" => celular = field.text().await.unwrap_or_default(),
            "foto" => {
                let file_name = field.file_name().unwrap_or("foto.jpg").to_string();
                let extension = StdPath::new(&file_name)
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("jpg");
                
                let nuevo_nombre = format!("{}.{}", Uuid::new_v4(), extension);
                let path = format!("static/uploads/{}", nuevo_nombre);
                let data = field.bytes().await.unwrap_or_default();

                if !data.is_empty() {
                    if let Ok(mut file) = File::create(&path).await {
                        if file.write_all(&data).await.is_ok() {
                            imagen_path = format!("/uploads/{}", nuevo_nombre);
                        }
                    }
                }
            },
            _ => {}
        }
    }

    let password_hash = match hash(password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error al cifrar contraseña").into_response(),
    };

    let res = sqlx::query!(
        r#"
        INSERT INTO usuarios (strNombreUsuario, idPerfil, strPwd, strCorreo, strNumeroCelular, strImagenPath, idEstadoUsuario)
        VALUES ($1, $2, $3, $4, $5, $6, 1)
        "#,
        nombre_usuario.trim(),
        id_perfil,
        password_hash,
        correo.trim(),
        celular.trim(),
        imagen_path
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, "Usuario registrado con éxito").into_response(),
        Err(e) => {
            println!("Error DB: {:?}", e);
            (StatusCode::BAD_REQUEST, "El usuario o correo ya existen").into_response()
        }
    }
}

pub async fn actualizar_usuario(
    State(pool): State<PgPool>,
    Path(id): Path<i32>, 
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut nombre_usuario = String::new();
    let mut id_perfil = 0;
    let mut correo = String::new();
    let mut celular = String::new();
    let mut password_plano = String::new();
    let mut id_estado: i16 = 1;
    let mut nueva_imagen: Option<String> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "usuario" => nombre_usuario = field.text().await.unwrap_or_default(),
            "perfil" => id_perfil = field.text().await.unwrap_or_default().parse::<i32>().unwrap_or(0),
            "correo" => correo = field.text().await.unwrap_or_default(),
            "celular" => celular = field.text().await.unwrap_or_default(),
            "password" => password_plano = field.text().await.unwrap_or_default(),
            "estado" => id_estado = field.text().await.unwrap_or_default().parse::<i16>().unwrap_or(1),
            "foto" => {
                let data = field.bytes().await.unwrap_or_default();
                if !data.is_empty() {
                    let nuevo_nombre = format!("{}.jpg", Uuid::new_v4());
                    let path = format!("static/uploads/{}", nuevo_nombre);
                    if let Ok(mut file) = File::create(&path).await {
                        if file.write_all(&data).await.is_ok() {
                            nueva_imagen = Some(format!("/uploads/{}", nuevo_nombre));
                        }
                    }
                }
            },
            _ => {}
        }
    }

    // Lógica para actualizar contraseña solo si se envió una nueva
    let nuevo_hash = if !password_plano.trim().is_empty() {
        match hash(password_plano, DEFAULT_COST) {
            Ok(h) => Some(h),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error al cifrar contraseña").into_response(),
        }
    } else {
        None
    };

    let res = sqlx::query!(
        r#"
        UPDATE usuarios 
        SET strNombreUsuario = $1, 
            idPerfil = $2, 
            strCorreo = $3, 
            strNumeroCelular = $4, 
            idEstadoUsuario = $5,
            strImagenPath = COALESCE($6, strImagenPath),
            strPwd = COALESCE($7, strPwd)
        WHERE id = $8
        "#,
        nombre_usuario.trim(),
        id_perfil,
        correo.trim(),
        celular.trim(),
        id_estado,
        nueva_imagen,
        nuevo_hash,
        id
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::OK, "Usuario actualizado con éxito").into_response(),
        Err(e) => {
            println!("Error al actualizar: {:?}", e);
            (StatusCode::BAD_REQUEST, "Error en la base de datos").into_response()
        }
    }
}

pub async fn eliminar_usuario() -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Próximamente")
}

pub async fn listar_usuarios() -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Próximamente")
}
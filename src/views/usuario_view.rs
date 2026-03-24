use crate::handlers::vistas::UsuarioConPerfil;

pub fn vista_registro_usuario(perfiles: Vec<crate::models::Perfil>) -> String {
    let mut opciones_perfil = String::new();
    for p in perfiles {
        opciones_perfil.push_str(&format!(
            r#"<option value="{}">{}</option>"#,
            p.id, p.strnombreperfil
        ));
    }

    format!(r##"

    <div class="breadcrumb">
        <a href="/dashboard"><i class="fas fa-home"></i> Inicio</a>
        <i class="fas fa-chevron-right separator"></i>
        <span>Seguridad</span>
        <i class="fas fa-chevron-right separator"></i>
        <a href="/vistas/usuarios">Usuario</a>
        <i class="fas fa-chevron-right separator"></i>
        <span class="current">Nuevo usuario</span>
    </div>
    
    <div class="card-usuario">
        <div class="form-header">
            <div class="avatar-wrapper">
                <img id="imgPreview" src="/uploads/default.png" 
                     onerror="this.src='https://ui-avatars.com/api/?name=User&background=1a3a5a&color=fff'" 
                     alt="Previsualización" class="preview-circular">
                <label for="inputFoto" class="btn-upload-icon"><i class="fas fa-camera"></i></label>
            </div>
            <h2 style="color: var(--primary-blue); margin: 10px 0 5px 0;">Registro Craxker Design Hub</h2>
            <p style="color: #666; font-size: 0.9rem;">Gestión Profesional de Personal</p>
        </div>

        <form id="formRegistro" onsubmit="validarYEnviar(event)">
            <div class="grid-form">
                <div class="form-group">
                    <label><i class="fas fa-user"></i> Usuario (Max 15 caracteres):</label>
                    <input type="text" id="txtUsuario" name="usuario" required maxlength="15" placeholder="Ej. craxker_dev">
                    <small id="errUsuario" class="text-error"></small>
                </div>
                <div class="form-group">
                    <label><i class="fas fa-id-badge"></i> Perfil de Acceso:</label>
                    <select name="perfil" required>
                        <option value="">Seleccione un rol...</option>
                        {perfiles_options}
                    </select>
                </div>
                <div class="form-group">
                    <label><i class="fas fa-lock"></i> Contraseña (Min 8 caracteres):</label>
                    <input type="password" id="txtPass" name="password" required minlength="8" placeholder="••••••••">
                </div>
                <div class="form-group">
                    <label><i class="fas fa-envelope"></i> Correo Electrónico:</label>
                    <input type="email" name="correo" required placeholder="admin@craxker.com">
                </div>
                <div class="form-group">
                    <label><i class="fas fa-phone"></i> Número Celular (10 dígitos):</label>
                    <input type="text" id="txtTel" name="celular" required pattern="[0-9]{{10}}" placeholder="Ej. 7731234567">
                </div>
                <div class="form-group">
                    <label><i class="fas fa-image"></i> Foto de Perfil:</label>
                    <input type="file" name="foto" id="inputFoto" accept="image/*" onchange="previewImage(event)" style="font-size: 0.8rem;">
                </div>
            </div>

            <div class="form-actions">
                <button type="submit" id="btnGuardar" class="btn-save"><i class="fas fa-save"></i> Registrar Usuario</button>
                <button type="button" class="btn-cancel" onclick="window.location.href='/vistas/usuarios'">Cancelar</button>
            </div>
        </form>
    </div>

    <style>
        .text-error {{ color: #dc3545; font-size: 0.75rem; font-weight: bold; margin-top: 4px; }}
        .card-usuario {{ background: white; padding: 40px; border-radius: 15px; box-shadow: 0 10px 25px rgba(0,0,0,0.05); max-width: 850px; margin: auto; }}
        .form-header {{ text-align: center; margin-bottom: 30px; }}
        .avatar-wrapper {{ position: relative; width: 120px; height: 120px; margin: 0 auto; }}
        .preview-circular {{ width: 120px; height: 120px; border-radius: 50%; object-fit: cover; border: 4px solid var(--accent-green); background: #eee; }}
        .btn-upload-icon {{ position: absolute; bottom: 5px; right: 5px; background: var(--primary-blue); color: white; width: 32px; height: 32px; border-radius: 50%; display: flex; align-items: center; justify-content: center; cursor: pointer; border: 2px solid white; }}
        .grid-form {{ display: grid; grid-template-columns: 1fr 1fr; gap: 25px; }}
        @media (max-width: 650px) {{ .grid-form {{ grid-template-columns: 1fr; }} }}
        .form-group {{ display: flex; flex-direction: column; gap: 8px; }}
        .form-group label {{ font-weight: 600; color: var(--primary-blue); font-size: 0.85rem; }}
        .form-group input, .form-group select {{ padding: 12px; border: 1px solid #e2e8f0; border-radius: 8px; font-size: 0.95rem; }}
        .form-actions {{ margin-top: 40px; display: flex; gap: 15px; justify-content: center; border-top: 1px solid #f1f5f9; padding-top: 30px; }}
        .btn-save {{ background: var(--accent-green); color: white; border: none; padding: 12px 40px; border-radius: 8px; cursor: pointer; font-weight: bold; }}
        .btn-cancel {{ background: #f1f5f9; color: #64748b; border: none; padding: 12px 40px; border-radius: 8px; cursor: pointer; font-weight: bold; }}
    </style>

    <script>
        function previewImage(event) {{
            const reader = new FileReader();
            reader.onload = () => document.getElementById('imgPreview').src = reader.result;
            reader.readAsDataURL(event.target.files[0]);
        }}
        function validarYEnviar(e) {{
            e.preventDefault();
            const usuario = document.getElementById('txtUsuario').value;
            const errorU = document.getElementById('errUsuario');
            errorU.innerText = "";
            if (/(.)\1{{4,}}/.test(usuario)) {{ errorU.innerText = "Demasiados caracteres repetidos."; return; }}
            if (/^\d+$/.test(usuario)) {{ errorU.innerText = "No puede ser solo números."; return; }}
            enviarFormulario();
        }}
        async function enviarFormulario() {{
            const btn = document.getElementById('btnGuardar');
            const form = document.getElementById('formRegistro');
            const formData = new FormData(form);
            
            btn.disabled = true;
            btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Registrando...';

            try {{
                const res = await fetch('/api/usuarios/registrar', {{ 
                    method: 'POST', 
                    headers: {{
                        'Authorization': 'Bearer ' + localStorage.getItem('jwt_token')
                    }},
                    body: formData 
                }});

                if (res.ok) {{ 
                    alert("Usuario registrado con éxito en Craxker Design Hub"); 
                    window.location.href = "/vistas/usuarios"; 
                }}
                else if (res.status === 403) {{
                    alert("Error 403: No tienes permiso para crear usuarios.");
                    btn.disabled = false;
                    btn.innerHTML = '<i class="fas fa-save"></i> Registrar Usuario';
                }}
                else {{ 
                    const txt = await res.text();
                    alert("Error en el registro: " + txt); 
                    btn.disabled = false; 
                    btn.innerHTML = '<i class="fas fa-save"></i> Registrar Usuario';
                }}
            }} catch (err) {{ 
                console.error(err);
                alert("Error de conexión con el servidor."); 
                btn.disabled = false; 
                btn.innerHTML = '<i class="fas fa-save"></i> Registrar Usuario';
            }}
        }}
        
        document.addEventListener('DOMContentLoaded', async () => {{
            await aplicarPermisosAcciones('USUARIOS', 'bitagregar');
        }});
    </script>
    "##, perfiles_options = opciones_perfil)
}


pub fn vista_tabla_usuarios(usuarios: Vec<UsuarioConPerfil>) -> String {
    let mut filas = String::new();
    
    let mut perfiles_vistos: Vec<String> = Vec::new();
    for u in &usuarios {
        if !perfiles_vistos.contains(&u.strnombreperfil) {
            perfiles_vistos.push(u.strnombreperfil.clone());
        }
    }
    let mut opciones_perfil_filtro = String::from(r#"<option value="">Todos</option>"#);
    for p in &perfiles_vistos {
        opciones_perfil_filtro.push_str(&format!(r#"<option value="{p}">{p}</option>"#));
    }

    for u in usuarios {
        filas.push_str(&format!(
            r#"
            <tr class="user-row"
                data-nombre="{user_lower}"
                data-perfil="{perfil}"
                data-estado="{estado_str}">
                <td class="user-td">
                    <img src="{img}" onerror="this.src='/uploads/default.png'" class="table-avatar">
                    <span>{user}</span>
                </td>
                <td><span class="badge-perfil">{perfil}</span></td>
                <td>{correo}</td>
                <td>{tel}</td>
                <td class="actions">
                    <a href="/vistas/usuario/editar/{id}" class="btn-action edit" title="Editar Usuario">
                        <i class="fas fa-user-edit"></i>
                    </a>
                    <button class="btn-action delete" onclick="eliminarUsuario({id})" title="Eliminar Usuario">
                        <i class="fas fa-trash-alt"></i>
                    </button>
                </td>
            </tr>
            "#,
            img = u.strimagenpath.unwrap_or_else(|| "/uploads/default.png".to_string()),
            user = u.strnombreusuario,
            user_lower = u.strnombreusuario.to_lowercase(),
            perfil = u.strnombreperfil,
            correo = u.strcorreo,
            tel = u.strnumerocelular.unwrap_or_else(|| "-".to_string()),
            estado_str = "activo", // idEstado no está en UsuarioConPerfil, se deja fijo o amplías el struct
            id = u.id
        ));
    }

    format!(r##"
    <div class="breadcrumb">
        <a href="/dashboard"><i class="fas fa-home"></i> Inicio</a>
        <i class="fas fa-chevron-right separator"></i>
        <span>Seguridad</span>
        <i class="fas fa-chevron-right separator"></i>
        <a href="/vistas/usuarios">Usuarios</a>
    </div>

    <div class="card-table">
        <div class="table-header">
            <div>
                <h2 style="color: var(--primary-blue); margin:0;">Personal Registrado</h2>
                <p style="color: #64748b; font-size: 0.85rem; margin-top: 5px;">Listado general de usuarios en Craxker Design Hub</p>
            </div>
            <a href="/vistas/usuario" class="btn-new-pro">
                <i class="fas fa-plus-circle"></i> Nuevo Usuario
            </a>
        </div>

        <!-- FILTROS -->
        <div class="filtros-container">
            <div class="filtro-grupo">
                <label class="filtro-label">BUSCAR POR NOMBRE</label>
                <div class="filtro-search-wrap">
                    <i class="fas fa-search filtro-icon"></i>
                    <input type="text" id="filtroBusqueda" class="filtro-input" placeholder="Buscar usuario..." oninput="aplicarFiltros()">
                </div>
            </div>
            <div class="filtro-grupo filtro-grupo-sm">
                <label class="filtro-label">PERFIL ASIGNADO</label>
                <select id="filtroPerfil" class="filtro-select" onchange="aplicarFiltros()">
                    {opciones_perfil_filtro}
                </select>
            </div>
            <div class="filtro-grupo filtro-grupo-sm">
                <label class="filtro-label">ESTADO</label>
                <select id="filtroEstado" class="filtro-select" onchange="aplicarFiltros()">
                    <option value="">Todos</option>
                    <option value="activo">Activo</option>
                    <option value="inactivo">Inactivo</option>
                </select>
            </div>
            <div class="filtro-grupo filtro-grupo-xs">
                <label class="filtro-label">&nbsp;</label>
                <button class="filtro-btn-limpiar" onclick="limpiarFiltros()">
                    <i class="fas fa-times"></i> Limpiar
                </button>
            </div>
        </div>

        <div class="table-responsive">
            <table class="craxker-table" id="tablaUsuarios">
                <thead>
                    <tr>
                        <th>Usuario</th>
                        <th>Perfil</th>
                        <th>Correo Electrónico</th>
                        <th>Teléfono</th>
                        <th style="text-align: center;">Acciones</th>
                    </tr>
                </thead>
                <tbody>
                    {filas}
                </tbody>
            </table>
            <div id="sinResultados" style="display:none; text-align:center; padding:40px; color:#94a3b8;">
                <i class="fas fa-search" style="font-size:2rem; margin-bottom:10px;"></i>
                <p style="margin:0; font-weight:600;">No se encontraron usuarios con esos filtros</p>
            </div>
        </div>

        <div class="pagination-container" style="display:flex; justify-content:space-between; align-items:center; margin-top:20px;">
            <span id="contadorResultados" style="font-size:0.85rem; color:#64748b;"></span>
            <div class="pagination-btns">
                <button class="btn-page" id="btnPrev" onclick="cambiarPagina(-1)"><i class="fas fa-chevron-left"></i> Anterior</button>
                <button class="btn-page" id="btnNext" onclick="cambiarPagina(1)">Siguiente <i class="fas fa-chevron-right"></i></button>
            </div>
        </div>
    </div>

    <style>
        .card-table {{ background: white; border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.08); padding: 25px; margin-top: 10px; }}
        .table-header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }}
        .btn-new-pro {{ background: var(--accent-green); color: white; text-decoration: none; padding: 12px 24px; border-radius: 8px; font-weight: 600; font-size: 0.9rem; display: inline-flex; align-items: center; gap: 8px; transition: 0.3s; box-shadow: 0 4px 10px rgba(40, 167, 69, 0.2); }}
        .btn-new-pro:hover {{ background: #218838; transform: translateY(-2px); }}

        .filtros-container {{ display: flex; gap: 16px; align-items: flex-end; flex-wrap: wrap; background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 10px; padding: 18px 20px; margin-bottom: 20px; }}
        .filtro-grupo {{ display: flex; flex-direction: column; gap: 6px; flex: 1; min-width: 200px; }}
        .filtro-grupo-sm {{ flex: 0 0 160px; min-width: 140px; }}
        .filtro-grupo-xs {{ flex: 0 0 110px; min-width: 100px; }}
        .filtro-label {{ font-size: 0.85rem; font-weight: 700; color: var(--primary-blue); letter-spacing: 0.5px; text-transform: uppercase; }}
        .filtro-search-wrap {{ position: relative; }}
        .filtro-icon {{ position: absolute; left: 12px; top: 50%; transform: translateY(-50%); color: #94a3b8; font-size: 0.85rem; }}
        .filtro-input {{ width: 100%; padding: 10px 12px 10px 34px; border: 1px solid #e2e8f0; border-radius: 8px; font-size: 0.9rem; box-sizing: border-box; font-family: inherit; }}
        .filtro-input:focus {{ outline: none; border-color: var(--primary-blue); }}
        .filtro-select {{ width: 100%; padding: 10px 12px; border: 1px solid #e2e8f0; border-radius: 8px; font-size: 0.9rem; background: white; font-family: inherit; }}
        .filtro-select:focus {{ outline: none; border-color: var(--primary-blue); }}
        .filtro-btn-limpiar {{ width: 100%; padding: 10px 12px; background: white; border: 1px solid #e2e8f0; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.85rem; color: #64748b; display: flex; align-items: center; justify-content: center; gap: 6px; transition: 0.2s; }}
        .filtro-btn-limpiar:hover {{ background: #f1f5f9; color: #e11d48; border-color: #fecaca; }}

        .craxker-table {{ width: 100%; border-collapse: collapse; }}
        .craxker-table th {{ text-align: left; padding: 15px; background: #f8fafc; color: var(--primary-blue); font-size: 0.85rem; text-transform: uppercase; letter-spacing: 0.5px; border-bottom: 2px solid #e2e8f0; }}
        .craxker-table td {{ padding: 12px 15px; border-bottom: 1px solid #f1f5f9; vertical-align: middle; color: #475569; }}
        .user-td {{ display: flex; align-items: center; gap: 12px; font-weight: 600; color: var(--primary-blue); }}
        .table-avatar {{ width: 38px; height: 38px; border-radius: 50%; object-fit: cover; border: 2px solid #e2e8f0; }}
        .badge-perfil {{ background: #eef2ff; color: #4338ca; padding: 4px 10px; border-radius: 6px; font-size: 0.75rem; font-weight: 700; }}
        .actions {{ text-align: center; white-space: nowrap; }}
        .btn-action {{ padding: 8px 10px; border-radius: 6px; text-decoration: none; display: inline-block; transition: 0.2s; border: none; cursor: pointer; }}
        .edit {{ background: #eef2ff; color: var(--primary-blue); }}
        .delete {{ background: #fff1f2; color: #e11d48; margin-left: 5px; }}
        .btn-action:hover {{ transform: scale(1.1); }}
        .btn-page {{ padding: 8px 16px; border: 1px solid #ddd; background: white; border-radius: 6px; cursor: pointer; }}
        .btn-page:disabled {{ opacity: 0.5; cursor: not-allowed; }}
    </style>

    <script>
        let paginaActual = 1;
        const filasPorPagina = 5;
        let filasFiltradas = [];

        function aplicarFiltros() {{
            const busqueda  = document.getElementById('filtroBusqueda').value.toLowerCase().trim();
            const perfil    = document.getElementById('filtroPerfil').value;
            const estado    = document.getElementById('filtroEstado').value;
            const todasFilas = Array.from(document.querySelectorAll('.user-row'));

            filasFiltradas = todasFilas.filter(fila => {{
                const nombre  = fila.dataset.nombre  || '';
                const prf     = fila.dataset.perfil  || '';
                const est     = fila.dataset.estado  || '';
                const okNombre = !busqueda || nombre.includes(busqueda);
                const okPerfil = !perfil   || prf === perfil;
                const okEstado = !estado   || est === estado;
                return okNombre && okPerfil && okEstado;
            }});

            // Ocultar todas primero
            todasFilas.forEach(f => f.style.display = 'none');

            paginaActual = 1;
            mostrarPagina(1);

            document.getElementById('sinResultados').style.display =
                filasFiltradas.length === 0 ? 'block' : 'none';
        }}

        function limpiarFiltros() {{
            document.getElementById('filtroBusqueda').value = '';
            document.getElementById('filtroPerfil').value  = '';
            document.getElementById('filtroEstado').value  = '';
            aplicarFiltros();
        }}

        function mostrarPagina(n) {{
            const base = filasFiltradas.length > 0
                ? filasFiltradas
                : Array.from(document.querySelectorAll('.user-row'));

            const totalPaginas = Math.ceil(base.length / filasPorPagina);
            if (n < 1) n = 1;
            if (n > totalPaginas) n = totalPaginas;
            paginaActual = n;

            // Ocultar todo y mostrar solo la página actual
            Array.from(document.querySelectorAll('.user-row')).forEach(f => f.style.display = 'none');
            base.forEach((fila, index) => {{
                fila.style.display = (index >= (n-1)*filasPorPagina && index < n*filasPorPagina)
                    ? 'table-row' : 'none';
            }});

            document.getElementById('btnPrev').disabled = (paginaActual === 1);
            document.getElementById('btnNext').disabled = (paginaActual === totalPaginas || totalPaginas === 0);
        }}

        function cambiarPagina(delta) {{
            mostrarPagina(paginaActual + delta);
        }}

        document.addEventListener('DOMContentLoaded', async () => {{
            await aplicarPermisosAcciones('USUARIOS');
            filasFiltradas = Array.from(document.querySelectorAll('.user-row'));
            mostrarPagina(1);
        }});

        async function eliminarUsuario(id) {{
            if(confirm("¿Desea eliminar este usuario?")) {{
                try {{
                    const res = await fetch(`/api/usuarios/${{id}}`, {{
                        method: 'DELETE',
                        headers: {{ 'Authorization': 'Bearer ' + localStorage.getItem('jwt_token') }}
                    }});
                    if(res.ok) location.reload();
                    else alert("No tiene permiso para eliminar registros.");
                }} catch(e) {{
                    alert("Error de conexión con el servidor.");
                }}
            }}
        }}
    </script>
    "##, filas = filas, opciones_perfil_filtro = opciones_perfil_filtro)
}

pub fn vista_editar_usuario(u: crate::models::Usuario, perfiles: Vec<crate::models::Perfil>) -> String {
    let mut opciones = String::new();
    for p in perfiles {
        let sel = if p.id == u.idperfil { "selected" } else { "" };
        opciones.push_str(&format!(r#"<option value="{}" {}>{}</option>"#, p.id, sel, p.strnombreperfil));
    }

    let sel_activo = if u.idestadousuario == 1 { "selected" } else { "" };
    let sel_inactivo = if u.idestadousuario != 1 { "selected" } else { "" };

    format!(r##"
    <div class="breadcrumb">
        <a href="/dashboard"><i class="fas fa-home"></i> Inicio</a>
        <i class="fas fa-chevron-right separator"></i>
        <span>Seguridad</span>
        <i class="fas fa-chevron-right separator"></i>
        <a href="/vistas/usuarios">Usuario</a>
        <i class="fas fa-chevron-right separator"></i>
        <span class="current">Editar Usuario</span>
    </div>

    <div class="card-usuario">
        <div class="form-header">
            <div class="avatar-wrapper">
                <img id="imgPreview" src="{foto_actual}" 
                     onerror="this.src='/uploads/default.png'" 
                     class="preview-circular">
                <label for="inputFoto" class="btn-upload-icon"><i class="fas fa-camera"></i></label>
            </div>
            <h2 style="color: var(--primary-blue); margin-top: 15px;">Modificar Datos de Usuario</h2>
            <p style="color: #64748b; font-size: 0.9rem;">Gestión Profesional de Personal</p>
        </div>

        <form id="formEdicion" onsubmit="actualizarUsuario(event, {id_user})">
            <div class="grid-form">
                <div class="form-group">
                    <label><i class="fas fa-user"></i> Nombre de Usuario:</label>
                    <input type="text" id="txtUsuario" name="usuario" value="{user_name}" required maxlength="15">
                </div>
                <div class="form-group">
                    <label><i class="fas fa-id-badge"></i> Perfil de Acceso:</label>
                    <select name="perfil" required>
                        {opciones}
                    </select>
                </div>
                <div class="form-group">
                    <label><i class="fas fa-envelope"></i> Correo Electrónico:</label>
                    <input type="email" name="correo" value="{correo}" required>
                </div>
                <div class="form-group">
                    <label><i class="fas fa-phone"></i> Número de Contacto:</label>
                    <input type="text" id="txtTel" name="celular" value="{tel}" required pattern="[0-9]{{10}}">
                </div>
                <div class="form-group">
                    <label><i class="fas fa-toggle-on"></i> Estado del Usuario:</label>
                    <select name="estado" required>
                        <option value="1" {sel_activo}>Activo</option>
                        <option value="2" {sel_inactivo}>Inactivo</option>
                    </select>
                </div>
                <div class="form-group">
                    <label><i class="fas fa-image"></i> Cambiar Foto de Perfil:</label>
                    <input type="file" name="foto" id="inputFoto" accept="image/*" onchange="previewImage(event)">
                </div>
            </div>

            <div class="form-actions">
                <button type="submit" id="btnGuardar" class="btn-save"><i class="fas fa-sync-alt"></i> Actualizar Usuario</button>
                <button type="button" class="btn-cancel" onclick="window.location.href='/vistas/usuarios'">Cancelar</button>
            </div>
        </form>
    </div>


        <style>

        .card-usuario {{ background: white; padding: 40px; border-radius: 15px; box-shadow: 0 10px 25px rgba(0,0,0,0.08); max-width: 800px; margin: auto; }}
        .form-header {{ text-align: center; margin-bottom: 30px; }}
        .avatar-wrapper {{ position: relative; width: 130px; height: 130px; margin: 0 auto; }}
        .preview-circular {{ width: 130px; height: 130px; border-radius: 50%; object-fit: cover; border: 4px solid var(--accent-green); box-shadow: 0 4px 12px rgba(0,0,0,0.1); }}
        .btn-upload-icon {{ position: absolute; bottom: 5px; right: 5px; background: var(--primary-blue); color: white; width: 35px; height: 35px; border-radius: 50%; display: flex; align-items: center; justify-content: center; cursor: pointer; border: 3px solid white; transition: 0.3s; }}
        .btn-upload-icon:hover {{ transform: scale(1.1); background: var(--accent-green); }}

        .grid-form {{ display: grid; grid-template-columns: 1fr 1fr; gap: 25px; }}

        .form-group {{ display: flex; flex-direction: column; gap: 8px; }}

        .form-group label {{ font-weight: 600; color: var(--primary-blue); font-size: 0.85rem; display: flex; align-items: center; gap: 6px; }}

        .form-group input, .form-group select {{ padding: 12px; border: 1px solid #e2e8f0; border-radius: 8px; font-size: 0.95rem; transition: 0.3s; }}

        .form-actions {{ margin-top: 40px; display: flex; gap: 15px; justify-content: center; border-top: 1px solid #f1f5f9; padding-top: 30px; }}

        .btn-save {{ background: var(--accent-green); color: white; border: none; padding: 12px 35px; border-radius: 8px; cursor: pointer; font-weight: bold; font-size: 0.95rem; transition: 0.3s; display: flex; align-items: center; gap: 8px; }}

        .btn-cancel {{ background: #f1f5f9; color: #64748b; border: none; padding: 12px 35px; border-radius: 8px; cursor: pointer; font-weight: bold; font-size: 0.95rem; transition: 0.3s; }}

        .text-error {{ color: #e11d48; font-size: 0.75rem; font-weight: 600; margin-top: 4px; }}

    </style>

    <script>
        function previewImage(event) {{
            const reader = new FileReader();
            reader.onload = () => document.getElementById('imgPreview').src = reader.result;
            reader.readAsDataURL(event.target.files[0]);
        }}

        async function actualizarUsuario(e, id) {{
            e.preventDefault();
            const btn = document.getElementById('btnGuardar');
            const formData = new FormData(document.getElementById('formEdicion'));
            btn.disabled = true;
            btn.innerHTML = 'Procesando...';

            try {{
                const res = await fetch(`/api/usuarios/${{id}}`, {{
                    method: 'PUT',
                    headers: {{ 'Authorization': 'Bearer ' + localStorage.getItem('jwt_token') }},
                    body: formData
                }});

                if (res.ok) {{
                    alert("Usuario actualizado correctamente");
                    window.location.href = "/vistas/usuarios";
                }} else {{
                    const msg = await res.text();
                    alert("Error: " + msg);
                    btn.disabled = false;
                }}
            }} catch (err) {{
                alert("Error de conexión con el servidor.");
                btn.disabled = false;
            }}
        }}

        document.addEventListener('DOMContentLoaded', async () => {{
            await aplicarPermisosAcciones('USUARIOS', 'biteditar');
        }});
    </script>
    "##, 
    id_user = u.id,
    user_name = u.strnombreusuario,
    foto_actual = u.strimagenpath,
    opciones = opciones,
    sel_activo = sel_activo,
    sel_inactivo = sel_inactivo,
    correo = u.strcorreo,
    tel = u.strnumerocelular.unwrap_or_default())
}
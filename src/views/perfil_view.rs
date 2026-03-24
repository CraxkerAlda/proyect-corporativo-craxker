pub fn vista_tabla_perfiles(perfiles: Vec<crate::models::Perfil>) -> String {
    let mut filas = String::new();
    for p in perfiles {
        filas.push_str(&format!(
            r#"
            <tr class="perfil-row">
                <td><strong>{nombre}</strong></td>
                <td>{tipo}</td>
                <td class="actions">
                    <a href="/vistas/perfiles/editar/{id}" class="btn-action edit" title="Editar Perfil">
                        <i class="fas fa-edit"></i>
                    </a>
                    <button class="btn-action delete" onclick="eliminarPerfil({id})" title="Eliminar Perfil">
                        <i class="fas fa-trash-alt"></i>
                    </button>
                </td>
            </tr>
            "#,
            nombre = p.strnombreperfil,
            tipo = if p.bitadministrador { 
                r#"<span class="badge admin"><i class="fas fa-shield-check"></i> Administrador</span>"# 
            } else { 
                r#"<span class="badge user"><i class="fas fa-user"></i> Estándar</span>"# 
            },
            id = p.id
        ));
    }

    format!(r##"
    <div class="breadcrumb">
        <a href="/dashboard"><i class="fas fa-home"></i> Inicio</a>
        <i class="fas fa-chevron-right separator"></i>
        <span>Seguridad</span>
        <i class="fas fa-chevron-right separator"></i>
        <span class="current">Perfiles</span>
    </div>

    <div class="card-table">
        <div class="table-header">
            <div>
                <h2 style="color: var(--primary-blue); margin:0;">Gestión de Perfiles</h2>
                <p style="color: #64748b; font-size: 0.85rem; margin-top: 5px;">Configure los niveles de acceso al sistema</p>
            </div>
            <a href="/vistas/perfiles/nuevo" class="btn-new-pro">
                <i class="fas fa-plus-circle"></i> Nuevo Perfil
            </a>
        </div>

        <div class="table-responsive">
            <table class="craxker-table" id="tablaPerfiles">
                <thead>
                    <tr>
                        <th>Nombre del Perfil</th>
                        <th>Tipo de Acceso</th>
                        <th style="text-align: center;">Acciones</th>
                    </tr>
                </thead>
                <tbody>
                    {filas}
                </tbody>
            </table>
        </div>

        <div class="pagination-container" style="display: flex; justify-content: space-between; align-items: center; margin-top: 20px;">
            <div class="pagination-btns">
                <button class="btn-page" id="btnPrev" onclick="cambiarPagina(-1)"><i class="fas fa-chevron-left"></i> Anterior</button>
                <button class="btn-page" id="btnNext" onclick="cambiarPagina(1)">Siguiente <i class="fas fa-chevron-right"></i></button>
            </div>
        </div>
    </div>

    <style>
        .card-table {{ background: white; border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.08); padding: 25px; }}
        .table-header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 25px; border-bottom: 1px solid #f1f5f9; padding-bottom: 20px; }}
        .btn-new-pro {{ background: var(--accent-green); color: white; text-decoration: none; padding: 12px 24px; border-radius: 8px; font-weight: 600; font-size: 0.9rem; display: inline-flex; align-items: center; gap: 8px; transition: 0.3s; box-shadow: 0 4px 10px rgba(40, 167, 69, 0.2); }}
        .craxker-table {{ width: 100%; border-collapse: collapse; }}
        .craxker-table th {{ text-align: left; padding: 15px; background: #f8fafc; color: var(--primary-blue); font-size: 0.85rem; text-transform: uppercase; border-bottom: 2px solid #e2e8f0; }}
        .craxker-table td {{ padding: 15px; border-bottom: 1px solid #f1f5f9; color: #475569; }}
        .badge {{ padding: 6px 12px; border-radius: 20px; font-size: 0.75rem; font-weight: bold; display: inline-flex; align-items: center; gap: 5px; }}
        .badge.admin {{ background: #e6fffa; color: #2c7a7b; }}
        .badge.user {{ background: #f1f5f9; color: #64748b; }}
        .btn-action {{ padding: 8px 10px; border-radius: 6px; text-decoration: none; display: inline-block; transition: 0.2s; border: none; cursor: pointer; }}
        .edit {{ background: #eef2ff; color: var(--primary-blue); }}
        .delete {{ background: #fff1f2; color: #e11d48; margin-left: 5px; }}
        .btn-page {{ padding: 8px 16px; border: 1px solid #ddd; background: white; border-radius: 6px; cursor: pointer; }}
        .btn-page:disabled {{ opacity: 0.5; cursor: not-allowed; }}
    </style>

    <script>
        let paginaActual = 1;
        const filasPorPagina = 5;

        function mostrarPagina(n) {{
            const filas = Array.from(document.querySelectorAll('.perfil-row'));
            const totalPaginas = Math.ceil(filas.length / filasPorPagina);
            if (n < 1) n = 1;
            if (n > totalPaginas) n = totalPaginas;
            paginaActual = n;

            filas.forEach((fila, index) => {{
                fila.style.display = (index >= (n - 1) * filasPorPagina && index < n * filasPorPagina) ? 'table-row' : 'none';
            }});

            document.getElementById('btnPrev').disabled = (paginaActual === 1);
            document.getElementById('btnNext').disabled = (paginaActual === totalPaginas || totalPaginas === 0);
        }}

        function cambiarPagina(delta) {{
            mostrarPagina(paginaActual + delta);
        }}

        async function eliminarPerfil(id) {{
            if (!confirm("¿Seguro de eliminar este perfil?")) return;
            try {{
                const res = await fetch(`/api/perfiles/${{id}}`, {{ 
                    method: 'DELETE',
                    headers: {{ 'Authorization': 'Bearer ' + localStorage.getItem('jwt_token') }}
                }});
                if (res.ok) location.reload();
                else alert("El perfil está en uso y no puede eliminarse.");
            }} catch(e) {{ alert("Error de conexión"); }}
        }}

        document.addEventListener('DOMContentLoaded', async () => {{
            await aplicarPermisosAcciones('PERFIL');
            mostrarPagina(1);
        }});
    </script>
    "##, filas = filas)
}


pub fn vista_nuevo_perfil() -> String {
    format!(r##"
    <div class="breadcrumb">
        <a href="/dashboard"><i class="fas fa-home"></i> Inicio</a>
        <i class="fas fa-chevron-right separator"></i>
        <span>Seguridad</span>
        <i class="fas fa-chevron-right separator"></i>
        <a href="/vistas/perfiles">Perfiles</a>
        <i class="fas fa-chevron-right separator"></i>
        <span class="current">Nuevo Perfil</span>
    </div>

    <div class="card-usuario" style="max-width: 550px; margin: 20px auto;">
        <div class="form-header">
            <h2 style="color: var(--primary-blue);"><i class="fas fa-shield-alt"></i> Crear Nuevo Perfil</h2>
            <p style="color: #64748b; font-size: 0.9rem; margin-top: 5px;">Defina un nombre y privilegios para el rol</p>
        </div>

        <form id="formNuevoPerfil" onsubmit="guardarPerfil(event)">
            <div class="form-group" style="margin-bottom: 20px;">
                <label style="font-weight: 600; color: var(--primary-blue); margin-bottom: 8px; display: block;">
                    <i class="fas fa-signature"></i> Nombre del Perfil:
                </label>
                <input type="text" id="nombre_perfil" required maxlength="30" 
                       placeholder="Ej. OPERADOR DE SISTEMA" 
                       style="width: 100%; padding: 12px; border: 1px solid #e2e8f0; border-radius: 8px; text-transform: uppercase; font-size: 1rem;">
                <small id="errNombre" style="color: #e11d48; font-size: 0.75rem; font-weight: 600;"></small>
            </div>
            
            <div class="form-group" style="display: flex; align-items: center; gap: 12px; padding: 15px; background: #f8fafc; border-radius: 10px; margin-bottom: 30px; border: 1px dashed #cbd5e1;">
                <input type="checkbox" id="es_admin" style="width: 20px; height: 20px; cursor: pointer;">
                <label for="es_admin" style="cursor: pointer; font-weight: 600; color: #1e293b;">
                    ¿Asignar privilegios de Administrador Maestro?
                </label>
            </div>

            <div class="form-actions" style="display: flex; gap: 15px; justify-content: center;">
                <button type="submit" class="btn-save" style="background: var(--accent-green); color: white; border: none; padding: 12px 35px; border-radius: 8px; cursor: pointer; font-weight: bold; transition: 0.3s;">
                    <i class="fas fa-check"></i> Guardar Perfil
                </button>
                <button type="button" class="btn-cancel" onclick="window.location.href='/vistas/perfiles'" 
                        style="background: #f1f5f9; color: #64748b; border: none; padding: 12px 35px; border-radius: 8px; cursor: pointer; font-weight: bold;">
                    Cancelar
                </button>
            </div>
        </form>
    </div>

    <script>
        async function guardarPerfil(e) {{
            e.preventDefault();
            
            const inputNombre = document.getElementById('nombre_perfil');
            const errNombre = document.getElementById('errNombre');
            const isAdmin = document.getElementById('es_admin').checked;
            const btn = e.target.querySelector('button[type="submit"]');
            
            const nombre = inputNombre.value.trim().toUpperCase();

            // Validaciones de detalle
            errNombre.innerText = "";
            
            if (nombre.length < 4) {{ 
                errNombre.innerText = "El nombre es muy corto (mínimo 4 caracteres)."; 
                return; 
            }}
            
            if (/(.)\1{{3,}}/.test(nombre)) {{ 
                errNombre.innerText = "El nombre contiene demasiados caracteres repetidos."; 
                return; 
            }}

            btn.disabled = true;
            btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Guardando...';

            try {{
                const res = await fetch('/api/perfiles', {{
                    method: 'POST',
                    headers: {{ 
                        'Content-Type': 'application/json',
                        'Authorization': 'Bearer ' + localStorage.getItem('jwt_token') 
                    }},
                    body: JSON.stringify({{ 
                        strnombreperfil: nombre, 
                        bitadministrador: isAdmin 
                    }})
                }});

                if (res.ok) {{
                    alert("Perfil creado exitosamente en Craxker Design Hub");
                    window.location.href = "/vistas/perfiles";
                }} else {{
                    const errorMsg = await res.text();
                    alert("Error: No se pudo crear el perfil. Verifique si ya existe o sus permisos.");
                    btn.disabled = false;
                    btn.innerHTML = '<i class="fas fa-check"></i> Guardar Perfil';
                }}
            }} catch (err) {{
                alert("Error crítico de conexión con el servidor.");
                btn.disabled = false;
                btn.innerHTML = '<i class="fas fa-check"></i> Guardar Perfil';
            }}
        }}

        document.addEventListener('DOMContentLoaded', async () => {{
            await aplicarPermisosAcciones('PERFIL', 'bitagregar');
        }});
        
    </script>
    "##)
}

pub fn vista_editar_perfil(perfil: crate::models::Perfil) -> String {
    format!(r##"
    <div class="breadcrumb">
        <a href="/dashboard"><i class="fas fa-home"></i> Inicio</a>
        <i class="fas fa-chevron-right separator"></i>
        <span>Seguridad</span>
        <i class="fas fa-chevron-right separator"></i>
        <a href="/vistas/perfiles">Perfiles</a>
        <i class="fas fa-chevron-right separator"></i>
        <span class="current">Editar Perfil</span>
    </div>
    
    <div class="card-usuario" style="max-width: 550px; margin: 20px auto; background: white; padding: 30px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.05);">
        <div class="form-header" style="text-align: center; margin-bottom: 30px;">
            <i class="fas fa-user-shield" style="font-size: 2.5rem; color: var(--primary-blue); margin-bottom: 10px;"></i>
            <h2 style="color: var(--primary-blue); margin: 0;">Modificar Perfil</h2>
        </div>

        <form id="formEditarPerfil" onsubmit="actualizarPerfil(event, {id})">
            <div class="form-group" style="margin-bottom: 20px; display: flex; flex-direction: column; gap: 8px;">
                <label style="font-weight: 600; color: var(--primary-blue);">Nombre del Perfil:</label>
                <input type="text" id="txtNombrePerfil" value="{nombre}" required maxlength="30"
                       style="width: 100%; padding: 12px; border: 1px solid #e2e8f0; border-radius: 8px; text-transform: uppercase; box-sizing: border-box;">
            </div>
            
            <div class="form-group" style="display: flex; align-items: center; gap: 12px; padding: 15px; background: #f8fafc; border-radius: 10px; margin-bottom: 30px; border: 1px dashed #cbd5e1;">
                <input type="checkbox" id="chkAdmin" {checked} style="width: 20px; height: 20px; cursor: pointer;">
                <label for="chkAdmin" style="cursor: pointer; font-weight: 600; color: #1e293b;">Habilitar privilegios de Administrador</label>
            </div>

            <div class="form-actions" style="display: flex; gap: 15px; justify-content: center;">
                <button type="submit" id="btnActualizar" class="btn-save" style="background: var(--primary-blue); color: white; border: none; padding: 12px 35px; border-radius: 8px; font-weight: bold; cursor: pointer;">
                    Actualizar
                </button>
                <button type="button" class="btn-cancel" onclick="window.location.href='/vistas/perfiles'" 
                        style="background: #f1f5f9; color: #64748b; border: none; padding: 12px 35px; border-radius: 8px; cursor: pointer; font-weight: bold;">
                    Volver
                </button>
            </div>
        </form>
    </div>

    <script>
        async function actualizarPerfil(e, id) {{
            e.preventDefault();
            const nombre = document.getElementById('txtNombrePerfil').value.trim().toUpperCase();
            const isAdmin = document.getElementById('chkAdmin').checked;
            const btn = document.getElementById('btnActualizar');

            btn.disabled = true;
            btn.innerHTML = 'Procesando...';

            try {{
                const res = await fetch(`/api/perfiles/${{id}}`, {{
                    method: 'PUT',
                    headers: {{ 
                        'Content-Type': 'application/json',
                        'Authorization': 'Bearer ' + localStorage.getItem('jwt_token')
                    }},
                    body: JSON.stringify({{ strnombreperfil: nombre, bitadministrador: isAdmin }})
                }});

                if (res.ok) {{
                    alert("Perfil actualizado correctamente");
                    window.location.href = "/vistas/perfiles";
                }} else {{
                    alert("Error al intentar actualizar el perfil");
                    btn.disabled = false;
                    btn.innerHTML = 'Actualizar';
                }}
            }} catch (err) {{
                alert("Error de conexión");
                btn.disabled = false;
                btn.innerHTML = 'Actualizar';
            }}
        }}

        document.addEventListener('DOMContentLoaded', async () => {{
            await aplicarPermisosAcciones('PERFIL', 'biteditar');
        }});
    </script>
    "##, 
    id = perfil.id, 
    nombre = perfil.strnombreperfil,
    checked = if perfil.bitadministrador { "checked" } else { "" })
}

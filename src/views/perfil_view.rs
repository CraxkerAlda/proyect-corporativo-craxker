pub fn vista_tabla_perfiles(perfiles: Vec<crate::models::Perfil>) -> String {
    let mut filas = String::new();
    for p in perfiles {
        let tipo_valor = if p.bitadministrador { "administrador" } else { "estandar" };
        filas.push_str(&format!(
            r#"
            <tr class="perfil-row" data-nombre="{nombre_lower}" data-tipo="{tipo_valor}">
                <td><strong>{nombre}</strong></td>
                <td>{tipo_badge}</td>
                <td class="actions">
                    <a href="/vistas/perfiles/editar/{id}" class="btn-action edit"><i class="fas fa-edit"></i></a>
                    <button class="btn-action delete" onclick="eliminarPerfil({id})"><i class="fas fa-trash-alt"></i></button>
                </td>
            </tr>
            "#,
            nombre_lower = p.strnombreperfil.to_lowercase(),
            nombre = p.strnombreperfil,
            tipo_valor = tipo_valor,
            tipo_badge = if p.bitadministrador {
                r#"<span class="badge admin"><i class="fas fa-shield-alt"></i> Administrador</span>"#
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
                <h2 style="color: var(--primary-blue); margin:0;">Módulo de Perfiles</h2>
                <p style="color: #64748b; font-size: 0.85rem; margin-top: 5px;">Gestión de roles y permisos del sistema.</p>
            </div>
            <a href="/vistas/perfiles/nuevo" class="btn-new-pro">
                <i class="fas fa-plus-circle"></i> Nuevo Perfil
            </a>
        </div>

        <!-- FILTROS -->
        <div class="filtros-container">
            <div class="filtro-grupo">
                <label class="filtro-label">BUSCAR POR NOMBRE</label>
                <div class="filtro-search-wrap">
                    <i class="fas fa-search filtro-icon"></i>
                    <input type="text" id="filtroBusqueda" class="filtro-input"
                           placeholder="Ej. Administrador, Ventas..."
                           oninput="aplicarFiltros()">
                </div>
            </div>
            <div class="filtro-grupo filtro-grupo-sm">
                <label class="filtro-label">TIPO DE PERFIL</label>
                <select id="filtroTipo" class="filtro-select" onchange="aplicarFiltros()">
                    <option value="">Todos los perfiles</option>
                    <option value="administrador">Administrador</option>
                    <option value="estandar">Estándar</option>
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
            <table class="craxker-table" id="tablaPerfiles">
                <thead>
                    <tr>
                        <th>Nombre del Perfil</th>
                        <th>Tipo de Acceso</th>
                        <th style="text-align: center;">Acciones</th>
                    </tr>
                </thead>
                <tbody>{filas}</tbody>
            </table>
            <div id="sinResultados" style="display:none; text-align:center; padding:40px; color:#94a3b8;">
                <i class="fas fa-search" style="font-size:2rem; margin-bottom:10px;"></i>
                <p style="margin:0; font-weight:600;">No hay perfiles con esos filtros</p>
            </div>
        </div>

        <div style="display:flex; justify-content:space-between; align-items:center; margin-top:20px;">
            <span id="contadorResultados" style="font-size:0.85rem; color:#64748b;"></span>
            <div class="pagination-btns">
                <button class="btn-page" id="btnPrev" onclick="cambiarPagina(-1)"><i class="fas fa-chevron-left"></i> Anterior</button>
                <button class="btn-page" id="btnNext" onclick="cambiarPagina(1)">Siguiente <i class="fas fa-chevron-right"></i></button>
            </div>
        </div>
    </div>

    <style>
        .card-table {{ background: white; border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.08); padding: 25px; }}
        .table-header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; border-bottom: 1px solid #f1f5f9; padding-bottom: 20px; }}
        .btn-new-pro {{ background: var(--accent-green); color: white; text-decoration: none; padding: 12px 24px; border-radius: 8px; font-weight: 600; font-size: 0.9rem; display: inline-flex; align-items: center; gap: 8px; transition: 0.3s; }}

        .filtros-container {{ display: flex; gap: 16px; align-items: flex-end; flex-wrap: wrap; background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 10px; padding: 18px 20px; margin-bottom: 20px; }}
        .filtro-grupo {{ display: flex; flex-direction: column; gap: 6px; flex: 1; min-width: 200px; }}
        .filtro-grupo-sm {{ flex: 0 0 200px; }}
        .filtro-grupo-xs {{ flex: 0 0 110px; }}
        .filtro-label {{ font-size: 0.85rem; font-weight: 700; color: var(--primary-blue); letter-spacing: 0.5px; text-transform: uppercase; }}
        .filtro-search-wrap {{ position: relative; }}
        .filtro-icon {{ position: absolute; left: 12px; top: 50%; transform: translateY(-50%); color: #94a3b8; font-size: 0.85rem; }}
        .filtro-input {{ width: 100%; padding: 10px 12px 10px 34px; border: 1px solid #e2e8f0; border-radius: 8px; font-size: 0.9rem; box-sizing: border-box; font-family: inherit; }}
        .filtro-input:focus {{ outline: none; border-color: var(--primary-blue); }}
        .filtro-select {{ width: 100%; padding: 10px 12px; border: 1px solid #e2e8f0; border-radius: 8px; font-size: 0.9rem; background: white; font-family: inherit; }}
        .filtro-btn-limpiar {{ width: 100%; padding: 10px 12px; background: white; border: 1px solid #e2e8f0; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 0.85rem; color: #64748b; display: flex; align-items: center; justify-content: center; gap: 6px; transition: 0.2s; }}
        .filtro-btn-limpiar:hover {{ background: #f1f5f9; color: #e11d48; border-color: #fecaca; }}

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
        let filasFiltradas = [];

        function aplicarFiltros() {{
            const busqueda = document.getElementById('filtroBusqueda').value.toLowerCase().trim();
            const tipo     = document.getElementById('filtroTipo').value;
            const todas    = Array.from(document.querySelectorAll('.perfil-row'));

            filasFiltradas = todas.filter(f => {{
                const okNombre = !busqueda || (f.dataset.nombre || '').includes(busqueda);
                const okTipo   = !tipo     || f.dataset.tipo === tipo;
                return okNombre && okTipo;
            }});

            todas.forEach(f => f.style.display = 'none');
            paginaActual = 1;
            mostrarPagina(1);
            document.getElementById('sinResultados').style.display =
                filasFiltradas.length === 0 ? 'block' : 'none';
        }}

        function limpiarFiltros() {{
            document.getElementById('filtroBusqueda').value = '';
            document.getElementById('filtroTipo').value = '';
            aplicarFiltros();
        }}

        function mostrarPagina(n) {{
            const base = filasFiltradas.length > 0
                ? filasFiltradas
                : Array.from(document.querySelectorAll('.perfil-row'));

            const total = Math.ceil(base.length / filasPorPagina);
            if (n < 1) n = 1;
            if (n > total) n = total;
            paginaActual = n;

            Array.from(document.querySelectorAll('.perfil-row')).forEach(f => f.style.display = 'none');
            base.forEach((f, i) => {{
                f.style.display = (i >= (n-1)*filasPorPagina && i < n*filasPorPagina) ? 'table-row' : 'none';
            }});

            document.getElementById('btnPrev').disabled = (paginaActual === 1);
            document.getElementById('btnNext').disabled = (paginaActual === total || total === 0);
        }}

        function cambiarPagina(delta) {{ mostrarPagina(paginaActual + delta); }}

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
            filasFiltradas = Array.from(document.querySelectorAll('.perfil-row'));
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

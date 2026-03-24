pub fn vista_matriz_permisos(perfiles: Vec<crate::models::Perfil>, modulos: Vec<crate::models::Modulo>) -> String {
    let mut opciones_perfil = String::new();
    for p in perfiles {
        opciones_perfil.push_str(&format!(r#"<option value="{}">{}</option>"#, p.id, p.strnombreperfil));
    }

    let mut filas_modulos = String::new();
    for m in modulos {
        filas_modulos.push_str(&format!(
            r#"
            <tr data-modulo-id="{id}">
                <td class="modulo-name">{nombre}</td>
                <td><input type="checkbox" class="chk-perm" data-accion="bitAgregar"></td>
                <td><input type="checkbox" class="chk-perm" data-accion="bitEditar"></td>
                <td><input type="checkbox" class="chk-perm" data-accion="bitEliminar"></td>
                <td><input type="checkbox" class="chk-perm" data-accion="bitConsulta"></td>
                <td><i class="fas fa-list-ul extra-icon"></i></td>
            </tr>
            "#,
            id = m.id,
            nombre = m.strnombremodulo
        ));
    }

    format!(r##"
    <div class="breadcrumb">
        <a href="/dashboard"><i class="fas fa-home"></i> Inicio</a>
        <i class="fas fa-chevron-right separator"></i>
        <span>Seguridad</span>
        <i class="fas fa-chevron-right separator"></i>
        <span class="current">Matriz de Permisos</span>
    </div>

    <div class="card-permisos">
        <div class="header-permisos">
            <i class="fas fa-key" style="font-size: 2rem; color: var(--primary-blue);"></i>
            <h2 style="margin:0; color: var(--primary-blue);">Configuración de Accesos</h2>
        </div>
        <hr class="linea-verde">

        <div class="search-area">
            <div style="flex: 1;">
                <label style="font-weight: bold; display: block; margin-bottom: 5px;">Perfil a configurar:</label>
                <select id="selectPerfil" onchange="cargarPermisosPerfil()">
                    <option value="">-- Seleccione un perfil para editar --</option>
                    {opciones_perfil}
                </select>
            </div>
        </div>

        <div class="table-container">
            <table class="tabla-azul">
                <thead>
                    <tr>
                        <th style="width: 30%;">Módulo</th>
                        <th>Agregar</th>
                        <th>Editar</th>
                        <th>Eliminar</th>
                        <th>Consultar</th>
                        <th>Estado</th>
                    </tr>
                </thead>
                <tbody id="body-permisos">
                    {filas_modulos}
                </tbody>
            </table>
        </div>

        <div class="footer-actions">
            <button id="btnGuardarPermisos" onclick="guardarTodosLosPermisos()" class="btn-save-blue">
                <i class="fas fa-save"></i> Guardar Cambios
            </button>
            <button onclick="window.location.href='/dashboard'" class="btn-cancel-gray">Cancelar</button>
        </div>
    </div>

    <style>
        .card-permisos {{ background: white; padding: 30px; border-radius: 12px; box-shadow: 0 10px 25px rgba(0,0,0,0.08); }}
        .header-permisos {{ display: flex; align-items: center; gap: 15px; margin-bottom: 15px; }}
        .linea-verde {{ border: 0; height: 4px; background: var(--accent-green); margin-bottom: 25px; border-radius: 2px; }}
        
        .search-area {{ background: #f8fafc; padding: 20px; border-radius: 8px; border: 1px solid #e2e8f0; margin-bottom: 20px; }}
        .search-area select {{ width: 100%; padding: 12px; border: 1px solid #cbd5e1; border-radius: 8px; font-size: 1rem; }}

        .table-container {{ overflow-x: auto; border-radius: 8px; border: 1px solid #e2e8f0; }}
        .tabla-azul {{ width: 100%; border-collapse: collapse; }}
        .tabla-azul thead {{ background: var(--primary-blue); color: white; }}
        .tabla-azul th, .tabla-azul td {{ padding: 12px; border-bottom: 1px solid #f1f5f9; text-align: center; }}
        .modulo-name {{ text-align: left !important; padding-left: 20px; font-weight: 600; color: #1e293b; }}
        
        .footer-actions {{ display: flex; justify-content: flex-end; gap: 12px; margin-top: 25px; padding-top: 20px; border-top: 1px solid #f1f5f9; }}
        
        .btn-save-blue {{ background: #0275d8; color: white; border: none; padding: 12px 35px; border-radius: 8px; cursor: pointer; font-weight: bold; transition: 0.3s; }}
        .btn-save-blue:hover {{ background: #025aa5; }}
        .btn-save-blue:disabled {{ background: #94a3b8; cursor: not-allowed; }}
        
        .btn-cancel-gray {{ background: #f1f5f9; color: #64748b; border: none; padding: 12px 35px; border-radius: 8px; cursor: pointer; font-weight: bold; }}

        input[type="checkbox"] {{ transform: scale(1.3); cursor: pointer; accent-color: var(--accent-green); }}
        
        .fa-spin-custom {{ animation: fa-spin 1s infinite linear; }}
    </style>

    <script>
        async function cargarPermisosPerfil() {{
            const idPerfil = document.getElementById('selectPerfil').value;
            if (!idPerfil) {{
                document.querySelectorAll('.chk-perm').forEach(c => c.checked = false);
                return;
            }}

            const res = await fetch(`/api/permisos/${{idPerfil}}`, {{
                headers: {{ 'Authorization': 'Bearer ' + localStorage.getItem('jwt_token') }}
            }});
            
            if (res.ok) {{
                const permisos = await res.json();
                document.querySelectorAll('.chk-perm').forEach(c => c.checked = false);
                
                permisos.forEach(p => {{
                    const fila = document.querySelector(`tr[data-modulo-id="${{p.idmodulo}}"]`);
                    if (fila) {{
                        fila.querySelector('[data-accion="bitAgregar"]').checked = p.bitagregar;
                        fila.querySelector('[data-accion="bitEditar"]').checked = p.biteditar;
                        fila.querySelector('[data-accion="bitEliminar"]').checked = p.biteliminar;
                        fila.querySelector('[data-accion="bitConsulta"]').checked = p.bitconsulta;
                    }}
                }});
            }}
        }}

        async function guardarTodosLosPermisos() {{
            const idPerfil = document.getElementById('selectPerfil').value;
            const btn = document.getElementById('btnGuardarPermisos');
            
            if(!idPerfil) return alert("Por favor seleccione un perfil primero.");

            btn.disabled = true;
            const originalHTML = btn.innerHTML;
            btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Guardando cambios...';

            const filas = document.querySelectorAll('#body-permisos tr');
            let errores = 0;

            for (const fila of filas) {{
                const idModulo = fila.getAttribute('data-modulo-id');
                const datos = {{
                    idperfil: parseInt(idPerfil),
                    idmodulo: parseInt(idModulo),
                    bitagregar: fila.querySelector('[data-accion="bitAgregar"]').checked,
                    biteditar: fila.querySelector('[data-accion="bitEditar"]').checked,
                    bitconsulta: fila.querySelector('[data-accion="bitConsulta"]').checked,
                    biteliminar: fila.querySelector('[data-accion="bitEliminar"]').checked,
                    bitdetalle: true
                }};

                try {{
                    const res = await fetch('/api/permisos', {{
                        method: 'POST',
                        headers: {{ 
                            'Content-Type': 'application/json',
                            'Authorization': 'Bearer ' + localStorage.getItem('jwt_token')
                        }},
                        body: JSON.stringify(datos)
                    }});
                    if(!res.ok) errores++;
                }} catch(e) {{ errores++; }}
            }}

            btn.disabled = false;
            btn.innerHTML = originalHTML;

            if(errores === 0) {{
                alert("Matriz de permisos actualizada");
            }} else {{
                alert("Se guardaron los permisos, pero hubo errores en " + errores + " módulos.");
            }}
        }}

        document.addEventListener('DOMContentLoaded', async () => {{
            // Primero verificar acceso general al módulo
            await aplicarPermisosAcciones('PERMISOS_PERFIL');

            // Luego verificar si puede EDITAR la matriz o solo consultarla
            const token = localStorage.getItem('jwt_token');
            if (!token) return;

            const res = await fetch('/api/permisos/mis-permisos', {{
                headers: {{ 'Authorization': 'Bearer ' + token }}
            }});

            if (!res.ok) return;

            const permisos = await res.json();
            const p = permisos.find(x => x.strnombremodulo === 'PERMISOS_PERFIL');

            // Si NO tiene biteditar → modo solo lectura
            if (!p || !p.biteditar) {{
                // Deshabilitar todos los checkboxes
                document.querySelectorAll('.chk-perm').forEach(chk => {{
                    chk.disabled = true;
                    chk.style.cursor = 'not-allowed';
                    chk.style.opacity = '0.5';
                }});

                // Ocultar botón guardar
                document.getElementById('btnGuardarPermisos').style.display = 'none';

                // Indicador visual de modo lectura
                const header = document.querySelector('.header-permisos');
                if (header) {{
                    const badge = document.createElement('span');
                    badge.innerHTML = '<i class="fas fa-eye"></i> Modo solo lectura';
                    badge.style.cssText = 'background:#f1f5f9; color:#64748b; padding:5px 14px; border-radius:20px; font-size:0.8rem; font-weight:600; margin-left:auto;';
                    header.appendChild(badge);
                }}
            }}
        }});
    </script>
    "##, opciones_perfil = opciones_perfil, filas_modulos = filas_modulos)
}
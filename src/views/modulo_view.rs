pub fn vista_tabla_modulos(modulos: Vec<crate::models::Modulo>) -> String {
    let mut filas = String::new();
    for m in modulos {
        filas.push_str(&format!(
            r#"
            <tr class="modulo-row">
                <td><i class="fas fa-cube" style="color: var(--accent-green); margin-right: 10px;"></i><strong>{nombre}</strong></td>
                <td class="actions" style="text-align: center;">
                    <a href="/vistas/modulo/editar/{id}" class="btn-action edit" title="Editar">
                        <i class="fas fa-edit"></i>
                    </a>
                    <button class="btn-action delete" onclick="eliminarModulo({id})" title="Eliminar">
                        <i class="fas fa-trash-alt"></i>
                    </button>
                </td>
            </tr>
            "#,
            nombre = m.strnombremodulo,
            id = m.id
        ));
    }

    format!(r##"
    <div class="breadcrumb">
        <a href="/dashboard"><i class="fas fa-home"></i> Inicio</a>
        <i class="fas fa-chevron-right separator"></i>
        <span>Seguridad</span>
        <i class="fas fa-chevron-right separator"></i>
        <span class="current">Módulos</span>
    </div>

    <div class="card-table">
        <div class="table-header">
            <div>
                <h2 style="color: var(--primary-blue); margin:0;">Módulos del Sistema</h2>
                <p style="color: #64748b; font-size: 0.85rem; margin-top: 5px;">Secciones activas en la barra de navegación</p>
            </div>
            <a href="/vistas/modulo/nuevo" class="btn-new-pro">
                <i class="fas fa-plus-circle"></i> Nuevo Módulo
            </a>
        </div>

        <div class="table-responsive">
            <table class="craxker-table" id="tablaModulos">
                <thead>
                    <tr>
                        <th>Nombre del Módulo</th>
                        <th style="text-align: center; width: 150px;">Acciones</th>
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
        .btn-new-pro {{ background: var(--accent-green); color: white; text-decoration: none; padding: 12px 24px; border-radius: 8px; font-weight: 600; font-size: 0.9rem; display: inline-flex; align-items: center; gap: 8px; transition: 0.3s; }}
        .craxker-table {{ width: 100%; border-collapse: collapse; }}
        .craxker-table th {{ text-align: left; padding: 15px; background: #f8fafc; color: var(--primary-blue); font-size: 0.85rem; text-transform: uppercase; border-bottom: 2px solid #e2e8f0; }}
        .craxker-table td {{ padding: 15px; border-bottom: 1px solid #f1f5f9; color: #475569; }}
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
            const filas = Array.from(document.querySelectorAll('.modulo-row'));
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

        async function eliminarModulo(id) {{
            if(!confirm("¿Deseas eliminar este módulo?")) return;
            try {{
                const res = await fetch(`/api/modulos/${{id}}`, {{ 
                    method: 'DELETE',
                    headers: {{ 'Authorization': 'Bearer ' + localStorage.getItem('jwt_token') }}
                }});
                if(res.ok) location.reload();
                else alert("Error al eliminar. Verifique permisos.");
            }} catch(e) {{ alert("Error de conexión"); }}
        }}

        document.addEventListener('DOMContentLoaded', async () => {{
            await aplicarPermisosAcciones('MODULOS');
            mostrarPagina(1);
        }});
    </script>
    "##, filas = filas)
}

pub fn vista_nuevo_modulo() -> String {
    format!(r##"
    <div class="breadcrumb">
        <a href="/dashboard"><i class="fas fa-home"></i> Inicio</a>
        <i class="fas fa-chevron-right separator"></i>
        <span>Seguridad</span>
        <i class="fas fa-chevron-right separator"></i>
        <a href="/vistas/modulo">Módulos</a>
        <i class="fas fa-chevron-right separator"></i>
        <span class="current">Nuevo Registro</span>
    </div>

    <div class="card-usuario" style="max-width: 550px; margin: 20px auto;">
        <div class="form-header" style="text-align: center; margin-bottom: 30px;">
            <i class="fas fa-cubes" style="font-size: 3.5rem; color: var(--accent-green); margin-bottom: 15px;"></i>
            <h2 style="color: var(--primary-blue); margin: 0;">Registrar Nuevo Módulo</h2>
            <p style="color: #64748b; font-size: 0.9rem; margin-top: 8px;">Asigne un nombre único para la nueva sección</p>
        </div>

        <form id="formModulo" onsubmit="guardarModulo(event)">
            <div class="form-group" style="display: flex; flex-direction: column; gap: 10px;">
                <label style="font-weight: 700; color: var(--primary-blue); font-size: 0.9rem;">
                    <i class="fas fa-tag"></i> Nombre del Módulo:
                </label>
                <input type="text" id="nombre_modulo" required maxlength="30" 
                       placeholder="EJ. CONTROL DE PAGOS" 
                       style="padding: 14px; border: 1px solid #e2e8f0; border-radius: 10px; text-transform: uppercase; font-size: 1rem; transition: 0.3s;">
                <small id="errNombre" style="color: #e11d48; font-size: 0.8rem; font-weight: 600;"></small>
            </div>

            <div class="form-actions" style="margin-top: 35px; display: flex; gap: 15px; justify-content: center;">
                <button type="submit" id="btnGuardar" class="btn-save" 
                        style="background: var(--primary-blue); color: white; border: none; padding: 12px 40px; border-radius: 8px; cursor: pointer; font-weight: bold; display: flex; align-items: center; gap: 8px;">
                    <i class="fas fa-save"></i> Guardar Módulo
                </button>
                <button type="button" class="btn-cancel" onclick="window.location.href='/vistas/modulo'" 
                        style="background: #f1f5f9; color: #64748b; border: none; padding: 12px 40px; border-radius: 8px; cursor: pointer; font-weight: bold;">
                    Cancelar
                </button>
            </div>
        </form>
    </div>

    <script>
        async function guardarModulo(e) {{
            e.preventDefault();
            
            const input = document.getElementById('nombre_modulo');
            const btn = document.getElementById('btnGuardar');
            const err = document.getElementById('errNombre');
            const nombre = input.value.trim().toUpperCase();

            err.innerText = "";
            if(nombre.length < 3) {{
                err.innerText = "El nombre es demasiado corto.";
                return;
            }}

            btn.disabled = true;
            btn.innerHTML = '<i class="fas fa-circle-notch fa-spin"></i> Procesando...';

            try {{
                const res = await fetch('/api/modulos', {{
                    method: 'POST',
                    headers: {{ 
                        'Content-Type': 'application/json',
                        'Authorization': 'Bearer ' + localStorage.getItem('jwt_token')
                    }},
                    body: JSON.stringify({{ strnombremodulo: nombre }})
                }});

                if (res.ok) {{
                    alert("¡Módulo '" + nombre + "' registrado con éxito!");
                    window.location.href = "/vistas/modulo";
                }} else {{
                    const msg = await res.text();
                    alert("Error del servidor: " + msg);
                    btn.disabled = false;
                    btn.innerHTML = '<i class="fas fa-save"></i> Guardar Módulo';
                }}
            }} catch (error) {{
                console.error("Error en fetch:", error);
                alert("Error de conexión. Asegúrate de que el servidor esté corriendo.");
                btn.disabled = false;
                btn.innerHTML = '<i class="fas fa-save"></i> Guardar Módulo';
            }}
        }}

        document.addEventListener('DOMContentLoaded', async () => {{
            await aplicarPermisosAcciones('MODULOS', 'bitagregar');
        }});

    </script>
    "##)
}


pub fn vista_editar_modulo(m: crate::models::Modulo) -> String {
    format!(r##"
    <div class="breadcrumb">
        <a href="/dashboard"><i class="fas fa-home"></i> Inicio</a>
        <i class="fas fa-chevron-right separator"></i>
        <span>Seguridad</span>
        <i class="fas fa-chevron-right separator"></i>
        <a href="/vistas/modulo">Módulos</a>
        <i class="fas fa-chevron-right separator"></i>
        <span class="current">Editar</span>
    </div>

    <div class="card-usuario" style="max-width: 500px; margin: 20px auto; background: white; padding: 30px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.05);">
        <div class="form-header" style="text-align: center; margin-bottom: 30px;">
            <i class="fas fa-edit" style="font-size: 2.5rem; color: var(--primary-blue); margin-bottom: 10px;"></i>
            <h2 style="color: var(--primary-blue); margin: 0;">Modificar Módulo</h2>
        </div>
        <form onsubmit="actualizarModulo(event, {id})">
            <div class="form-group" style="display: flex; flex-direction: column; gap: 8px;">
                <label style="font-weight: 600; color: var(--primary-blue);">Nombre del Módulo:</label>
                <input type="text" id="nombre_modulo" value="{nombre}" required 
                       style="padding: 12px; border: 1px solid #e2e8f0; border-radius: 8px; text-transform: uppercase;">
            </div>
            <div class="form-actions" style="margin-top: 30px; display: flex; gap: 15px; justify-content: center;">
                <button type="submit" id="btnActualizar" class="btn-save" style="background: var(--accent-green); color: white; border: none; padding: 12px 30px; border-radius: 8px; font-weight: bold; cursor: pointer;">Actualizar</button>
                <button type="button" class="btn-cancel" onclick="window.location.href='/vistas/modulo'" style="background: #f1f5f9; padding: 12px 30px; border-radius: 8px; border: none; cursor: pointer;">Volver</button>
            </div>
        </form>
    </div>

    <script>
        async function actualizarModulo(e, id) {{
            e.preventDefault();
            const nombre = document.getElementById('nombre_modulo').value.toUpperCase();
            const btn = document.getElementById('btnActualizar');
            
            btn.disabled = true;
            btn.innerHTML = 'Procesando...';

            try {{
                const res = await fetch(`/api/modulos/${{id}}`, {{
                    method: 'PUT',
                    headers: {{ 
                        'Content-Type': 'application/json',
                        'Authorization': 'Bearer ' + localStorage.getItem('jwt_token')
                    }},
                    body: JSON.stringify({{ strnombremodulo: nombre }})
                }});

                if (res.ok) {{
                    alert("Módulo actualizado con éxito");
                    window.location.href = "/vistas/modulo";
                }} else {{
                    const msg = await res.text();
                    alert("Error: " + msg);
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
            await aplicarPermisosAcciones('MODULOS', 'biteditar');
        }});
    </script>
    "##, id = m.id, nombre = m.strnombremodulo)
}
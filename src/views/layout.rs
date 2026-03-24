pub fn layout_maestro(titulo: &str, contenido: String) -> String {
    format!(r##"
    <!DOCTYPE html>
    <html lang="es">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>{titulo} | Craxker Design Hub</title>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css">
        <style>
            :root {{
                --primary-blue: #1a3a5a;
                --accent-green: #28a745;
                --sidebar-width: 260px;
                --white: #ffffff;
                --bg-gray: #f8fafc;
            }}
            body {{ font-family: 'Inter', sans-serif; margin: 0; background: var(--bg-gray); display: flex; flex-direction: column; min-height: 100vh; }}
            
            .top-nav {{ 
                background: var(--primary-blue); 
                color: white; 
                padding: 0.8rem 2rem; 
                display: flex; 
                justify-content: space-between; 
                align-items: center; 
                border-bottom: 4px solid var(--accent-green); 
                position: sticky; top: 0; z-index: 1000; 
            }}

            .nav-brand {{ font-size: 1.4rem; font-weight: 800; letter-spacing: -1px; }}
            .nav-brand a {{ color: white; text-decoration: none; transition: 0.3s; }}
            .nav-brand a:hover {{ opacity: 0.8; }}

            .menu-container {{ display: flex; gap: 1rem; }}
            .menu-item {{ position: relative; padding: 10px 15px; cursor: pointer; border-radius: 6px; transition: 0.3s; color: white; display: flex; align-items: center; gap: 8px; font-weight: 500; }}
            .menu-item:hover {{ background: rgba(255,255,255,0.1); }}
            
            .dropdown {{ display: none; position: absolute; top: 100%; left: 0; background: var(--white); min-width: 220px; border-radius: 0 0 8px 8px; box-shadow: 0 10px 15px -3px rgba(0,0,0,0.1); overflow: hidden; }}
            .menu-item:hover .dropdown {{ display: block; }}
            
            .dropdown a {{ display: block; padding: 12px 20px; color: var(--primary-blue); text-decoration: none; font-size: 0.9rem; border-bottom: 1px solid #f1f5f9; }}
            .dropdown a:hover {{ background: #f1f5f9; color: var(--accent-green); padding-left: 25px; transition: 0.3s; }}
            
            .user-pill {{ display: flex; align-items: center; gap: 10px; background: rgba(255,255,255,0.1); padding: 5px 15px; border-radius: 30px; cursor: pointer; position: relative; }}
            .user-avatar {{ width: 32px; height: 32px; border-radius: 50%; object-fit: cover; border: 2px solid var(--accent-green); background: #eee; }}
            
            .user-menu {{ display: none; position: absolute; top: 110%; right: 0; background: white; min-width: 180px; border-radius: 8px; box-shadow: 0 10px 25px rgba(0,0,0,0.15); overflow: hidden; z-index: 1100; }}
            .user-menu.show {{ display: block; }}
            .user-menu button {{ width: 100%; border: none; background: none; padding: 12px 20px; text-align: left; cursor: pointer; color: #e11d48; font-weight: bold; }}
            
            .modal-overlay {{ display: none; position: fixed; top:0; left:0; width:100%; height:100%; background: rgba(0,0,0,0.5); z-index: 2000; align-items: center; justify-content: center; backdrop-filter: blur(4px); }}
            .modal-exit {{ background: white; padding: 30px; border-radius: 16px; width: 90%; max-width: 400px; text-align: center; }}
            
            .main-content {{ padding: 2rem 5%; flex: 1; }}
        </style>
    </head>
    <body>
        <header class="top-nav">
            <div class="nav-brand">
                <a href="/dashboard">CRAXKER DESIGN HUB</a>
            </div>
            <div class="menu-container" id="main-menu"></div>
            <div class="user-pill" onclick="toggleMenu(event)">
                <span id="nav-user-name">Cargando...</span>
                <img id="nav-user-img" 
                     src="/uploads/default.png" 
                     class="user-avatar" 
                     onerror="this.src='/uploads/default.png'; this.onerror=null;">
                
                <div class="user-menu" id="userMenu">
                    <button onclick="showExitModal()"><i class="fas fa-sign-out-alt"></i> Cerrar Sesión</button>
                </div>
            </div>
        </header>

        <main class="main-content">
            {contenido}
        </main>

        <div id="exitModal" class="modal-overlay">
            <div class="modal-exit">
                <i class="fas fa-door-open" style="font-size: 3rem; color: #e11d48; margin-bottom: 15px;"></i>
                <h2 style="margin: 0; color: var(--primary-blue);">¿Cerrar Sesión?</h2>
                <p style="color: #64748b;">Estás a punto de salir de Craxker Design Hub.</p>
                <div style="margin-top:20px; display: flex; gap: 10px; justify-content: center;">
                    <button style="background:#e11d48; color:white; border:none; padding:10px 20px; border-radius:8px; cursor:pointer; font-weight:bold;" onclick="confirmLogout()">Salir</button>
                    <button style="background:#f1f5f9; border:none; padding:10px 20px; border-radius:8px; cursor:pointer; font-weight:bold;" onclick="hideExitModal()">Cancelar</button>
                </div>
            </div>
        </div>

        <script>
            // SEGURIDAD: Verificar si tiene permiso de consulta en el módulo actual
            async function aplicarPermisosAcciones(nombreModulo) {{
                const token = localStorage.getItem('jwt_token');
                if (!token) return;

                try {{
                    const res = await fetch('/api/permisos/mis-permisos', {{ 
                        headers: {{ 'Authorization': 'Bearer ' + token }} 
                    }});
                    if (!res.ok) return;

                    const permisos = await res.json();
                    const p = permisos.find(x => x.strnombremodulo === nombreModulo);

                    if (!p || !p.bitconsulta) {{
                        window.location.href = "/error-403";
                        return;
                    }}

                    if (!p.bitagregar) document.querySelectorAll('.btn-new-pro, .btn-save').forEach(el => el.style.display = 'none');
                    if (!p.biteditar) document.querySelectorAll('.edit').forEach(el => el.style.display = 'none');
                    if (!p.biteliminar) document.querySelectorAll('.delete').forEach(el => el.style.display = 'none');
                }} catch(e) {{ console.error(e); }}
            }}

            document.addEventListener('DOMContentLoaded', async () => {{
                const token = localStorage.getItem('jwt_token');
                if(!token) {{ window.location.href = '/login'; return; }}

                // Cargar datos de usuario
                const userName = localStorage.getItem('user_name') || 'Usuario';
                const userImg = localStorage.getItem('user_img');
                
                document.getElementById('nav-user-name').innerText = userName;
                
                // Si existe imagen en localStorage, intentar cargarla
                if(userImg && userImg !== 'null') {{
                    document.getElementById('nav-user-img').src = userImg;
                }}

                try {{
                    const res = await fetch('/dinamico', {{ headers: {{ 'Authorization': 'Bearer ' + token }} }});
                    if (res.ok) {{
                        const data = await res.json();
                        const menuContainer = document.getElementById('main-menu');
                        const grupos = {{}};
                        
                        data.forEach(item => {{
                            if (!grupos[item.parent_name]) grupos[item.parent_name] = [];
                            grupos[item.parent_name].push(item);
                        }});

                        for (const [nombrePadre, modulos] of Object.entries(grupos)) {{
                            let icon = 'folder';
                            if (nombrePadre === 'Seguridad') icon = 'shield-alt';
                            if (nombrePadre === 'Principal 1') icon = 'hospital';
                            if (nombrePadre === 'Principal 2') icon = 'th-large';

                            let linksHtml = '';
                            modulos.forEach(m => {{
                                let ruta = m.modulo_name.toLowerCase()
                                    .replace('permisos_perfil', 'permisos')
                                    .replace('perfil', 'perfiles')
                                    .replace('modulos', 'modulo')
                                    .replace(/\s+/g, '_').replace(/\./g, '_');
                                
                                linksHtml += `<a href="/vistas/${{ruta}}">${{m.modulo_name}}</a>`;
                            }});

                            menuContainer.innerHTML += `
                                <div class="menu-item">
                                    <i class="fas fa-${{icon}}"></i> ${{nombrePadre}}
                                    <div class="dropdown">${{linksHtml}}</div>
                                </div>`;
                        }}
                    }}
                }} catch(e) {{ console.error(e); }}
            }});

            function toggleMenu(e) {{
                e.stopPropagation();
                document.getElementById('userMenu').classList.toggle('show');
            }}
            window.onclick = () => {{
                const m = document.getElementById('userMenu');
                if(m) m.classList.remove('show');
            }};
            function showExitModal() {{ document.getElementById('exitModal').style.display = 'flex'; }}
            function hideExitModal() {{ document.getElementById('exitModal').style.display = 'none'; }}
            function confirmLogout() {{ localStorage.clear(); window.location.href = '/login'; }}
        </script>
    </body>
    </html>
    "##, titulo = titulo, contenido = contenido)
}
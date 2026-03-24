pub fn vista_login(site_key: &str) -> String {
    format!(r##"
    <!DOCTYPE html>
    <html lang="es">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Login | Craxker Design Hub</title>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css">
        <script src="https://www.google.com/recaptcha/api.js" async defer></script>
        <style>
            :root {{ --primary: #1a3a5a; --accent: #28a745; --bg: #f1f5f9; }}
            body {{ font-family: 'Inter', sans-serif; background: var(--bg); display: flex; align-items: center; justify-content: center; min-height: 100vh; margin: 0; }}
            .login-card {{ background: white; padding: 40px; border-radius: 16px; box-shadow: 0 10px 25px rgba(0,0,0,0.1); width: 100%; max-width: 400px; text-align: center; }}
            .logo-area {{ margin-bottom: 30px; }}
            .logo-area h1 {{ color: var(--primary); font-size: 1.5rem; margin: 0; letter-spacing: -1px; }}
            .form-group {{ text-align: left; margin-bottom: 20px; }}
            .form-group label {{ display: block; font-weight: 600; color: #475569; font-size: 0.85rem; margin-bottom: 8px; }}
            .form-group input {{ width: 100%; padding: 12px; border: 1px solid #e2e8f0; border-radius: 8px; box-sizing: border-box; font-size: 1rem; transition: 0.3s; }}
            .btn-login {{ background: var(--primary); color: white; border: none; width: 100%; padding: 14px; border-radius: 8px; font-weight: bold; cursor: pointer; font-size: 1rem; transition: 0.3s; }}
            .btn-login:disabled {{ opacity: 0.7; cursor: not-allowed; }}
            #msg-error {{ color: #e11d48; font-size: 0.85rem; margin-top: 15px; font-weight: 600; min-height: 1.2rem; }}
            .g-recaptcha {{ margin: 15px 0; display: flex; justify-content: center; }}
        </style>
    </head>
    <body>
        <div class="login-card">
            <div class="logo-area">
                <h1>CRAXKER DESIGN HUB</h1>
                <p style="color: #64748b; font-size: 0.9rem;">Acceso al Sistema Profesional</p>
            </div>

            <form id="loginForm" onsubmit="ejecutarLogin(event)">
                <div class="form-group">
                    <label><i class="fas fa-user"></i> Usuario</label>
                    <input type="text" id="usuario" required placeholder="Nombre de usuario">
                </div>
                <div class="form-group">
                    <label><i class="fas fa-lock"></i> Contraseña</label>
                    <input type="password" id="password" required placeholder="••••••••">
                </div>

                <div class="g-recaptcha" data-sitekey="{site_key}"></div>

                <button type="submit" id="btnLogin" class="btn-login">
                    <i class="fas fa-sign-in-alt"></i> Iniciar Sesión
                </button>
            </form>
            <div id="msg-error"></div>
        </div>

        <script>
            async function ejecutarLogin(e) {{
                e.preventDefault();
                const user = document.getElementById('usuario').value;
                const pass = document.getElementById('password').value;
                const captcha = grecaptcha.getResponse();
                const btn = document.getElementById('btnLogin');
                const errorDiv = document.getElementById('msg-error');

                if(!captcha) {{ errorDiv.innerText = "Por favor, verifica el captcha."; return; }}

                btn.disabled = true;
                btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Validando...';
                errorDiv.innerText = "";

                try {{
                    const res = await fetch('/api/auth/login', {{
                        method: 'POST',
                        headers: {{ 'Content-Type': 'application/json' }},
                        body: JSON.stringify({{ usuario: user, password: pass, captcha_token: captcha }})
                    }});

                    if(res.ok) {{
                        const data = await res.json();
                        localStorage.setItem('jwt_token', data.token);
                        localStorage.setItem('user_name', data.nombre);
                        localStorage.setItem('user_img', data.imagen);
                        localStorage.setItem('perfil_id', data.perfil_id);
                        window.location.href = "/dashboard";
                    }} else {{
                        // Capturamos el mensaje de error del servidor (Contraseña incorrecta, inactivo, etc)
                        const errorMsgFromServer = await res.text();
                        errorDiv.innerText = errorMsgFromServer;
                        
                        btn.disabled = false;
                        btn.innerHTML = '<i class="fas fa-sign-in-alt"></i> Iniciar Sesión';
                        grecaptcha.reset();
                    }}
                }} catch(err) {{
                    errorDiv.innerText = "Error crítico de conexión.";
                    btn.disabled = false;
                    btn.innerHTML = '<i class="fas fa-sign-in-alt"></i> Iniciar Sesión';
                }}
            }}
        </script>
    </body>
    </html>
    "##, site_key = site_key)
}
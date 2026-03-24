use crate::views::layout::layout_maestro;

pub fn vista_error_personalizada(codigo: &str, mensaje: &str, sub_mensaje: &str, icono: &str) -> String {
    let contenido = format!(r##"
    <div style="display: flex; align-items: center; justify-content: center; min-height: 80vh; font-family: 'Inter', sans-serif;">
        <div style="position: relative; width: 100%; max-width: 500px; padding: 40px; text-align: center;">
            
            <div style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); width: 300px; height: 300px; background: radial-gradient(circle, rgba(26,58,90,0.05) 0%, rgba(255,255,255,0) 70%); z-index: 0;"></div>

            <div style="position: relative; z-index: 1;">
                <div style="margin-bottom: 20px;">
                    <i class="fas {icono}" style="font-size: 4rem; color: var(--primary-blue); opacity: 0.8;"></i>
                </div>

                <h1 style="font-size: 6rem; font-weight: 900; margin: 0; color: var(--primary-blue); letter-spacing: -4px; line-height: 0.8; opacity: 0.1;">
                    {codigo}
                </h1>
                
                <h2 style="font-size: 1.8rem; font-weight: 700; color: var(--primary-blue); margin-top: -20px; text-transform: uppercase;">
                    {mensaje}
                </h2>

                <div style="width: 40px; height: 4px; background: var(--accent-green); margin: 20px auto;"></div>

                <p style="color: #64748b; font-size: 1rem; line-height: 1.6; margin-bottom: 35px; font-weight: 400;">
                    {sub_mensaje}
                </p>

                <div style="display: flex; gap: 12px; justify-content: center;">
                    <a href="/dashboard" style="background: var(--primary-blue); color: white; text-decoration: none; padding: 12px 25px; border-radius: 8px; font-weight: 700; transition: 0.3s; font-size: 0.85rem; text-transform: uppercase; letter-spacing: 1px; display: inline-block;">
                        Ir al inicio
                    </a>
                </div>
            </div>
        </div>
    </div>

    <style>
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;700;900&display=swap');
        body {{ background-color: #f8fafc !important; }}
        button:hover {{ background: var(--primary-blue) !important; color: white !important; }}
        a:hover {{ transform: translateY(-2px); box-shadow: 0 10px 20px rgba(26,58,90,0.15); }}
    </style>
    "##, codigo = codigo, mensaje = mensaje, sub_mensaje = sub_mensaje, icono = icono);

    layout_maestro(mensaje, contenido)
}
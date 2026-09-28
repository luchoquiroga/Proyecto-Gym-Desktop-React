// El shell no tiene lógica propia: abre la ventana definida en tauri.conf.json,
// que carga la web (devUrl en desarrollo, frontendDist en el instalador).
// Sin comandos ni plugins: la página no puede pedirle nada al sistema.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("no se pudo iniciar la aplicación de escritorio");
}

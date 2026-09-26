#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        // Aucune commande métier avant la validation Windows du module 1.
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("Impossible de démarrer Windows Maintenance Tool");
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ytmusic::{app, global_shortcuts, tray};

fn main() {
    let builder = tauri::Builder::default()
        .setup(app::handle_setup)
        .system_tray(tray::get_system_tray())
        .on_system_tray_event(tray::handle_tray_event);

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(global_shortcuts::register_global_shortcuts);
}

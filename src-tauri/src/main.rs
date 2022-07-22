#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    CustomMenuItem, GlobalShortcutManager, Manager, RunEvent, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};

fn main() {
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let pause = CustomMenuItem::new("play_pause".to_string(), "Play/Pause");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(pause)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    let builder = tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "play_pause" => {
                    let window = app.get_window("main").unwrap();
                    window
                        .eval("document.querySelector('#play-pause-button').click() ")
                        .unwrap();
                }
                "prev" => {
                    let window = app.get_window("main").unwrap();
                    window
                        .eval("document.querySelector('.previous-button').click() ")
                        .unwrap();
                }
                "hide" => {
                    let item_handle = app.tray_handle().get_item(&id);
                    let window = app.get_window("main").unwrap();

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                        item_handle.set_title("Show").unwrap();
                    } else {
                        window.show().unwrap();
                        item_handle.set_title("Hide").unwrap();
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        });

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, e| match e {
        RunEvent::Ready => {
            let i_app_handle = app_handle.clone();
            let mut gsm = app_handle.global_shortcut_manager();

            gsm.register("CmdOrCtrl+Option+p", move || {
                let app_handle = i_app_handle.clone();
                let window = app_handle.get_window("main").unwrap();
                window
                    .eval("document.querySelector('#play-pause-button').click() ")
                    .unwrap();
            })
            .unwrap();

            let i_app_handle = app_handle.clone();
            gsm.register("CmdOrCtrl+Option+n", move || {
                let app_handle = i_app_handle.clone();
                let window = app_handle.get_window("main").unwrap();
                window
                    .eval("document.querySelector('.next-button').click() ")
                    .unwrap();
            })
            .unwrap();
        }
        _ => {}
    })
}

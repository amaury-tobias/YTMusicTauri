use tauri::Manager;

enum TrayIdentifier {
    PlayPause,
    Previous,
    Next,

    Hide,
    Show,
    Quit,
    Unimplemented,
}

impl Into<String> for TrayIdentifier {
    fn into(self) -> String {
        match self {
            TrayIdentifier::PlayPause => "play_pause".to_owned(),
            TrayIdentifier::Previous => "prev".to_owned(),
            TrayIdentifier::Next => "next".to_owned(),

            TrayIdentifier::Hide => "hide".to_owned(),
            TrayIdentifier::Show => "show".to_owned(),
            TrayIdentifier::Quit => "quit".to_owned(),
            TrayIdentifier::Unimplemented => "unimplemented".to_owned(),
        }
    }
}

impl From<String> for TrayIdentifier {
    fn from(s: String) -> TrayIdentifier {
        match s.as_str() {
            "play_pause" => TrayIdentifier::PlayPause,
            "prev" => TrayIdentifier::Previous,
            "next" => TrayIdentifier::Next,

            "hide" => TrayIdentifier::Hide,
            "show" => TrayIdentifier::Show,
            "quit" => TrayIdentifier::Quit,
            _ => TrayIdentifier::Unimplemented,
        }
    }
}

pub fn get_system_tray_menu(is_hiden: bool) -> tauri::SystemTrayMenu {
    let play_pause_item = tauri::CustomMenuItem::new(TrayIdentifier::PlayPause, "Play/Pause");
    let previous_item = tauri::CustomMenuItem::new(TrayIdentifier::Previous, "Previous");
    let next_item = tauri::CustomMenuItem::new(TrayIdentifier::Next, "Next");
    let hide_item = tauri::CustomMenuItem::new(TrayIdentifier::Hide, "Hide");
    let show_item = tauri::CustomMenuItem::new(TrayIdentifier::Show, "Show");
    let quit_item = tauri::CustomMenuItem::new(TrayIdentifier::Quit, "Quit");

    tauri::SystemTrayMenu::new()
        .add_item(play_pause_item)
        .add_item(previous_item)
        .add_item(next_item)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(match is_hiden {
            true => show_item,
            false => hide_item,
        })
        .add_item(quit_item)
}

pub fn get_system_tray() -> tauri::SystemTray {
    let tray_menu = get_system_tray_menu(false);
    tauri::SystemTray::new().with_menu(tray_menu)
}

pub fn handle_tray_event(app: &tauri::AppHandle<tauri::Wry>, event: tauri::SystemTrayEvent) {
    let main_window = app
        .get_window("main")
        .expect("Could not get the main window.");

    match event {
        tauri::SystemTrayEvent::MenuItemClick { id, .. } => {
            let tray_identifier = TrayIdentifier::from(id.clone());

            match tray_identifier {
                TrayIdentifier::PlayPause => {
                    main_window
                        .eval("document.querySelector('#play-pause-button').click() ")
                        .expect("Could not click the play/pause button.");
                }
                TrayIdentifier::Previous => {
                    main_window
                        .eval("document.querySelector('.previous-button').click() ")
                        .expect("Could not click the previous button.");
                }
                TrayIdentifier::Next => {
                    main_window
                        .eval("document.querySelector('.next-button').click() ")
                        .expect("Could not click the next button.");
                }
                TrayIdentifier::Hide => {
                    main_window.hide().expect("Could not hide the main window.");
                    app.tray_handle()
                        .set_menu(get_system_tray_menu(true))
                        .expect("Could not set system tray menu.");
                }
                TrayIdentifier::Show => {
                    main_window.show().expect("Could not show the main window.");
                    app.tray_handle()
                        .set_menu(get_system_tray_menu(false))
                        .expect("Could not set system tray menu.");
                }
                TrayIdentifier::Quit => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        _ => {}
    }
}

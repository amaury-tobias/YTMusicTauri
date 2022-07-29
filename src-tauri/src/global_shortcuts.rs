use tauri::{GlobalShortcutManager, Manager};

enum GlobalShortcutAccelerator {
    PlayPause,
    Previous,
    Next,
    Shuffle,
    Repeat,

    Unimplemented,
}

impl Default for GlobalShortcutAccelerator {
  fn default() -> Self {
      Self::Unimplemented
  }
}

impl Into<&str> for GlobalShortcutAccelerator {
  fn into(self) -> &'static str {
      match self {
          GlobalShortcutAccelerator::PlayPause => "CmdOrCtrl+Option+p",
          GlobalShortcutAccelerator::Previous => "CmdOrCtrl+Option+[",
          GlobalShortcutAccelerator::Next => "CmdOrCtrl+Option+]",
          GlobalShortcutAccelerator::Shuffle => "CmdOrCtrl+Option+s",
          GlobalShortcutAccelerator::Repeat => "CmdOrCtrl+Option+r",

          GlobalShortcutAccelerator::Unimplemented => "unimplemented",
      }
  }
}


pub fn register_global_shortcuts(app: &tauri::AppHandle, event: tauri::RunEvent) {
    match event {
        tauri::RunEvent::Ready => {
            let app_handle: tauri::AppHandle = app.clone();
            let mut gsm = app_handle.global_shortcut_manager();

            gsm.register(GlobalShortcutAccelerator::PlayPause.into(), move || {
                let app_handle = app_handle.clone();
                let window = app_handle.get_window("main").unwrap();
                window
                    .eval("document.querySelector('#play-pause-button').click() ")
                    .unwrap();
            })
            .expect("error while registering play/pause shortcut");

            let app_handle: tauri::AppHandle = app.clone();
            gsm.register(GlobalShortcutAccelerator::Previous.into(), move || {
                let app_handle = app_handle.clone();
                let window = app_handle.get_window("main").unwrap();
                window
                    .eval("document.querySelector('.previous-button').click() ")
                    .unwrap();
            })
            .expect("error while registering previous shortcut");

            let app_handle: tauri::AppHandle = app.clone();
            gsm.register(GlobalShortcutAccelerator::Next.into(), move || {
                let app_handle = app_handle.clone();
                let window = app_handle.get_window("main").unwrap();
                window
                    .eval("document.querySelector('.next-button').click() ")
                    .unwrap();
            })
            .expect("error while registering next shortcut");

            let app_handle: tauri::AppHandle = app.clone();
            gsm.register(GlobalShortcutAccelerator::Shuffle.into(), move || {
                let app_handle = app_handle.clone();
                let window = app_handle.get_window("main").unwrap();
                window
                    .eval("document.querySelector('.shuffle').click() ")
                    .unwrap();
            })
            .expect("error while registering shuffle shortcut");

            let app_handle: tauri::AppHandle = app.clone();
            gsm.register(GlobalShortcutAccelerator::Repeat.into(), move || {
                let app_handle = app_handle.clone();
                let window = app_handle.get_window("main").unwrap();
                window
                    .eval("document.querySelector('.repeat').click() ")
                    .unwrap();
            })
            .expect("error while registering repeat shortcut");
        }
        _ => {}
    }
}

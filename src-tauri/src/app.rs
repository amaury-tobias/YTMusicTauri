#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os = "macos")]
use objc_foundation::{INSString, NSString};

use tauri::Manager;

pub fn handle_setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let main_window = app.get_window("main").unwrap();
    #[allow(unused)]
    main_window.with_webview(|webview| {
        #[cfg(target_os = "macos")]
        unsafe {
            let () = msg_send![webview.inner(), setCustomUserAgent: NSString::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.5 Safari/605.1.15")];
        }
    }).unwrap();

    Ok(())
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#![allow(clippy::needless_return)]

use once_cell::sync::OnceCell;
use serde_json::Value;
use service::TimerHandler;
mod service;
mod tray;

static TIMER: OnceCell<TimerHandler> = OnceCell::new();

fn main() {
    TIMER.get_or_init(TimerHandler::new).start_timer();

    let tray = tray::create_tray_menu();
    tauri::Builder::default()
        .setup(setup)
        .system_tray(tray)
        .on_system_tray_event(tray::tray_event_handle)
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                api.prevent_close();
                event.window().hide().unwrap();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            set_shutdown,
            toggle_autorun,
        ])
        // .build(tauri::generate_context!())
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}

#[tauri::command]
fn get_settings() -> Value {
    TIMER.get().unwrap().get_settings()
}

#[tauri::command]
fn set_shutdown(hour: i32, minute: i32) {
    TIMER.get().unwrap().set_shutdown(hour, minute);
}

#[tauri::command]
fn toggle_autorun(enable: bool) {
    if enable {
        service::enable_autorun();
    } else {
        service::cancel_autorun();
    }
}

fn setup(app: &mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = TIMER
        .get()
        .expect("定时器未初始化！！！")
        .app
        .write()
        .expect("无法获得写锁！！！")
        .insert(app.handle());
    Ok(())
}

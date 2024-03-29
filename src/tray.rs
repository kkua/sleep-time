use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

type TrayEventHandler = fn(app: &AppHandle, event: &SystemTrayEvent) -> bool;

const EXIT_MENU_ID: &str = "quit";
const SETTING_MENU_ID: &str = "settings";

const TRAY_EVENT_HANDLERS: &[TrayEventHandler] = &[on_toggle_window, exit, show_window];

pub fn create_tray_menu() -> SystemTray {
    let quit = CustomMenuItem::new(EXIT_MENU_ID.to_string(), "退出");
    let display = CustomMenuItem::new(SETTING_MENU_ID.to_string(), "设置");
    let tray_menu = SystemTrayMenu::new()
        .add_item(display)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

pub fn tray_event_handle(app: &AppHandle, event: SystemTrayEvent) {
    TRAY_EVENT_HANDLERS
        .iter()
        .any(|on_event| on_event(app, &event));
}

fn on_toggle_window(app: &AppHandle, event: &SystemTrayEvent) -> bool {
    if let SystemTrayEvent::LeftClick { .. } = event {
        app.get_window("main").map(|win| {
            if win.is_visible().unwrap_or(false) {
                let _ = win.hide();
            } else {
                let _ = win.show();
                let _ = win.set_focus();
            }
            win
        });
        return true;
    }
    false
}

fn is_tray_item_click(event: &SystemTrayEvent, menu_id: &str) -> bool {
    if let SystemTrayEvent::MenuItemClick { ref id, .. } = event {
        return menu_id.eq(id);
    }
    false
}

fn exit(app: &AppHandle, event: &SystemTrayEvent) -> bool {
    if is_tray_item_click(event, EXIT_MENU_ID) {
        app.exit(0);
        return true;
    }
    false
}

fn show_window(app: &AppHandle, event: &SystemTrayEvent) -> bool {
    if is_tray_item_click(event, SETTING_MENU_ID) {
        app.get_window("main").map(|win| {
            let _ = win.show();
            let _ = win.set_focus();
            win
        });
        return true;
    }
    false
}

use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

type TrayEventHandler = fn(app: &AppHandle, event: SystemTrayEvent) -> (bool, SystemTrayEvent);

const EXIT_MENU_ID: &str = &"quit";
const SETTING_MENU_ID: &str = &"settings";

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
    let mut event = event;
    for on_event in TRAY_EVENT_HANDLERS {
        let (is_ok, e) = on_event(app, event);
        event = e;
        if is_ok {
            break;
        }
    }
}

fn on_toggle_window(app: &AppHandle, event: SystemTrayEvent) -> (bool, SystemTrayEvent) {
    if let SystemTrayEvent::LeftClick { .. } = event {
        app.get_window("main").and_then(|win| {
            if win.is_visible().unwrap_or(false) {
                let _ = win.hide();
            } else {
                let _ = win.show();
                let _ = win.set_focus();
            }
            Some(win)
        });
        return (true, event);
    }
    (false, event)
}

fn is_tray_item_click(event: &SystemTrayEvent, menu_id: &str) -> bool {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        return menu_id.eq(id);
    }
    false
}

fn exit(app: &AppHandle, event: SystemTrayEvent) -> (bool, SystemTrayEvent) {
    if is_tray_item_click(&event, EXIT_MENU_ID) {
        app.exit(0);
        return (true, event);
    }
    (false, event)
}

fn show_window(app: &AppHandle, event: SystemTrayEvent) -> (bool, SystemTrayEvent) {
    if is_tray_item_click(&event, SETTING_MENU_ID) {
        app.get_window("main").and_then(|win| {
            let _ = win.show();
            let _ = win.set_focus();
            Some(win)
        });
        return (true, event);
    }
    (false, event)
}

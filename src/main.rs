// #![cfg_attr(feature = "no-console", windows_subsystem = "windows")]
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

#[macro_use]
extern crate sciter;
#[macro_use]
extern crate anyhow;

mod assets;
mod conf;
mod handler;
mod service;

#[inline]
fn init_sciter() {
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8		// Enables `Sciter.machineName()`
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8 // Enables opening file dialog (`view.selectFile()`)
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SOCKET_IO as u8,
    ))
    .unwrap();
    sciter::set_options(sciter::RuntimeOptions::InitScript(
        conf::SCITER_GLOBAL_SCRIPT,
    ))
    .unwrap();

    #[cfg(debug_assertions)]
    // Enable debug mode for all windows, so that we can inspect them via Inspector.
    sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).unwrap();
}

fn main() {
    init_sciter();
    let index_html = assets::get("index.html").unwrap();
    let mut frame = sciter::Window::new();
    let host_handler = handler::HostHandler::new(&frame);
    let event_handler = handler::EventHandler::new();
    frame.sciter_handler(host_handler);
    frame.event_handler(event_handler);
    frame.load_html(&index_html, Some(conf::UI_SCHEME));
    frame.run_loop();
}

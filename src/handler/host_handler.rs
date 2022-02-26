use crate::assets;
use sciter::host;
use sciter::host::LOAD_RESULT;
use sciter::utf;
use sciter::window::Window;
use std::rc::{Rc, Weak};

pub struct HostHandler {
    pub host: Weak<sciter::Host>,
}

impl HostHandler {
    pub fn new(frame: &Window) -> Self {
        HostHandler {
            host: Rc::downgrade(&frame.get_host()),
        }
    }
}

impl sciter::HostHandler for HostHandler {
    fn on_data_load(&mut self, pnm: &mut host::SCN_LOAD_DATA) -> Option<host::LOAD_RESULT> {
        let uri = utf::w2s(pnm.uri);
        println!("req {}", uri);
        let ui_scheme = crate::conf::UI_SCHEME;
        if uri.starts_with(ui_scheme) {
            if let Some(file) = assets::get(&uri[ui_scheme.len()..]) {
                self.data_ready(pnm.hwnd, &uri, &file.data, None);
                Some(LOAD_RESULT::LOAD_DEFAULT)
            } else {
                None
            }
        } else {
            None
        }
    }
}

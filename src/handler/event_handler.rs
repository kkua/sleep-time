use crate::service;
use chrono::offset::TimeZone;
use chrono::{DateTime, Local};
use sciter::{Element, Value, HELEMENT};
use std::sync::{Arc, Mutex};
pub struct EventHandler {
    root: Option<Element>,
    timer_data: Arc<Mutex<ShutdownTimerData>>,
}

pub const MIN_RUNNING_SECONDS: i64 = 3 * 60;

pub const NOTICE_AHEAD_SECONDS: i64 = 30;

#[derive(Debug)]
struct ShutdownTimerData {
    shutdown_time: i64,
    shutdown_hour: u8,
    shutdown_minute: u8,
    notified: bool,
    recalc: bool,
    shutdown_time_str: String,
    reset_timer: bool,
}

impl ShutdownTimerData {
    fn new(hour: u8, minute: u8) -> Self {
        let mut data = ShutdownTimerData {
            shutdown_time: 0,
            shutdown_hour: hour,
            shutdown_minute: minute,
            notified: false,
            recalc: false,
            shutdown_time_str: String::new(),
            reset_timer: true,
        };
        data.update_timestamp(Local::now());
        data
    }

    fn calc_timestamp(
        now: DateTime<Local>,
        hour: u8,
        minute: u8,
        reset: bool,
    ) -> Option<DateTime<Local>> {
        let today = now.date().naive_local();
        let mut datetime = today.and_hms(hour as u32, minute as u32, 59);
        let remaind_seconds = datetime
            .signed_duration_since(now.naive_local())
            .num_seconds();
        if remaind_seconds < 0 || (reset && remaind_seconds < MIN_RUNNING_SECONDS) {
            datetime = datetime
                .checked_add_signed(chrono::Duration::days(1))
                .unwrap();
        }
        now.timezone().from_local_datetime(&datetime).latest()
    }

    fn update_timestamp(&mut self, now: DateTime<Local>) -> bool {
        if self.reset_timer || self.recalc || now.timestamp() > self.shutdown_time {
            let datetime = ShutdownTimerData::calc_timestamp(
                now,
                self.shutdown_hour,
                self.shutdown_minute,
                self.reset_timer,
            )
            .unwrap();
            let timestamp = datetime.timestamp();
            self.recalc = false;
            self.reset_timer = false;
            if self.shutdown_time != timestamp {
                self.shutdown_time = timestamp;
                self.shutdown_time_str = datetime.format("%m-%d %H:%M:%S").to_string();
                return true;
            }
        }
        false
    }

    fn set_shutdown(&mut self, hour: u8, minute: u8) {
        self.shutdown_hour = hour;
        self.shutdown_minute = minute;
        self.recalc = true;
    }
}

impl EventHandler {
    pub fn new() -> Self {
        return EventHandler {
            root: None,
            timer_data: Arc::new(Mutex::new(ShutdownTimerData::new(0, 0))),
        };
    }

    fn start_timer(&self) {
        use std::thread::{sleep, spawn};
        let timer_data = self.timer_data.clone();
        let root_elem = Arc::new(self.root.as_ref().unwrap().clone());

        spawn(move || loop {
            let now = Local::now();
            {
                let lock_result = (*timer_data).lock();
                if lock_result.is_err() {
                    sleep(std::time::Duration::from_millis(3000));
                    continue;
                }
                let timer_data = &mut lock_result.unwrap();
                let now_timestamp = now.timestamp();
                let shutdown_count_down = timer_data.shutdown_time - now_timestamp;
                if shutdown_count_down < NOTICE_AHEAD_SECONDS && !timer_data.notified {
                    // 通知
                    let _ = root_elem.call_function(
                        "notice",
                        &make_args!(format!("计算机将在{}秒后关闭", NOTICE_AHEAD_SECONDS)),
                    );
                    timer_data.notified = true;
                    continue;
                }
                println!("{} {}", now_timestamp, shutdown_count_down);
                if timer_data.update_timestamp(now) {
                    let _ = root_elem.call_function(
                        "updateShutdownTime",
                        &make_args!(timer_data.shutdown_time_str.clone()),
                    );
                    continue;
                }
                if shutdown_count_down <= 0 {
                    timer_data.reset_timer = true;
                    println!("shutdwon system");
                    service::shutdown_system();
                }
            }
            sleep(std::time::Duration::from_millis(3000));
        });
    }

    fn set_shutdown(&mut self, hour: i32, minute: i32) -> Value {
        self.timer_data
            .lock()
            .expect("failed to get lock of timer_data")
            .set_shutdown(hour as u8, minute as u8);
        Value::null()
    }

    fn autorun_when_boot(&mut self, enable: bool) -> Value {
        if enable {
            let is_ok = service::enable_autorun();
            println!("set autorun: {}", is_ok);
        } else {
            service::cancel_autorun();
        }
        return Value::from(true);
    }

    fn get_settings(&mut self) -> Value {
        let mut val = Value::new();
        val.set_item("autorun", crate::service::is_enable_autorun());
        val.set_item(
            "shutdownTime",
            self.timer_data.lock().unwrap().shutdown_time_str.clone(),
        );
        val
    }
}

impl sciter::EventHandler for EventHandler {
    fn attached(&mut self, root: HELEMENT) {
        println!("attach event handler");
    }

    fn document_complete(&mut self, root: HELEMENT, target: HELEMENT) {
        println!("dom complete callback");
        if dbg!(root.is_null()) {
            panic!("root element is null");
        }
        self.root = Some(Element::from(root));
        self.start_timer();
    }

    dispatch_script_call! {
        fn autorun_when_boot(bool);
        fn get_settings();
        fn set_shutdown(i32, i32);
    }

    fn on_script_call(&mut self, root: HELEMENT, name: &str, argv: &[Value]) -> Option<Value> {
        let args = argv
            .iter()
            .map(|x| format!("{:?}", &x))
            .collect::<Vec<String>>()
            .join(", ");
        println!(
            "script->native: {}({}), root {:?}",
            name,
            args,
            Element::from(root)
        );

        let handled = self.dispatch_script_call(root, name, argv);
        if handled.is_some() {
            return handled;
        }

        // if name == "ScriptCallTest" {
        // 	return self.script_call_test(argv, &Element::from(root));
        // }

        None
    }
}

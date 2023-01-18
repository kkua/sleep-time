use crate::service;
use chrono::offset::TimeZone;
use chrono::{DateTime, Local};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
pub struct TimerHandler {
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
        let today = now.date_naive();
        let opt = today.and_hms_opt(hour as u32, minute as u32, 59);
        if let Some(mut datetime) = opt {
            let remaind_seconds = datetime
                .signed_duration_since(now.naive_local())
                .num_seconds();
            if remaind_seconds < 0 || (reset && remaind_seconds < MIN_RUNNING_SECONDS) {
                datetime = datetime
                    .checked_add_signed(chrono::Duration::days(1))
                    .unwrap();
            }
            now.timezone().from_local_datetime(&datetime).latest()
        } else {
            None
        }
    }

    fn update_timestamp(&mut self, now: DateTime<Local>) -> bool {
        if self.reset_timer || self.recalc || now.timestamp() > self.shutdown_time {
            if let Some(datetime) = ShutdownTimerData::calc_timestamp(
                now,
                self.shutdown_hour,
                self.shutdown_minute,
                self.reset_timer,
            ) {
                let timestamp = datetime.timestamp();
                if self.shutdown_time != timestamp {
                    self.shutdown_time = timestamp;
                    self.shutdown_time_str = datetime.format("%m-%d %H:%M:%S").to_string();
                    return true;
                }
            }
            self.recalc = false;
            self.reset_timer = false;
        }
        false
    }

    fn set_shutdown(&mut self, hour: u8, minute: u8) {
        self.shutdown_hour = hour;
        self.shutdown_minute = minute;
        self.recalc = true;
    }
}

impl TimerHandler {
    pub fn new() -> Self {
        return TimerHandler {
            timer_data: Arc::new(Mutex::new(ShutdownTimerData::new(0, 0))),
        };
    }

    pub fn start_timer(&self) {
        use std::thread::{sleep, spawn};
        let timer_data = self.timer_data.clone();

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
                    timer_data.notified = true;
                    continue;
                }
                println!("{} {}", now_timestamp, shutdown_count_down);
                if timer_data.update_timestamp(now) {
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

    pub fn set_shutdown(&self, hour: i32, minute: i32) {
        let mut data = self
            .timer_data
            .lock()
            .expect("failed to get lock of timer_data");
        data.set_shutdown(hour as u8, minute as u8);
        data.update_timestamp(Local::now());
    }

    pub fn get_settings(&self) -> Value {
        json! ({"autorun": crate::service::is_enable_autorun(),
        "shutdownTime":
            self.timer_data.lock().unwrap().shutdown_time_str.clone()
        })
    }
}

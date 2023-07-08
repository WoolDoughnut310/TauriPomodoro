#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::sync::{mpsc::channel, Arc, Mutex};
use tauri::{Manager, Wry};
use tauri_plugin_store::{Builder, Store, StoreBuilder};
use timer::Timer as Scheduler;

#[derive(Serialize, Clone, Copy)]
enum TimePhase {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Stat {
    minutes: i64,
    sessions: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    today: Stat,
    week: Stat,
    total: Stat,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            today: Stat::default(),
            week: Stat::default(),
            total: Stat::default(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Settings {
    work_time: i32,
    short_break_time: i32,
    long_break_time: i32,
    long_break_interval: i32,
}

struct Timer {
    paused: bool,
    remaining: i32,
    phase: TimePhase,
    last_update: DateTime<Utc>,
    scheduler: Scheduler,
    session_number: i32,
    store: Store<Wry>,
    app_handle: tauri::AppHandle,
}

impl Timer {
    fn get_next_phase(&mut self) -> Result<TimePhase, serde_json::Error> {
        let settings: Settings = get_from_store("settings".into(), &mut self.store)?;
        let long_break_interval = settings.long_break_interval;
        match self.phase {
            TimePhase::Work => {
                if (self.session_number % long_break_interval) == long_break_interval {
                    Ok(TimePhase::LongBreak)
                } else {
                    Ok(TimePhase::ShortBreak)
                }
            }
            _ => Ok(TimePhase::Work),
        }
    }

    fn get_previous_phase(&mut self) -> Result<TimePhase, serde_json::Error> {
        let settings: Settings = get_from_store("settings".into(), &mut self.store)?;
        let long_break_interval = settings.long_break_interval;
        match self.phase {
            TimePhase::Work => {
                if (self.session_number % long_break_interval) == 0 {
                    Ok(TimePhase::LongBreak)
                } else {
                    Ok(TimePhase::ShortBreak)
                }
            }
            _ => Ok(TimePhase::Work),
        }
    }

    fn reset_scheduler(&mut self) -> Result<(), serde_json::Error> {
        let settings: Settings = get_from_store("settings".into(), &mut self.store)?;
        self.remaining = match self.phase {
            TimePhase::Work => settings.work_time,
            TimePhase::ShortBreak => settings.short_break_time,
            TimePhase::LongBreak => settings.long_break_time,
        };
        self.app_handle.emit_all("switch-phase", self.phase);
        self.app_handle.emit_all("remaining", self.remaining);

        let (tx, rx) = channel();

        // if self.scheduler == None {
        //     self.scheduler = Scheduler::new()
        // }

        self.scheduler
            .schedule_with_delay(Duration::minutes(self.remaining.into()), move || {
                tx.send(()).unwrap();
            });

        self.pause(false);

        // Once the delay has been reached
        rx.recv().unwrap();
        let new_phase = self.get_next_phase()?;
        self.switch_phase(new_phase, false, false);
        Ok(())
    }

    fn get_elapsed(&self) -> i64 {
        (self.last_update - Utc::now()).num_minutes()
    }

    fn switch_phase(&mut self, new_phase: TimePhase, is_previous: bool, is_user: bool) {
        let elapsed = self.get_elapsed();
        self.last_update = Utc::now();

        self.phase = new_phase;
        self.reset_scheduler();

        self.session_number += if !is_previous { 1 } else { -1 };
        self.update_stats(elapsed, !(is_user || is_previous));
    }

    fn update_stats(
        &mut self,
        elapsed_time: i64,
        session_completed: bool,
    ) -> Result<(), serde_json::Error> {
        let mut stats: serde_json::Value = get_from_store("stats".into(), &mut self.store)?;

        let session_increment = if session_completed { 1 } else { 0 };

        for key in ["today", "week", "total"].iter() {
            let mut minutes: i64 = serde_json::from_value(stats[key]["minutes"].clone())?;
            minutes += elapsed_time;
            stats[key]["minutes"] = json!(minutes);

            let mut sessions: i32 = serde_json::from_value(stats[key]["sessions"].clone())?;
            sessions += session_increment;
            stats[key]["sessions"] = json!(sessions);
        }
        self.store.insert("stats".into(), json!(stats));
        Ok(())
    }

    fn pause(&mut self, paused: bool) {
        if paused {
            self.scheduler = Scheduler::new();
        } else if self.paused {
            // Only resume if `paused` is false and the timer is already paused
            self.reset_scheduler();
        }
        self.paused = paused;
    }
}

#[tauri::command]
fn switch_phase(is_previous: bool, timer_state: tauri::State<Arc<Mutex<Timer>>>) {
    let mut timer = timer_state.lock().unwrap();

    let new_phase = if !is_previous {
        timer.get_next_phase().unwrap()
    } else {
        timer.get_previous_phase().unwrap()
    };

    timer.switch_phase(new_phase, is_previous, true);
}

#[tauri::command]
fn pause(paused: bool, timer_state: tauri::State<Arc<Mutex<Timer>>>) {
    let mut timer = timer_state.lock().unwrap();
    timer.pause(paused);
}

fn get_from_store<'a, T: DeserializeOwned>(
    key: String,
    store: &Store<Wry>,
) -> Result<T, serde_json::Error> {
    let output: T;
    match (*store).get(key.clone()) {
        Some(value) => output = serde_json::from_value((*value).clone())?,
        None => panic!("{} doesn't exist in the store!", key.as_str()),
    };
    Ok(output)
}

// Check if the stats for yesterday or last week need resetting
fn check_stat_reset(store: &mut Store<Wry>) -> Result<bool, serde_json::Error> {
    let last_opened: DateTime<Utc> = get_from_store("last_opened".into(), store)?;
    let today = Utc::now();
    let mut stats: Stats = get_from_store("stats".into(), store)?;

    if today.year() != last_opened.year() || today.ordinal() != last_opened.ordinal() {
        // Reset "today" on stats
        stats.today = Stat::default();
        store.insert("stats".into(), json!(stats));
        return Ok(true);
    }
    if today.year() != last_opened.year()
        || today.iso_week().week() != last_opened.iso_week().week()
    {
        // Reset "week" on stats
        stats.week = Stat::default();
        store.insert("stats".into(), json!(stats));
        return Ok(true);
    }
    return Ok(false);
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let store = StoreBuilder::new(app.handle(), ".store.dat".into())
                .default("settings".into(), json!(Settings::default()))
                .default("stats".into(), json!(Stats::default()))
                .default("last_opened".into(), json!(Utc::now()))
                .build();
            app.handle().plugin(Builder::default().store(store).build());
            Ok(())
        })
        .setup(|app| {
            let mut store = StoreBuilder::new(app.handle(), ".store.dat".into())
                .default("settings".into(), json!(Settings::default()))
                .default("stats".into(), json!(Stats::default()))
                .default("last_opened".into(), json!(Utc::now()))
                .build();
            store.load();
            check_stat_reset(&mut store);

            app.manage(Arc::new(Mutex::new(Timer {
                paused: true,
                remaining: 0,
                phase: TimePhase::Work,
                last_update: Utc::now(),
                scheduler: Scheduler::new(),
                session_number: 0,
                app_handle: app.handle().clone(),
                store,
            })));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![pause])
        .invoke_handler(tauri::generate_handler![switch_phase])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

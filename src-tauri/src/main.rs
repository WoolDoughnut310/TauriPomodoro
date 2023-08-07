#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::{DateTime, Datelike, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{from_value, json};
use std::{path::PathBuf, sync::Mutex};
use tauri::{api::notification::Notification, AppHandle, Manager, Wry};
use tauri_plugin_store::{Builder, Store, StoreBuilder, StoreCollection};

const STORE_PATH: &str = ".store.dat";

#[derive(PartialEq, Serialize, Clone, Copy, Debug)]
enum TimePhase {
    Work,
    ShortBreak,
    LongBreak,
}

impl Default for TimePhase {
    fn default() -> Self {
        Self::Work
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Stat {
    minutes: i32,
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

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    work_time: i32,
    short_break_time: i32,
    long_break_time: i32,
    long_break_interval: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            work_time: 25,
            short_break_time: 5,
            long_break_time: 20,
            long_break_interval: 4,
        }
    }
}

struct Phase(Mutex<TimePhase>);
struct SessionNumber(Mutex<i32>);

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Store(#[from] tauri_plugin_store::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

fn with_store<F>(app: &AppHandle, f: F)
where
    F: FnOnce(&mut Store<Wry>) -> Result<(), tauri_plugin_store::Error>,
{
    let stores = app.state::<StoreCollection<Wry>>();
    tauri_plugin_store::with_store(app.clone(), stores, PathBuf::from(STORE_PATH), f);
}

fn get_from_store<'a, T: DeserializeOwned>(store: &mut Store<Wry>, key: &str) -> Result<T, Error> {
    Ok(from_value(
        (*store.get(key.clone()).expect("Field doesn't exist!")).clone(),
    )?)
}

fn set_phase(app: &AppHandle, new_phase: TimePhase) {
    let phase = app.state::<Phase>();
    *phase.0.lock().unwrap() = new_phase;
    app.emit_all("switch-phase", phase.0.lock().unwrap().clone());
}

fn update_session_number(app: &AppHandle, previous_value: i32, is_previous: bool) -> i32 {
    let session_number = app.state::<SessionNumber>();

    let new_value = if !is_previous {
        previous_value + 1
    } else {
        previous_value - 1
    };

    *session_number.0.lock().unwrap() = new_value;

    app.emit_all("session-number", new_value);
    new_value
}

fn get_remaining(app: &AppHandle, store: &mut Store<Wry>) -> Result<i32, Error> {
    let settings: Settings = get_from_store(store, "settings")?;

    let phase = app.state::<Phase>();
    let value = match phase.0.lock().unwrap().clone() {
        TimePhase::Work => settings.work_time,
        TimePhase::ShortBreak => settings.short_break_time,
        TimePhase::LongBreak => settings.long_break_time,
    };

    Ok(value)
}

fn get_new_phase(
    app: &AppHandle,
    store: &mut Store<Wry>,
    session_number: i32,
) -> Result<TimePhase, Error> {
    let settings: Settings = get_from_store(store, "settings")?;

    let phase = app.state::<Phase>();
    let long_break_interval = settings.long_break_interval;

    let new_phase = if session_number % 2 == 1 {
        if (session_number % (long_break_interval * 2 - 1)) == 0 {
            Ok(TimePhase::LongBreak)
        } else {
            Ok(TimePhase::ShortBreak)
        }
    } else {
        Ok(TimePhase::Work)
    };
    new_phase
}

fn update_stats(app: &AppHandle, store: &mut Store<Wry>) -> Result<(), Error> {
    let elapsed_time = get_remaining(&app, store)?;
    let mut stats: serde_json::Value = get_from_store(store, "stats")?;

    for key in ["today", "week", "total"].iter() {
        let minutes: i32 = from_value(stats[key]["minutes"].clone())?;
        stats[key]["minutes"] = json!(minutes + elapsed_time);

        let sessions: i32 = from_value(stats[key]["sessions"].clone())?;
        stats[key]["sessions"] = json!(sessions + 1);
    }
    store.insert("stats".into(), json!(stats));
    Ok(())
}

fn emit_status_notification(app: &AppHandle) {
    let phase = app.state::<Phase>();
    let body = match phase.0.lock().unwrap().clone() {
        TimePhase::Work => "Time to get back to work!",
        TimePhase::ShortBreak => "Have a little rest!",
        TimePhase::LongBreak => "Take some extra time to relax!",
    };

    Notification::new(app.config().tauri.bundle.identifier.clone())
        .title("Phase changed")
        .body(body)
        .show()
        .unwrap();
}

#[tauri::command]
fn reset_phase(app: AppHandle) {
    with_store(&app, |store| {
        let remaining = get_remaining(&app, store).unwrap();
        app.emit_all("remaining", remaining);
        Ok(())
    });
}

#[tauri::command]
fn switch_phase(
    is_previous: bool,
    is_user: bool,
    app: AppHandle,
    session_number_state: tauri::State<SessionNumber>,
    phase_state: tauri::State<Phase>,
) {
    let session_number = *session_number_state.0.lock().unwrap();
    let phase = phase_state.0.lock().unwrap().clone();

    with_store(&app, |store| {
        if TimePhase::Work == phase && !(is_user || is_previous) {
            update_stats(&app, store);
        }

        let session_number = update_session_number(&app, session_number, is_previous);

        let new_phase = get_new_phase(&app, store, session_number).unwrap();
        set_phase(&app, new_phase);

        emit_status_notification(&app);

        let remaining = get_remaining(&app, store).unwrap();
        app.emit_all("remaining", remaining);
        Ok(())
    });
}

#[tauri::command]
fn update_settings(settings: Settings, app: AppHandle) {
    with_store(&app, |store| {
        store.insert("settings".into(), json!(settings));
        Ok(())
    });
}

#[tauri::command]
fn restore_state(
    app: AppHandle,
    phase: tauri::State<Phase>,
    session_number: tauri::State<SessionNumber>,
) {
    app.emit_all("switch-phase", phase.0.lock().unwrap().clone());
    app.emit_all("session-number", *session_number.0.lock().unwrap());
    with_store(&app, |store| {
        let remaining = get_remaining(&app, store).unwrap();
        app.emit_all("remaining", remaining);
        Ok(())
    });
}

// Check if the stats for yesterday or last week need resetting
fn check_stat_reset(store: &mut Store<Wry>) -> Result<bool, Error> {
    let last_opened: DateTime<Utc> =
        from_value((*(store.get("last_opened").expect("Field doesn't exist"))).clone())?;
    let mut stats: Stats =
        from_value((*(store.get("stats").expect("Field doesn't exist"))).clone())?;

    let today = Utc::now();

    // If last opened is on a different year,
    // or on a different day of the year
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
        .setup(|app| {
            let store = StoreBuilder::new(app.handle(), STORE_PATH.into())
                .default("settings".into(), json!(Settings::default()))
                .default("stats".into(), json!(Stats::default()))
                .default("last_opened".into(), json!(Utc::now()))
                .build();
            app.handle().plugin(Builder::default().store(store).build());
            let mut store = StoreBuilder::new(app.handle(), STORE_PATH.into())
                .default("settings".into(), json!(Settings::default()))
                .default("stats".into(), json!(Stats::default()))
                .default("last_opened".into(), json!(Utc::now()))
                .build();
            check_stat_reset(&mut store);
            Ok(())
        })
        .manage(Phase(Mutex::new(TimePhase::default())))
        .manage(SessionNumber(Mutex::new(0)))
        .invoke_handler(tauri::generate_handler![
            switch_phase,
            reset_phase,
            update_settings,
            restore_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

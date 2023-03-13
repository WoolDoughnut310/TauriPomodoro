#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

enum TimePhase {
    Work,
    ShortBreak,
    LongBreak,
}

struct Timer {
    paused: bool,
    remaining: i32,
    phase: TimePhase,
}

fn main() {
  tauri::Builder::default()
      .manage(Timer {
          paused: true,
          remaining: 0,
          phase: TimePhase::Work
      })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

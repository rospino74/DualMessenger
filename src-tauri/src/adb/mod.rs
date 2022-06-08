use tauri::{generate_handler, Invoke, Wry};

mod commands;
pub mod types;

pub fn enumerate_adb_handlers() -> Box<dyn Fn(Invoke<Wry>) + Send + Sync + 'static> {
    Box::new(generate_handler![
        commands::is_adb_installed,
        commands::get_adb_devices,
        commands::get_adb_users,
    ])
}
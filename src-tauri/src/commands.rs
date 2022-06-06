use super::adb::*;
use std::process::Command;
use sys_locale::get_locale as get_system_locale;
use tauri::{command, generate_handler, Invoke, Result as TResult, Wry};
use which::which;

#[cfg(target_os = "windows")]
use {winreg::enums::HKEY_LOCAL_MACHINE, winreg::RegKey};

#[cfg(not(target_os = "windows"))]
#[command]
fn get_sys_version() -> String {
    // Exexute the command `uname -a` and return the output
    let output = Command::new("uname")
        .arg("-a")
        .output()
        .expect("failed to execute process");

    return String::from_utf8(output.stdout).expect("failed to convert output to string");
}

#[cfg(target_os = "windows")]
#[command]
fn get_sys_version() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let sub = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
        .expect("failed to open the registry subkey");
    let product_name: String = sub
        .get_value("ProductName")
        .expect("failed to get the value of the ProductName key");
    let product_version: String = sub
        .get_value("DisplayVersion")
        .expect("failed to get the value of the DisplayVersion key");

    return format!("{} {}", product_name, product_version);
}

#[command]
fn get_locale() -> String {
    get_system_locale().unwrap_or_else(|| String::from("en-US"))
}

#[command]
fn is_adb_installed() -> bool {
    let result = which("adb");
    return result.is_ok();
}

#[command]
fn get_adb_devices() -> TResult<Vec<String>> {
// fn get_adb_devices() -> TResult<Vec<Device>> {
    let output = Command::new("adb").arg("devices").output()?;

    // Drop all the lines until the device list
    let result = String::from_utf8(output.stdout).unwrap();
    let mut lines = result
        .lines()
        .skip_while(|line| !line.starts_with("List of devices attached"));
    // Drop the first line
    lines.next();

    // Now we can iterate over the lines and parse the device
    // let devices: Vec<_> = lines
    //     .filter_map(|line| {
    //         // Split the line by spaces
    //         let mut parts = line.split_whitespace();
    //         // Get the device identifier
    //         let device_id_str = parts.next().unwrap();
    //         let device_id = u64::from_str_radix(device_id_str, 16).unwrap();

    //         // Get the device type
    //         let device_type_str = parts.next().unwrap();
    //         let device_type = if device_type_str == "device" {
    //             DeviceType::Device
    //         } else {
    //             DeviceType::Emulator
    //         };

    //         Some(Device::new(
    //             device_id,
    //             device_type,
    //             device_type_str != "unauthorized",
    //         ))
    //     })
    //     .collect();

    let devices = lines.map(|s| String::from(s)).collect::<Vec<String>>();
    Ok(devices)
}

pub fn enumerate_native_handlers() -> Box<dyn Fn(Invoke<Wry>) + Send + Sync + 'static> {
    Box::new(generate_handler![
        get_sys_version,
        get_locale,
        is_adb_installed,
        get_adb_devices
    ])
}

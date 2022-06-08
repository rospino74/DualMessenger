use tauri::{command, api::process::Command};
use text_io::scan;
use which::which;
use super::types::*;

#[command]
pub async fn is_adb_installed() -> bool {
    let result = which("adb");
    return result.is_ok();
}

#[command]
pub async fn get_adb_users(device: Device) -> Vec<User> {
    let id = if *device.online() {
        device.ip().unwrap()
    } else {
        device.id().to_string()
    };

    let output = Command::new("adb")
        .args(["-s", &id]) // Specify the device serial id
        .args(["shell", "pm", "list", "users"])
        .output()
        .expect("failed to get the user list");

    // Drop all the lines until the user list
    let mut lines = output.stdout.lines().skip_while(|line| !line.starts_with("Users"));
    lines.next();

    let users: Vec<_> = lines
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            let (user_id, user_name, number, status): (u32, String, u64, String);
            let mut it = line.bytes().map(|b| {
                if b == b'}' {
                    b']'
                } else if b == b'{' {
                    b'['
                } else {
                    b
                }
            }); // BUG in scan!() macro
            scan!(it => "\tUserInfo[{}:{}:{}] {}", user_id, user_name, number, status);

            Some(User::new(user_id, user_name))
        })
        .collect();

    users
}

#[command]
pub async fn get_adb_devices() -> Vec<Device> {
    let output = Command::new("adb")
        .args(["devices"])
        .output()
        .expect("failed to get the adb devices");

    // Drop all the lines until the device list
    let mut lines = output.stdout
        .lines()
        .skip_while(|line| !line.starts_with("List of devices attached"));
    // Drop the first line
    lines.next();

    // Now we can iterate over the lines and parse the device
    let devices: Vec<_> = lines
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let (device_id_str, device_type_str): (String, String);
            scan!(line.bytes() => "{}\t{}", device_id_str, device_type_str);
            
            let is_online = device_id_str.contains(".");
            let authorized = device_type_str != "unauthorized";
            let device_type = if device_type_str == "device" {
                DeviceType::Device
            } else {
                DeviceType::Emulator
            };

            if is_online {
                Some(Device::new_online(
                    device_id_str,
                    device_type,
                    authorized,
                ))
            } else {
                Some(Device::new(
                    u64::from_str_radix(&device_id_str, 16).unwrap(),
                    device_type,
                    authorized,
                ))
            }
        })
        .collect();

    devices
}

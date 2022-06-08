use super::types::*;
use tauri::{api::process::Command, command};
use text_io::scan;
use which::which;

macro_rules! run_adb_command {
    ($param:expr, $args:expr) => {
        Command::new("adb")
            .args($param)
            .args($args)
            .output()
            .expect("error while running command")
            .stdout
    };
    ($args:expr) => {
        run_adb_command!([""], $args)
    };
}

macro_rules! skip_until {
    ($iterator:expr, $search:expr) => {{
        let mut lines = $iterator.skip_while(|line| !line.starts_with($search));
        lines.next();
        lines
    }};
}

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

    let output = run_adb_command!(
        ["-s", &id], // Specify the device to use (-s <device_id>)
        ["shell", "pm", "list", "users"]
    );

    // Drop all the lines until the user list
    let users: Vec<_> = skip_until!(output.lines(), "User")
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
    let output = run_adb_command!(["devices"]);

    // Drop all the lines until the device list
    // Now we can iterate over the lines and parse the device
    let devices: Vec<_> = skip_until!(output.lines(), "List of devices attached")
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
                Some(Device::new_online(device_id_str, device_type, authorized))
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


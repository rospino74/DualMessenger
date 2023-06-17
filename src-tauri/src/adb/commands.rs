use super::{types::*, utils::*};
use tauri::command;
use text_io::scan;
use which::which;

#[command]
pub async fn is_adb_installed() -> bool {
    let result = which("adb");
    return result.is_ok();
}

#[command]
pub async fn get_adb_users(device: Device) -> Vec<User> {
    let output = run_adb_command!(
        ["-s", &device.id()], // Specify the device to use (-s <device_id>)
        ["shell", "pm", "list", "users"]
    );

    println!("ADB -s {} shell pm list users: {}", &device.id(), output);

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
    
    println!("ADB devices: {}", output);

    // Drop all the lines until the device list
    // Now we can iterate over the lines and parse the device
    let devices: Vec<_> = {
        let mut lines = output
            .lines()
            .skip_while(|line| !line.starts_with("List of devices attached"));
        lines.next();
        lines
    }
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

        Some(Device::new(
            device_id_str,
            device_type,
            authorized,
            is_online,
        ))
    })
    .collect();

    devices
}

#[command]
pub async fn get_adb_packages(device: Device, user: User) -> Vec<Package> {
    let device_id = device.id();
    let user_id = user.id().to_string();

    let output = run_adb_command!(
        ["-s", &device_id], // Specify the device to use (-s <device_id>)
        ["shell", "pm", "list", "packages", "-3", "--user", &user_id]
    );

    println!("ADB -s {} shell pm list packages -3 --user {}: {}", &device_id, &user_id, output);

    output
        .lines()
        .filter_map(|line| {
            let (package_name, package_version): (String, String);

            // Check if the line is empty, if so, go to the next line
            if line.is_empty() {
                return None;
            }

            // Get the package name
            scan!(line.bytes() => "package:{}", package_name);

            // Now get the package version
            let dumpsys_output = run_adb_command!(
                ["-s", &device_id], // Specify the device to use (-s <device_id>)
                [
                    "shell",
                    "dumpsys",
                    "package",
                    &package_name,
                    "|",
                    "grep",
                    "versionName"
                ]
            );
            scan!(dumpsys_output.bytes() => "versionName={}", package_version);

            Some(Package::new(
                package_name.clone(),
                package_name,
                package_version,
            ))
        })
        .collect()
}

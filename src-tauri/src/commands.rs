use sys_locale::get_locale as get_system_locale;
use tauri::{command, generate_handler, Invoke, Wry, api::process::Command};

#[cfg(target_os = "windows")]
use {winreg::enums::HKEY_LOCAL_MACHINE, winreg::RegKey};

#[cfg(not(target_os = "windows"))]
#[command]
fn get_sys_version() -> String {
    // Exexute the command `uname -a` and return the output
    let output = Command::new("uname")
        .args(["-a"])
        .output()
        .expect("failed to execute process");

    return output.stdout;
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

pub fn enumerate_native_handlers() -> Box<dyn Fn(Invoke<Wry>) + Send + Sync + 'static> {
    Box::new(generate_handler![
        get_sys_version,
        get_locale,
    ])
}

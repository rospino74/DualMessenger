use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DeviceType {
    Emulator,
    Device,
}

#[derive(Serialize, Deserialize, Getters)]
pub struct Device {
    id: String,
    device_type: DeviceType,
    authorized: bool,
    online: bool,
}

impl Device {
    pub fn new(id: String, device_type: DeviceType, authorized: bool, online: bool) -> Self {
        Self {
            id,
            device_type,
            authorized,
            online,
        }
    }
}

#[derive(Serialize, Deserialize, Getters)]
pub struct User {
    id: u32,
    name: String,
}

impl User {
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }
}


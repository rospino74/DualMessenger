use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum DeviceType {
    Emulator,
    Device,
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    id: u64,
    device_type: DeviceType,
    authorized: bool,
}

impl Device {
    pub fn new(id: u64, device_type: DeviceType, authorized: bool) -> Self {
        Self {
            id,
            device_type,
            authorized,
        }
    }
}

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use text_io::scan;

#[derive(Serialize, Deserialize)]
pub enum DeviceType {
    Emulator,
    Device,
}

#[derive(Serialize, Deserialize, Getters)]
pub struct Device {
    id: u64,
    device_type: DeviceType,
    authorized: bool,
    online: bool,
}

impl Device {
    pub fn new(id: u64, device_type: DeviceType, authorized: bool) -> Self {
        Self {
            id,
            device_type,
            authorized,
            online: false,
        }
    }

    pub fn new_online(ip: String, device_type: DeviceType, authorized: bool) -> Self {
        Self {
            id: Device::convert_ip_to_id(ip),
            device_type,
            authorized,
            online: true,
        }
    }

    pub fn ip(&self) -> Option<String> {
        if self.online {
            Some(Device::convert_id_to_ip(self.id))
        } else {
            None
        }
    }

    fn convert_ip_to_id(ip: String) -> u64 {
        let (a, b, c, d, port): (u64, u64, u64, u64, u64);
        scan!(ip.bytes() => "{}.{}.{}.{}:{}", a, b, c, d, port);

        // First 4 bytes are the ip, the last 2 bytes are the port
        ((a << 24 | b << 16 | c << 8 | d) << 32) + port
    }

    fn convert_id_to_ip(id: u64) -> String {
        let ip_id = (id >> 32) as u32;
        let port = (id & 0xFFFF) as u16;

        format!(
            "{}.{}.{}.{}:{}",
            ip_id >> 24,
            (ip_id >> 16) & 0xFF,
            (ip_id >> 8) & 0xFF,
            ip_id & 0xFF,
            port
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u32,
    name: String,
}

impl User {
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }
}

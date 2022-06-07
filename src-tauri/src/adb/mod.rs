use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

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
        let mut address_parts = ip.split(":");
        let ip = address_parts.next().unwrap().parse::<Ipv4Addr>().unwrap();
        let port = address_parts.next().unwrap().parse::<u16>().unwrap();

        let ip_id: u32 = ip.into();

        // First 4 bytes are the ip, the last 2 bytes are the port
        ((ip_id as u64) << 32) + port as u64
    }

    fn convert_id_to_ip(id: u64) -> String {
        let ip_id = (id >> 32) as u32;
        let port = (id & 0xFFFF) as u16;

        format!("{}:{}", Ipv4Addr::from(ip_id), port)
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

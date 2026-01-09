use crate::models::v1::Device;
use crate::{VERSION, models};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type TempId = String;
pub type ChannelId = String;
pub type DeviceId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomDevices {
    #[serde(default)]
    pub devices: Vec<CustomDevice>,
}

impl CustomDevices {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn get_device(&self, device_id: &str) -> Option<&CustomDevice> {
        self.devices.iter().find(|d| d.id == device_id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomDevice {
    pub id: DeviceId,
    pub label: String,
    pub channels: Vec<DeviceChannel>,
    pub temps: Vec<DeviceTemp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceChannel {
    pub id: ChannelId,
    pub label: String,
    pub commands: HashMap<ChannelCommand, String>,
}

impl DeviceChannel {
    pub fn get_command(&self, cmd: &ChannelCommand) -> Option<&str> {
        self.commands.get(cmd).map(|s| s.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ChannelCommand {
    GetRpm,
    GetDuty,
    SetDuty,
    EnableManual,
    Reset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTemp {
    pub id: TempId,
    pub label: String,
    pub command: String,
}

pub fn convert_to_devices(config: &CustomDevices) -> Result<Vec<Device>> {
    let mut devices = Vec::new();

    for custom_device in &config.devices {
        let channels = custom_device
            .channels
            .iter()
            .cloned()
            .map(|channel| {
                let is_controllable = channel.get_command(&ChannelCommand::SetDuty).is_some();
                (
                    channel.id,
                    models::v1::ChannelInfo {
                        label: Some(channel.label),
                        options: Some(models::v1::channel_info::Options::SpeedOptions(
                            models::v1::SpeedOptions {
                                min_duty: 0,
                                max_duty: 100,
                                fixed_enabled: is_controllable,
                                extension: None,
                            },
                        )),
                    },
                )
            })
            .collect();

        let mut temps = HashMap::new();
        for (i, temp) in custom_device.temps.iter().cloned().enumerate() {
            temps.insert(
                temp.id,
                models::v1::TempInfo {
                    label: temp.label,
                    number: (i + 1) as u32,
                },
            );
        }

        let driver_info = models::v1::DriverInfo {
            name: Some("CustomDevice".to_string()),
            version: Some(VERSION.to_string()),
            locations: vec![],
        };

        devices.push(Device {
            id: custom_device.id.clone(),
            name: custom_device.label.clone(),
            uid_info: None,
            info: Some(models::v1::DeviceInfo {
                channels,
                temps,
                lighting_speeds: vec![],
                temp_min: Some(0.),
                temp_max: Some(100.),
                profile_min_length: None,
                profile_max_length: None,
                model: None,
                driver_info: Some(driver_info),
            }),
        });
    }
    Ok(devices)
}

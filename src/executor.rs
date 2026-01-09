use crate::config::{ChannelCommand, CustomDevice};
use crate::models;
use log::warn;
use tokio::process::Command;

pub async fn poll_device_status(device: &CustomDevice) -> Vec<models::v1::Status> {
    let mut statuses = Vec::new();

    for channel in &device.channels {
        let rpm = match channel.get_command(&ChannelCommand::GetRpm) {
            Some(cmd) => run_shell_command(cmd)
                .await
                .and_then(|out| out.trim().parse::<u32>().ok()),
            None => None,
        };
        let duty = match channel.get_command(&ChannelCommand::GetDuty) {
            Some(cmd) => run_shell_command(cmd)
                .await
                .and_then(|out| out.trim().parse::<f64>().ok()),
            None => None,
        };
        if rpm.is_some() || duty.is_some() {
            statuses.push(models::v1::Status {
                id: channel.id.to_string(),
                metric: Some(models::v1::status::Metric::Speed(
                    models::v1::status::FanSpeed { duty, rpm },
                )),
            });
        }
    }
    for temp in &device.temps {
        if let Some(value) = run_shell_command(&temp.command)
            .await
            .and_then(|out| out.trim().parse::<f64>().ok())
        {
            // Temps should be in millidegrees, but have a backup:
            let temp_c = if value > 1000.0 {
                value / 1000.0
            } else {
                value
            };
            statuses.push(models::v1::Status {
                id: temp.id.to_string(),
                metric: Some(models::v1::status::Metric::Temp(temp_c)),
            });
        }
    }

    statuses
}

pub async fn run_shell_command(cmd: &str) -> Option<String> {
    match Command::new("sh").arg("-c").arg(cmd).output().await {
        Ok(output) if output.status.success() => String::from_utf8(output.stdout).ok(),
        Ok(output) => {
            warn!("Command failed: {cmd} - {:?}", output.status);
            None
        }
        Err(e) => {
            warn!("Failed to execute command: {cmd} - {e}");
            None
        }
    }
}

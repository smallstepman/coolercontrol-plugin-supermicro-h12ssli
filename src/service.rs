use crate::config::CustomDevices;
use crate::device_service::v1::device_service_server::DeviceService;
use crate::device_service::v1::{
    health_response, CustomFunctionOneRequest, CustomFunctionOneResponse,
    EnableManualFanControlRequest, EnableManualFanControlResponse, FixedDutyRequest, FixedDutyResponse,
    HealthRequest, HealthResponse, InitializeDeviceRequest, InitializeDeviceResponse, LcdRequest,
    LcdResponse, LightingRequest, LightingResponse, ListDevicesRequest,
    ListDevicesResponse, ResetChannelRequest, ResetChannelResponse, ShutdownRequest,
    ShutdownResponse, SpeedProfileRequest, SpeedProfileResponse, StatusRequest, StatusResponse,
};
use crate::models::v1::Device;
use crate::{config, executor, SERVICE_ID, VERSION};
use anyhow::Result;
use log::debug;
use tonic::{Request, Response, Status};

pub struct CustomDeviceService {
    config: CustomDevices,
    devices: Vec<Device>,
}

impl CustomDeviceService {
    pub fn load_from_config_json() -> Result<Self> {
        let config = std::fs::read_to_string("config.json")?;
        let config: CustomDevices = serde_json::from_str(&config)?;
        let devices = config::convert_to_devices(&config)?;
        Ok(Self { config, devices })
    }
}

#[tonic::async_trait]
impl DeviceService for CustomDeviceService {
    /// Used to confirm service connection and retrieve service health information.
    async fn health(
        &self,
        _request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let reply = HealthResponse {
            name: SERVICE_ID.to_string(),
            version: VERSION.to_string(),
            status: health_response::Status::Ok.into(),
            // information purposes only
            uptime_seconds: 1,
        };
        Ok(Response::new(reply))
    }

    /// This is the first message sent to the device service after establishing a connection
    /// and is used to detect the service's devices and capabilities.
    /// The device models should be filled out for each device and all of their
    /// available channels. This information is used to populate the CoolerControl device
    /// list and available features in the UI.
    async fn list_devices(
        &self,
        _request: Request<ListDevicesRequest>,
    ) -> Result<Response<ListDevicesResponse>, Status> {
        Ok(Response::new(ListDevicesResponse {
            devices: self.devices.clone(),
        }))
    }

    /// This is called and used by some devices to initialize hardware, before starting to send
    /// commands to it. It is also be called after resuming from sleep, as many firmwares are rest.
    async fn initialize_device(
        &self,
        _request: Request<InitializeDeviceRequest>,
    ) -> Result<Response<InitializeDeviceResponse>, Status> {
        // No init logic needed.
        Ok(Response::new(InitializeDeviceResponse {}))
    }

    async fn shutdown(
        &self,
        _request: Request<ShutdownRequest>,
    ) -> Result<Response<ShutdownResponse>, Status> {
        // No shutdown logic needed
        Ok(Response::new(ShutdownResponse {}))
    }

    /// This is called to retrieve the status per device and their respective channels
    /// and is called at a regular intervals (default 1 second).
    ///
    /// Device _channels_ usually can not be done concurrently, but that depends on the hardware and drivers.
    async fn status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let device_id = &request.get_ref().device_id;
        let Some(custom_device) = self.config.get_device(device_id) else {
            return Err(Status::not_found("Device not found"));
        };
        let status = executor::poll_device_status(custom_device).await;
        Ok(Response::new(StatusResponse { status }))
    }

    /// Reset the device channel to it's default state if applicable. (Auto)
    async fn reset_channel(
        &self,
        request: Request<ResetChannelRequest>,
    ) -> Result<Response<ResetChannelResponse>, Status> {
        let req = request.get_ref();
        let Some(device) = self.config.get_device(&req.device_id) else {
            return Err(Status::not_found("Device not found"));
        };
        let Some(channel) = device.channels.iter().find(|c| c.id == req.channel_id) else {
            return Err(Status::not_found("Channel not found"));
        };
        let Some(cmd) = channel.get_command(&config::ChannelCommand::Reset) else {
            return Ok(Response::new(ResetChannelResponse {}));
        };
        let output = executor::run_shell_command(cmd).await;
        debug!("Reset output: {output:?}");
        Ok(Response::new(ResetChannelResponse {}))
    }

    async fn enable_manual_fan_control(
        &self,
        request: Request<EnableManualFanControlRequest>,
    ) -> Result<Response<EnableManualFanControlResponse>, Status> {
        let req = request.get_ref();
        let Some(device) = self.config.get_device(&req.device_id) else {
            return Err(Status::not_found("Device not found"));
        };
        let Some(channel) = device.channels.iter().find(|c| c.id == req.channel_id) else {
            return Err(Status::not_found("Channel not found"));
        };
        let Some(cmd) = channel.get_command(&config::ChannelCommand::EnableManual) else {
            return Ok(Response::new(EnableManualFanControlResponse {}));
        };
        let output = executor::run_shell_command(cmd).await;
        debug!("EnableManual output: {output:?}");
        Ok(Response::new(EnableManualFanControlResponse {}))
    }

    async fn fixed_duty(
        &self,
        request: Request<FixedDutyRequest>,
    ) -> Result<Response<FixedDutyResponse>, Status> {
        let req = request.get_ref();
        let Some(device) = self.config.get_device(&req.device_id) else {
            return Err(Status::not_found("Device not found"));
        };
        let Some(channel) = device.channels.iter().find(|c| c.id == req.channel_id) else {
            return Err(Status::not_found("Channel not found"));
        };
        let Some(cmd_template) = channel.get_command(&config::ChannelCommand::SetDuty) else {
            return Err(Status::unimplemented(
                "SetDuty not configured for this channel",
            ));
        };
        let cmd = cmd_template.replace("{duty}", &req.duty.to_string());
        let output = executor::run_shell_command(&cmd).await;
        debug!("SetDuty output: {output:?}");
        Ok(Response::new(FixedDutyResponse {}))
    }

    async fn speed_profile(
        &self,
        _request: Request<SpeedProfileRequest>,
    ) -> Result<Response<SpeedProfileResponse>, Status> {
        Err(Status::unimplemented("No Firmware Profiles"))
    }

    async fn lighting(
        &self,
        _request: Request<LightingRequest>,
    ) -> Result<Response<LightingResponse>, Status> {
        Err(Status::unimplemented("No Lighting Channels"))
    }

    async fn lcd(&self, _request: Request<LcdRequest>) -> Result<Response<LcdResponse>, Status> {
        Err(Status::unimplemented("No LCD Channels"))
    }

    /// This is a placeholder for any custom functions that the device service might expose.
    async fn custom_function_one(
        &self,
        _request: Request<CustomFunctionOneRequest>,
    ) -> Result<Response<CustomFunctionOneResponse>, Status> {
        Err(Status::unimplemented("No Custom Function"))
    }
}

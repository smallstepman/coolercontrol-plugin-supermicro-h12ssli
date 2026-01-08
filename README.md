# Custom Device Plugin

This plugin enables a relatively simple and convenient service where you can define your own shell commands for reading
and writing device channels.

> **Performance Note:** Spawning shell commands for each status poll (default: every second) incurs more overhead than
> native drivers. Use with care.

## Installation

### Quick Install (Prebuilt Binary)

Run the installation script to download and install the latest release:

```bash
curl -fsSL https://gitlab.com/coolercontrol/cc-plugin-custom-device/-/raw/main/install.sh | sh
```

To install a specific version:

```bash
curl -fsSL https://gitlab.com/coolercontrol/cc-plugin-custom-device/-/raw/main/install.sh | sh -s -- v0.1.0
```

### Build from Source

Alternatively you can build and install the plugin from source:

Requirements:

- Rust >= 1.88
- Make
- Cargo
- `protoc` (protobuf-compiler)

```bash
make install
```

## After Installation

If you need root permissions to write to your devices (e.g. to control fans), you will need to run the service with
privileged access. Edit the `manifest.toml` file and set `privileged = true`.

```bash
sudo nano /etc/coolercontrol/plugins/custom-device/manifest.toml
```

Restart the CoolerControl daemon:

```bash
sudo systemctl restart coolercontrold
```

Check the CoolerControl UI Plugin settings page to confirm your service is detected and started. You can then click on
the Plugin's settings button in the UI and configure your device channel shell commands.

### Configuration Notes

**Fan Channels:**

- At least one of `get_rpm` or `get_duty` is required; all other commands are optional
- `get_duty` should return an integer percentage (0–100)
- `get_rpm` should return an integer RPM value (typical range: 0–10,000)
- `set_duty` uses `{duty}` as a placeholder for the target duty percentage

**Temperature Sensors:**

- `command` is required
- Should return temperature in millidegrees Celsius (e.g., `45000` for 45°C)

## Problems?

Check the service logs for warnings and errors:
`journalctl -u service_name`.

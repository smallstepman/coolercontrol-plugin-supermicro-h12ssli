# Custom Device Plugin

This plugin enables a relatively simple and convenient service where you can define your own shell commands for reading
and writing device channels.

> **Performance Note:** Spawning shell commands for each status poll (default: every second) incurs more overhead than
> native drivers. Use with care.

## Installation

You can run the installation script:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://gitlab.com/coolercontrol/cc-plugin-custom-device/raw/main/install.sh | sh
curl -fsSL https://gitlab.com/coolercontrol/cc-plugin-custom-device/raw/main/install.sh | sh
```

Alternatively you can build the plugin from source:

Requirements:

- Rust >= 1.86
- Make
- Cargo
- `protoc` (protobuf-compiler)

```bash
make install
```

## After Installation

Restart the CoolerControl daemon:

```bash
sudo systemctl restart coolercontrold
```

Check the CoolerControl UI Plugin settings page to confirm your service is detected and started. You can then click on
the Plugin's settings button in the UI and configure your device channel shell commands.

## Problems?

Check the service logs for warnings and errors:
`journalctl -u service_name`.

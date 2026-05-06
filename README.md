# ⏻ wakeUP

A lightweight macOS menu bar Wake-on-LAN tool. Silently runs in the menu bar — no Dock icon, no main window, no clutter.

[中文文档](README_CN.md)

## Features

- 🔌 **Menu Bar Resident** — Lives in the macOS menu bar, zero Dock presence
- 📡 **One-Click Wake** — Click a device name to instantly send a WOL magic packet
- 💾 **Persistent Storage** — Devices saved to `~/.wol_devices.json`, survives restarts
- ✅ **MAC Validation** — Automatic format checking with error prompts
- 🔔 **System Notifications** — Confirmation via macOS Notification Center
- 🪶 **Ultra-Lightweight** — ~728KB, pure native, no runtime dependencies

## Screenshot

```
┌──────────────────────────┐
│ 🖥 Living Room PC        │
│ 💻 Office Desktop        │
│ ─────────────────────── │
│ Add New Device           │
│ Clear All Devices        │
│ ─────────────────────── │
│ Quit                     │
└──────────────────────────┘
```

## Requirements

- macOS 12+ (Monterey or later)
- [Rust](https://www.rust-lang.org/tools/install) (for building)
- [cargo-bundle](https://github.com/burtonageo/cargo-bundle) (for packaging)

## Build

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/wakeUP.git
cd wakeUP

# Build release binary
cargo build --release

# Package as macOS .app
cargo bundle --release

# The app will be at:
# target/release/bundle/osx/wakeUP.app

# Copy to Desktop (optional)
ditto target/release/bundle/osx/wakeUP.app ~/Desktop/wakeUP.app
```

## Usage

1. Launch **wakeUP.app** — a power icon appears in the menu bar
2. Click the icon → **Add New Device**
3. Fill in device Name, IP Address, and MAC Address in separate dialogs
4. Click the device name in the menu to send a WOL magic packet
5. Use **Clear All Devices** to remove all saved devices
6. **Quit** to exit

## Configuration

Devices are stored in `~/.wol_devices.json`:

```json
[
  {
    "name": "Living Room PC",
    "ip": "192.168.1.100",
    "mac": "AA:BB:CC:DD:EE:FF"
  }
]
```

## How It Works

When you click a device, wakeUP sends a WOL magic packet to:
1. The subnet broadcast address (e.g., `192.168.1.255:9`)
2. The global broadcast address (`255.255.255.255:9`)

This dual-send approach maximizes compatibility across different network configurations.

## Tech Stack

| Dependency | Version | Purpose |
|---|---|---|
| tray-icon | 0.11.0 | Menu bar icon management |
| wol | 0.2.0 | WOL magic packet sending |
| serde / serde_json | 1.0 | Configuration serialization |
| dirs | 5.0.1 | System directory paths |
| anyhow | 1.0.75 | Error handling |
| winit | 0.29.9 | Event loop |
| objc | 0.2.7 | macOS Dock icon hiding |

## License

[MIT](LICENSE)

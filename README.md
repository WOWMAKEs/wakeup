# ⏻ wakeUP

A lightweight macOS menu bar Wake-on-LAN tool written in Rust. No Dock icon, no main window — just a silent menu bar resident that wakes your devices with one click.

[中文文档](README_CN.md)

## Features

- 🔌 **Menu Bar Only** — Lives in the macOS menu bar, zero Dock presence
- 📡 **One-Click Wake** — Click a device name to send a WOL magic packet
- 📝 **Native Dialog** — Add device with a single window containing three labeled input fields
- 💾 **Persistent Storage** — Devices saved to `~/.wol_devices.json`, survives restarts
- ✅ **MAC Validation** — Automatic format checking with error prompts
- 🪶 **Ultra-Lightweight** — ~728KB, pure native, no runtime dependencies

## Menu Structure

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

## Add Device Dialog

A native macOS dialog with three labeled input fields in one window:

```
┌─────────────────────────────────────┐
│  Add Device                         │
│                                     │
│  Device Name:  [My PC           ]   │
│  IP Address:   [192.168.1.100   ]   │
│  MAC Address:  [AA:BB:CC:DD:EE:FF]  │
│                                     │
│                    [Cancel]  [OK]    │
└─────────────────────────────────────┘
```

## Requirements

- macOS 12+ (Monterey or later)
- [Rust](https://www.rust-lang.org/tools/install) (for building)
- [cargo-bundle](https://github.com/burtonageo/cargo-bundle) (for packaging)

## Build

```bash
git clone https://github.com/YOUR_USERNAME/wakeUP.git
cd wakeUP

# Build release binary
cargo build --release

# Package as macOS .app
cargo bundle --release

# Add LSUIElement to hide Dock icon
/usr/libexec/PlistBuddy -c "Add :LSUIElement bool true" \
  target/release/bundle/osx/wakeUP.app/Contents/Info.plist

# Or use the build script
chmod +x build.sh
./build.sh
```

The app will be at `target/release/bundle/osx/wakeUP.app`.

## Usage

1. Launch **wakeUP.app** — a power icon appears in the menu bar
2. Click the icon → **Add New Device**
3. Fill in Device Name, IP Address, and MAC Address → click OK
4. Click the device name in the menu to send a WOL magic packet
5. A system notification confirms the packet was sent
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

When you click a device, wakeUP sends a WOL magic packet to the subnet broadcast address (e.g., `192.168.1.255:9`). The packet is sent in a background thread so the menu stays responsive.

The add-device dialog is a compiled Swift binary cached at `~/.wakeup_dialog`. It's compiled once on first use, then launches instantly on subsequent uses.

## Tech Stack

| Dependency | Version | Purpose |
|---|---|---|
| tray-icon | 0.11.0 | Menu bar icon management |
| wol | 0.2.0 | WOL magic packet sending |
| serde / serde_json | 1.0 | Configuration serialization |
| dirs | 5.0.1 | System directory paths |
| anyhow | 1.0.75 | Error handling |
| winit | 0.29.9 | Event loop & activation policy |

## License

[MIT](LICENSE)

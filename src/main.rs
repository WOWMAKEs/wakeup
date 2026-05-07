use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;
use std::thread;
use std::sync::mpsc;
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use tray_icon::{TrayIcon, TrayIconBuilder};
use winit::event_loop::ControlFlow;
use winit::platform::macos::{ActivationPolicy, EventLoopBuilderExtMacOS};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Device {
    name: String,
    ip: String,
    mac: String,
}

fn app_dir() -> PathBuf {
    let dir = dirs::data_dir()
        .expect("Cannot find data directory")
        .join("wakeUP");
    if !dir.exists() {
        let _ = fs::create_dir_all(&dir);
    }
    dir
}

fn config_path() -> PathBuf {
    app_dir().join("devices.json")
}

fn load_devices() -> Vec<Device> {
    let path = config_path();
    if !path.exists() {
        return Vec::new();
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

fn save_devices(devices: &[Device]) -> Result<()> {
    let path = config_path();
    let content = serde_json::to_string_pretty(devices)?;
    fs::write(path, content)?;
    Ok(())
}

fn validate_mac(mac: &str) -> bool {
    let mac = mac.trim();
    let parts: Vec<&str> = mac.split(':').collect();
    if parts.len() == 6 {
        return parts.iter().all(|p| p.len() == 2 && u8::from_str_radix(p, 16).is_ok());
    }
    let parts: Vec<&str> = mac.split('-').collect();
    if parts.len() == 6 {
        return parts.iter().all(|p| p.len() == 2 && u8::from_str_radix(p, 16).is_ok());
    }
    false
}

fn normalize_mac(mac: &str) -> String {
    let hex: String = mac.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    (0..6)
        .map(|i| &hex[i * 2..i * 2 + 2])
        .collect::<Vec<_>>()
        .join(":")
        .to_uppercase()
}

fn compute_broadcast(ip: &str) -> String {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() == 4 {
        // Default to local subnet broadcast for /24, 
        // but fallback to global broadcast which is more reliable for WOL
        format!("{}.{}.{}.255", parts[0], parts[1], parts[2])
    } else {
        "255.255.255.255".to_string()
    }
}

fn send_wol(device: &Device) {
    let mac_str = normalize_mac(&device.mac);
    if let Ok(mac) = wol::MacAddr6::from_str(&mac_str) {
        let broadcast_str = compute_broadcast(&device.ip);
        if let Ok(broadcast) = broadcast_str.parse::<Ipv4Addr>() {
            let _ = wol::send_magic_packet(mac, None, SocketAddr::new(broadcast.into(), 9));
        }
    }
}

fn osascript(script: &str) -> Option<String> {
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

const DIALOG_SWIFT: &str = r#"
import Cocoa
let app = NSApplication.shared
app.setActivationPolicy(.accessory)
app.activate(ignoringOtherApps: true)
let alert = NSAlert()
alert.messageText = "Add Device"
alert.addButton(withTitle: "OK")
alert.addButton(withTitle: "Cancel")
let view = NSView(frame: NSRect(x: 0, y: 0, width: 340, height: 94))
let nameLabel = NSTextField(labelWithString: "Device Name:")
nameLabel.frame = NSRect(x: 0, y: 66, width: 105, height: 24)
nameLabel.alignment = .right
let nameField = NSTextField()
nameField.frame = NSRect(x: 115, y: 66, width: 215, height: 24)
nameField.placeholderString = "My PC"
let ipLabel = NSTextField(labelWithString: "IP Address:")
ipLabel.frame = NSRect(x: 0, y: 36, width: 105, height: 24)
ipLabel.alignment = .right
let ipField = NSTextField()
ipField.frame = NSRect(x: 115, y: 36, width: 215, height: 24)
ipField.placeholderString = "192.168.1.100"
let macLabel = NSTextField(labelWithString: "MAC Address:")
macLabel.frame = NSRect(x: 0, y: 6, width: 105, height: 24)
macLabel.alignment = .right
let macField = NSTextField()
macField.frame = NSRect(x: 115, y: 6, width: 215, height: 24)
macField.placeholderString = "AA:BB:CC:DD:EE:FF"
view.addSubview(nameLabel)
view.addSubview(nameField)
view.addSubview(ipLabel)
view.addSubview(ipField)
view.addSubview(macLabel)
view.addSubview(macField)
alert.accessoryView = view
let response = alert.runModal()
if response == .alertFirstButtonReturn {
    print("\(nameField.stringValue)||\(ipField.stringValue)||\(macField.stringValue)")
} else {
    print("CANCEL")
}
"#;

fn ensure_dialog_binary() -> Option<PathBuf> {
    let dir = app_dir();
    let bin_path = dir.join("wakeup_dialog");
    if bin_path.exists() {
        return Some(bin_path);
    }
    let src_path = dir.join("wakeup_dialog.swift");
    fs::write(&src_path, DIALOG_SWIFT).ok()?;
    let result = std::process::Command::new("swiftc")
        .args(["-o", bin_path.to_str()?, src_path.to_str()?])
        .output()
        .ok()?;
    let _ = fs::remove_file(&src_path);
    if result.status.success() {
        Some(bin_path)
    } else {
        let _ = fs::remove_file(&bin_path);
        None
    }
}

fn show_add_dialog() -> Option<(String, String, String)> {
    let bin_path = ensure_dialog_binary()?;
    let output = std::process::Command::new(&bin_path).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if result == "CANCEL" {
        return None;
    }
    let parts: Vec<&str> = result.split("||").collect();
    if parts.len() != 3 {
        return None;
    }
    let name = parts[0].trim().to_string();
    let ip = parts[1].trim().to_string();
    let mac = parts[2].trim().to_string();
    if name.is_empty() || ip.is_empty() || mac.is_empty() {
        show_error("All fields are required");
        return None;
    }
    Some((name, ip, mac))
}

fn show_error(message: &str) {
    let script = format!(
        "display dialog \"{}\" with title \"wakeUP\" buttons {{\"OK\"}} default button 1 with icon stop",
        message.replace('\\', "\\\\").replace('"', "\\\"")
    );
    let _ = osascript(&script);
}

fn show_notification(title: &str, body: &str) {
    let script = format!(
        "display notification \"{}\" with title \"{}\"",
        body.replace('\\', "\\\\").replace('"', "\\\""),
        title.replace('\\', "\\\\").replace('"', "\\\"")
    );
    let _ = osascript(&script);
}

fn confirm_clear() -> bool {
    let script = "display dialog \"Clear all devices?\" with title \"wakeUP\" buttons {\"Cancel\", \"Clear\"} default button 1 with icon caution";
    osascript(script).map_or(false, |r| r.contains("Clear"))
}

struct AppState {
    devices: Vec<Device>,
    device_items: Vec<MenuItem>,
    remove_items: Vec<MenuItem>,
    add_item: MenuItem,
    clear_item: MenuItem,
    quit_item: MenuItem,
}

impl AppState {
    fn new(devices: Vec<Device>) -> Self {
        Self {
            devices,
            device_items: Vec::new(),
            remove_items: Vec::new(),
            add_item: MenuItem::new("Add New Device", true, None),
            clear_item: MenuItem::new("Clear All Devices", true, None),
            quit_item: MenuItem::new("Quit", true, None),
        }
    }

    fn build_menu(&mut self) -> Menu {
        let menu = Menu::new();
        self.device_items.clear();
        self.remove_items.clear();

        if !self.devices.is_empty() {
            for device in &self.devices {
                let item = MenuItem::new(&device.name, true, None);
                self.device_items.push(item);
                let _ = menu.append(self.device_items.last().unwrap());
            }
            let _ = menu.append(&PredefinedMenuItem::separator());
        }

        let _ = menu.append(&self.add_item);

        if !self.devices.is_empty() {
            let remove_menu = Submenu::new("Remove Device", true);
            for device in &self.devices {
                let item = MenuItem::new(format!("Remove {}", device.name), true, None);
                self.remove_items.push(item);
                let _ = remove_menu.append(self.remove_items.last().unwrap());
            }
            let _ = menu.append(&remove_menu);
            let _ = menu.append(&self.clear_item);
        }

        let _ = menu.append(&PredefinedMenuItem::separator());
        let _ = menu.append(&self.quit_item);

        menu
    }
}

fn handle_clear_devices(state: &mut AppState, tray: &TrayIcon) {
    if state.devices.is_empty() {
        return;
    }

    if confirm_clear() {
        state.devices.clear();
        if let Err(e) = save_devices(&state.devices) {
            show_error(&format!("Failed to save: {}", e));
        }
        let menu = state.build_menu();
        tray.set_menu(Some(Box::new(menu)));
    }
}

fn handle_wake(device: Device) {
    send_wol(&device);
    show_notification("wakeUP", &format!("WOL sent to {}", device.name));
}

fn main() -> Result<()> {
    let devices = load_devices();

    let mut builder = winit::event_loop::EventLoopBuilder::new();
    builder.with_activation_policy(ActivationPolicy::Accessory);
    builder.with_default_menu(false);
    let event_loop = builder.build()?;

    let mut state = AppState::new(devices);
    let menu = state.build_menu();

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("wakeUP")
        .with_title("⏻")
        .with_menu_on_left_click(true)
        .build()?;

    let state = Rc::new(RefCell::new(state));
    let tray = Rc::new(RefCell::new(tray));

    let state_rc = state.clone();
    let tray_rc = tray.clone();

    let menu_channel = MenuEvent::receiver();
    let (proxy_tx, proxy_rx) = mpsc::channel::<(String, String, String)>();

    event_loop.run(move |_event, window_target| {
        window_target.set_control_flow(ControlFlow::Wait);

        if let Ok(new_device) = proxy_rx.try_recv() {
            let mut state = state_rc.borrow_mut();
            let tray = tray_rc.borrow();
            let (name, ip, mac) = new_device;
            if !validate_mac(&mac) {
                show_error("Invalid MAC format. Expected: XX:XX:XX:XX:XX:XX");
            } else {
                state.devices.push(Device { name, ip, mac });
                if let Err(e) = save_devices(&state.devices) {
                    show_error(&format!("Failed to save: {}", e));
                    state.devices.pop();
                } else {
                    let menu = state.build_menu();
                    tray.set_menu(Some(Box::new(menu)));
                }
            }
        }

        if let Ok(event) = menu_channel.try_recv() {
            let event_id = event.id;
            let mut state = state_rc.borrow_mut();
            let tray = tray_rc.borrow();

            if event_id == *state.quit_item.id() {
                window_target.exit();
            } else if event_id == *state.add_item.id() {
                let tx = proxy_tx.clone();
                thread::spawn(move || {
                    if let Some(res) = show_add_dialog() {
                        let _ = tx.send(res);
                    }
                });
            } else if event_id == *state.clear_item.id() {
                handle_clear_devices(&mut state, &tray);
            } else {
                let mut found = false;
                for (i, item) in state.device_items.iter().enumerate() {
                    if event_id == *item.id() {
                        let device = state.devices[i].clone();
                        thread::spawn(move || handle_wake(device));
                        found = true;
                        break;
                    }
                }

                if !found {
                    for (i, item) in state.remove_items.iter().enumerate() {
                        if event_id == *item.id() {
                            state.devices.remove(i);
                            let _ = save_devices(&state.devices);
                            let menu = state.build_menu();
                            tray.set_menu(Some(Box::new(menu)));
                            break;
                        }
                    }
                }
            }
        }
    })?;

    Ok(())
}

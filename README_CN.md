# ⏻ wakeUP

一款用 Rust 编写的轻量级 macOS 菜单栏网络唤醒工具。无 Dock 图标，无主窗口，静默驻留菜单栏，一键唤醒设备。

[English](README.md)

## 功能特性

- 🔌 **纯菜单栏** — 仅在顶部菜单栏显示图标，Dock 完全不可见
- 📡 **一键唤醒** — 点击设备名称即可发送 WOL 魔术包
- 📝 **原生对话框** — 单窗口内三个标签+输入框，一次填写完成
- 💾 **持久保存** — 设备列表保存在 `~/Library/Application Support/wakeUP/devices.json`，重启不丢失
- 🗑️ **设备管理** — 通过 "Remove Device" 子菜单轻松删除单个设备
- ⚡ **响应式 UI** — 异步对话框与后台执行，确保菜单栏操作丝滑顺畅
- ✅ **MAC 校验** — 自动校验 MAC 地址格式，输入错误弹出提示
- 🪶 **极致轻量** — 仅约 728KB，纯原生实现，无运行时依赖

## 菜单结构

```
┌──────────────────────────┐
│ 🖥 客厅电脑              │
│ 💻 办公台式机            │
│ ─────────────────────── │
│ Add New Device           │
│ Remove Device        >   │
│ Clear All Devices        │
│ ─────────────────────── │
│ Quit                     │
└──────────────────────────┘
```

## 添加设备对话框

原生 macOS 对话框，一个窗口内包含三个带标签的输入框：

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

## 系统要求

- macOS 12+（Monterey 及以上版本）
- [Rust](https://www.rust-lang.org/tools/install)（用于编译）
- [cargo-bundle](https://github.com/burtonageo/cargo-bundle)（用于打包）

## 编译构建

```bash
git clone https://github.com/YOUR_USERNAME/wakeUP.git
cd wakeUP

# 编译 Release 版本
cargo build --release

# 打包为 macOS .app
cargo bundle --release

# 添加 LSUIElement 隐藏 Dock 图标
/usr/libexec/PlistBuddy -c "Add :LSUIElement bool true" \
  target/release/bundle/osx/wakeUP.app/Contents/Info.plist

# 或使用构建脚本
chmod +x build.sh
./build.sh
```

应用包位于 `target/release/bundle/osx/wakeUP.app`。

## 使用方法

1. 启动 **wakeUP.app** — 菜单栏出现电源图标
2. 点击图标 → **Add New Device**
3. 填写设备名称、IP 地址、MAC 地址 → 点击 OK
4. 点击菜单中的设备名称即可发送 WOL 魔术包
5. 系统通知确认魔术包已发送
6. **Quit** 退出应用

## 配置文件

设备信息保存在 `~/Library/Application Support/wakeUP/devices.json`：

```json
[
  {
    "name": "客厅电脑",
    "ip": "192.168.1.100",
    "mac": "AA:BB:CC:DD:EE:FF"
  }
]
```

## 工作原理

点击设备后，wakeUP 向子网广播地址（如 `192.168.1.255:9`）发送 WOL 魔术包。发送在后台线程执行，菜单保持响应。

添加设备对话框为编译后的 Swift 二进制文件，缓存在 `~/.wakeup_dialog`。首次使用时编译一次，之后瞬间弹出。

## 技术栈

| 依赖 | 版本 | 用途 |
|---|---|---|
| tray-icon | 0.11.0 | 菜单栏图标管理 |
| wol | 0.2.0 | 发送 WOL 魔术包 |
| serde / serde_json | 1.0 | 配置文件序列化 |
| dirs | 5.0.1 | 系统目录读取 |
| anyhow | 1.0.75 | 错误处理 |
| winit | 0.29.9 | 事件循环与激活策略 |

## 开源许可

[MIT](LICENSE)

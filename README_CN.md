# ⏻ wakeUP

一款轻量级 macOS 菜单栏网络唤醒工具。静默驻留菜单栏，无 Dock 图标，无主窗口，纯净后台运行。

[English](README.md)

## 功能特性

- 🔌 **菜单栏常驻** — 仅在顶部菜单栏显示图标，完全不显示 Dock 图标
- 📡 **一键唤醒** — 点击设备名称即可发送 WOL 魔术包
- 💾 **持久保存** — 设备列表保存在 `~/.wol_devices.json`，重启不丢失
- ✅ **MAC 校验** — 自动校验 MAC 地址格式，输入错误弹出提示
- 🔔 **系统通知** — 唤醒成功后通过 macOS 通知中心提醒
- 🪶 **极致轻量** — 仅约 728KB，纯原生实现，无运行时依赖

## 截图示意

```
┌──────────────────────────┐
│ 🖥 客厅电脑              │
│ 💻 办公台式机            │
│ ─────────────────────── │
│ Add New Device           │
│ Clear All Devices        │
│ ─────────────────────── │
│ Quit                     │
└──────────────────────────┘
```

## 系统要求

- macOS 12+（Monterey 及以上版本）
- [Rust](https://www.rust-lang.org/tools/install)（用于编译）
- [cargo-bundle](https://github.com/burtonageo/cargo-bundle)（用于打包）

## 编译构建

```bash
# 克隆仓库
git clone https://github.com/YOUR_USERNAME/wakeUP.git
cd wakeUP

# 编译 Release 版本
cargo build --release

# 打包为 macOS .app
cargo bundle --release

# 应用包位于：
# target/release/bundle/osx/wakeUP.app

# 复制到桌面（可选）
ditto target/release/bundle/osx/wakeUP.app ~/Desktop/wakeUP.app
```

## 使用方法

1. 启动 **wakeUP.app** — 菜单栏出现电源图标
2. 点击图标 → **Add New Device**
3. 依次填写设备名称、IP 地址、MAC 地址
4. 点击菜单中的设备名称即可发送 WOL 魔术包
5. 使用 **Clear All Devices** 清空所有设备
6. **Quit** 退出应用

## 配置文件

设备信息保存在 `~/.wol_devices.json`：

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

点击设备后，wakeUP 会同时向两个地址发送 WOL 魔术包：
1. 子网广播地址（如 `192.168.1.255:9`）
2. 全局广播地址（`255.255.255.255:9`）

双路发送策略最大化了不同网络环境下的兼容性。

## 技术栈

| 依赖 | 版本 | 用途 |
|---|---|---|
| tray-icon | 0.11.0 | 菜单栏图标管理 |
| wol | 0.2.0 | 发送 WOL 魔术包 |
| serde / serde_json | 1.0 | 配置文件序列化 |
| dirs | 5.0.1 | 系统目录读取 |
| anyhow | 1.0.75 | 错误处理 |
| winit | 0.29.9 | 事件循环 |
| objc | 0.2.7 | macOS Dock 图标隐藏 |

## 开源许可

[MIT](LICENSE)

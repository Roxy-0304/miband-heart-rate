[English](README_EN.md) or [中文](README.md)

## ⚠️ Disclaimer

> This project is forked from [Tnze/miband-heart-rate](https://github.com/Tnze/miband-heart-rate), code written by AI.

---

## 📋 Table of Contents

- [About](#about)
- [✨ Features](#features)
- [🖥️ Dual Interface](#dual-interface)
  - [Desktop App (Tauri)](#desktop-app-tauri)
  - [Web Server (OBS)](#web-server-obs)
  - [Heart Rate Zones](#heart-rate-zones)
- [📺 OBS Live Stream Overlay Setup](#obs-live-stream-overlay-setup)
- [📦 Quick Start](#quick-start)
  - [Download Pre-built Binary](#download-pre-built-binary)
  - [Build from Source](#build-from-source)
- [🚀 Usage Guide](#usage-guide)
- [⚙️ Environment Variables](#environment-variables)
- [📁 Project Structure](#project-structure)
- [🖼️ Screenshots](#screenshots)
- [Supported Platforms](#supported-platforms)
- [Compatible Devices](#compatible-devices)

---

## About

**Band Heart Rate Monitor** is a Tauri-based desktop heart rate monitoring application that receives real-time heart rate data from wearable devices via the standard BLE Heart Rate Service (UUID 0x180D). It also includes a built-in web server that can be used as an OBS live stream overlay.

You need to enable the heart rate broadcast function in your wearable device's settings.

> 💡 Latest builds can be downloaded from the [GitHub Releases](https://github.com/Roxy-0304/band-heart-rate/releases) page.

---

## ✨ Features

- **Tauri Desktop App** — Sleek dark-themed UI, minimizes to system tray when closed, continues collecting data in the background
- **Real-time Heart Rate Display** — Large digits with pulsing heartbeat animation
- **Heart Rate Zone Detection** — Automatically identifies Warmup / Fat Burn / Aerobic / Limit zones with color-coded indicators
- **Live Statistics** — Automatically tracks min / max / average heart rate, with one-click reset
- **Connection Status** — Status bar shows Connected / Scanning / Disconnected
- **System Tray** — Close window to tray, keep the app running in the background; click tray icon to restore
- **Web Server** — Built-in web server for OBS live stream overlays, supports custom CSS
- **OBS Live Stream Compatible** — Transparent background, no green screen needed
- **Auto Reconnect** — Automatically scans and reconnects when device disconnects, with exponential backoff
- **Cross-platform** — Windows, macOS/iOS, Linux

---

## 🖥️ Dual Interface

The program provides two independent interfaces that work simultaneously:

### Desktop App (Tauri)

The main interface with full interactive experience:

- Large real-time heart rate display with heartbeat animation
- Heart rate zone color badges (Warmup / Fat Burn / Aerobic / Limit)
- Min / Max / Average statistics panel
- Reset statistics button
- System tray icon: right-click to show window or quit
- **Closing the window does NOT exit the app** — it minimizes to the system tray and continues collecting data
- Re-open the app or click tray "Show Window" to restore the interface

### Web Server (OBS)

Used for OBS live stream overlays. Default address:

```
http://127.0.0.1:3030
```

(Automatically switches to an available port if 3030 is occupied. Check the startup message for the actual address.)

Web page features:
- Designed for **1920x1080** resolution, suitable for fullscreen display
- **Transparent background** — can be overlaid directly onto live streams without a green screen
- Heart icon with **pulsing animation**
- Heart rate number displayed in **Orbitron font** for a tech-forward look
- Number appears **dimmed** when not connected

**Custom Styles:**

```css
:root {
  --red: #00FF00;   /* Change to green */
}
```

---

### Heart Rate Zones

Zones are automatically calculated as a percentage of **maximum heart rate** (default reference: `220 - 30 = 190`):

| Zone | Range | Color | Description |
|------|-------|-------|-------------|
| 🟦 Warmup | 0%–60% | Blue `#4FC3F7` | Low intensity, suitable for warming up |
| 🟩 Fat Burn | 60%–70% | Green `#66BB6A` | Moderate intensity, optimal fat burning |
| 🟧 Aerobic | 70%–80% | Orange `#FFA726` | High intensity aerobic, improves cardiovascular fitness |
| 🟥 Limit | 80%–100% | Red `#EF5350` | Very high intensity, anaerobic endurance training |

---

## 📺 OBS Live Stream Overlay Setup

1. Run the program and ensure the Web UI is accessible
2. In OBS, click `+` → **Browser** to add a browser source
3. Configure the source properties:
   - **URL**: `http://127.0.0.1:3030`
   - **Width**: `1920`
   - **Height**: `1080`
4. Click OK to finish

The page has a transparent background by default — no chroma key or green screen setup needed.

---

## 📦 Quick Start

### Download Pre-built Binary

1. Go to the [GitHub Releases page](https://github.com/Roxy-0304/band-heart-rate/releases)
2. Download the latest `band-heart-rate.exe` (or the executable for your platform)
3. Run it directly

### Build from Source

**Requirements:**

- [Rust toolchain](https://www.rust-lang.org/tools/install) (recommended via rustup)
- [Node.js](https://nodejs.org/) (for compiling the frontend TypeScript)

**Build steps:**

```bash
# Clone the repository
git clone https://github.com/Roxy-0304/band-heart-rate.git
cd band-heart-rate

# Install frontend dependencies and build
cd frontend
npm install
npm run build
cd ..

# Build release version
cargo build --release

# The executable is at target/release/band-heart-rate.exe
```

---

## 🚀 Usage Guide

1. Enable **Heart Rate Broadcast** in your **band/watch settings**
2. Make sure Bluetooth is enabled on your device
3. Run the program
4. The program will automatically scan for nearby heart rate broadcasting devices and connect
5. Heart rate data will be displayed in real-time on the desktop interface, and you can also access the Web UI in your browser
6. **Close window**: minimizes to system tray, continues background collection
7. **Exit completely**: right-click tray icon → Quit

---

## ⚙️ Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `MIBAND_ALLOWED_DEVICES` | Comma-separated device name keywords for allowed connections (case-insensitive) | `band,amazfit,watch,mi` |

**Example:**

```bash
# Only allow devices containing "mi" or "honor"
set MIBAND_ALLOWED_DEVICES=mi,honor
band-heart-rate.exe
```

---

## 📁 Project Structure

```
band-heart-rate/
├── src/
│   └── main.rs              # Main entry point (Bluetooth, Web server, Tauri event system)
├── frontend/
│   ├── index.html           # Tauri desktop window HTML
│   ├── style.css            # Dark tech theme styles
│   ├── app.ts               # TypeScript frontend logic (heart rate display, stats, zones, events)
│   ├── package.json         # Frontend dependency management
│   ├── tsconfig.json        # TypeScript config
│   └── dist/                # Compiled JS output
├── icons/
│   └── icon.ico             # App icon
├── capabilities/
│   └── default.json         # Tauri permissions
├── doc/
│   ├── 1.png                # Screenshot (to be updated with desktop UI)
│   └── 2.gif                # Animated screenshot
├── .github/
│   └── workflows/
│       ├── ci.yml           # CI workflow
│       └── release.yml      # GitHub Actions automated build & release
├── build.rs                 # Tauri build script
├── tauri.conf.json          # Tauri configuration
├── Cargo.toml               # Rust project configuration and dependencies
├── Cargo.lock               # Dependency lock file
├── README.md                # Chinese documentation
├── README_EN.md             # English documentation
└── LICENSE                  # Open source license
```

---

## 🖼️ Screenshots

![Desktop Main Interface](doc/1.png)

![Tauri Desktop UI Feature Demo](doc/3.png)

![Live Demo](doc/2.gif)

---

## Supported Platforms

Uses the `bluest` crate. Here is its description:

> Bluest is a cross-platform Rust low-power Bluetooth (BLE) library. Currently supports Windows (version 10 and above), MacOS/iOS, and Linux. Android support is planned.

Therefore, supported:

- Windows 10/11
- MacOS/iOS
- Linux

---

## Compatible Devices

This program is compatible with any wearable device that supports the standard BLE Heart Rate Service (UUID 0x180D), including but not limited to:

- **Xiaomi Mi Band** series (tested on Mi Band 10)
- **Honor Band** series
- **Huawei Band/Watch** series
- **Amazfit** devices
- **Apple Watch**
- Other sports watches / chest straps that support BLE heart rate broadcasting

> Enable the "Heart Rate Broadcast" feature in your device settings to be detected by this program.

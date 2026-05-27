[English](README_EN.md) 或 [中文](README.md)

## ⚠️ 免责声明

> 本项目 Fork 自 [Tnze/miband-heart-rate](https://github.com/Tnze/miband-heart-rate)，代码由 AI 编写。

---

## 📋 目录

- [项目简介](#项目简介)
- [✨ 功能特性](#功能特性)
- [🖥️ 双界面说明](#双界面说明)
  - [桌面应用（Tauri）](#桌面应用tauri)
  - [Web 服务器（OBS）](#web-服务器obs)
  - [心率区间说明](#心率区间说明)
- [📺 OBS 直播叠加层设置](#obs-直播叠加层设置)
- [📦 快速开始](#快速开始)
  - [下载构建产物](#下载构建产物)
  - [从源码编译](#从源码编译)
- [🚀 使用指南](#使用指南)
- [⚙️ 环境变量](#环境变量)
- [📁 项目结构](#项目结构)
- [🖼️ 截图](#截图)
- [支持的平台](#支持的平台)
- [兼容设备](#兼容设备)

---

## 项目简介

**Band Heart Rate Monitor** 是一个基于 Tauri 的桌面心率监测应用，通过标准 BLE 心率服务（Heart Rate Service, UUID 0x180D）接收穿戴设备的实时心率数据。同时内置 Web 服务器，可作为 OBS 直播叠加层使用。

需要在手环/手表的设置中开启心率广播功能。

> 💡 最新构建版本可从 [GitHub Releases](https://github.com/Roxy-0304/band-heart-rate/releases) 页面下载。

---

## ✨ 功能特性

- **Tauri 桌面应用** — 暗色科技风 UI，关窗后最小化到系统托盘，后台持续采集
- **实时心率显示** — 大数字实时刷新，带有心跳脉冲动画
- **心率区间识别** — 自动判断热身/燃脂/有氧/极限四区，彩色标识
- **实时统计** — 自动记录最低/最高/平均心率，支持一键重置
- **连接状态指示** — 状态栏显示已连接/扫描中/断开连接
- **托盘常驻** — 关闭窗口后程序在后台继续运行，点击托盘图标重新显示
- **Web 服务器** — 作为 OBS 直播叠加层使用，支持自定义 CSS
- **OBS 直播兼容** — 页面自带透明背景，无需绿幕
- **自动重连** — 设备断开后自动扫描并重新连接，指数退避
- **跨平台支持** — Windows、macOS/iOS、Linux

---

## 🖥️ 双界面说明

本程序提供两个独立的界面，互不干扰：

### 桌面应用（Tauri）

主界面，提供完整的交互体验：

- 实时心率大数字 + 心跳动画
- 心率区间彩色标签（热身/燃脂/有氧/极限）
- 最低/最高/平均统计面板
- 重置统计按钮
- 系统托盘图标：右键可显示窗口或退出程序
- **关闭窗口不会退出程序** — 程序将最小化到系统托盘继续采集数据
- 再次打开程序或点击托盘「显示窗口」可恢复界面

### Web 服务器（OBS）

用于 OBS 直播叠加层，默认地址：

```
http://127.0.0.1:3030
```

（端口冲突时自动切换，启动时会有提示）

Web 页面特性：
- 设计尺寸为 **1920x1080**，适合全屏展示
- **透明背景**，无需绿幕即可直接叠加到直播画面
- 心跳图标带有 **脉冲动画** 效果
- 心率数字使用 **Orbitron 字体** 科技感显示
- 未连接时数字呈 **半透明** 状态

**自定义样式：**

```css
:root {
  --red: #00FF00;   /* 改为绿色 */
}
```

---

### 心率区间说明

区间按 **最大心率百分比** 自动划分（默认参考最大心率 `220 - 30 = 190`）：

| 区间 | 范围 | 颜色 | 说明 |
|------|------|------|------|
| 🟦 热身 | 0%–60% | 蓝色 `#4FC3F7` | 低强度活动，适合热身 |
| 🟩 燃脂 | 60%–70% | 绿色 `#66BB6A` | 中等强度，脂肪燃烧效率最高 |
| 🟧 有氧 | 70%–80% | 橙色 `#FFA726` | 高强度有氧，提升心肺功能 |
| 🟥 极限 | 80%–100% | 红色 `#EF5350` | 极高强度，无氧耐力训练 |

---

## 📺 OBS 直播叠加层设置

1. 运行程序，确保 Web 界面可访问
2. 在 OBS 中点击 `+` → **浏览器** 添加浏览器源
3. 在属性中设置：
   - **URL**：`http://127.0.0.1:3030`
   - **宽度**：`1920`
   - **高度**：`1080`
4. 点击确定完成添加

页面自带透明背景，无需额外抠图或绿幕设置。

---

## 📦 快速开始

### 下载构建产物

1. 前往 [GitHub Releases 页面](https://github.com/Roxy-0304/band-heart-rate/releases)
2. 下载最新版本的 `band-heart-rate.exe`（或对应平台的可执行文件）
3. 直接运行即可

### 从源码编译

**环境要求：**

- [Rust 工具链](https://www.rust-lang.org/tools/install)（推荐使用 rustup 安装）
- [Node.js](https://nodejs.org/)（用于编译前端 TypeScript 代码）

**编译步骤：**

```bash
# 克隆仓库
git clone https://github.com/Roxy-0304/band-heart-rate.git
cd band-heart-rate

# 安装前端依赖并构建
cd frontend
npm install
npm run build
cd ..

# 编译发布版本
cargo build --release

# 可执行文件位于 target/release/band-heart-rate.exe
```

---

## 🚀 使用指南

1. 在**手环/手表设置**中开启 **心率广播** 功能
2. 确保设备蓝牙已开启
3. 运行程序
4. 程序会自动扫描周围的心率广播设备并连接
5. 心率数据将在桌面界面实时显示，同时可通过浏览器访问 Web 界面
6. **关闭窗口**：程序最小化到系统托盘，继续后台采集
7. **彻底退出**：右键托盘图标 → 退出

---

## ⚙️ 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `MIBAND_ALLOWED_DEVICES` | 允许连接的设备名称关键词（逗号分隔，不区分大小写） | `band,amazfit,watch,mi` |

**示例：**

```bash
# 仅允许连接包含 "mi" 或 "honor" 的设备
set MIBAND_ALLOWED_DEVICES=mi,honor
band-heart-rate.exe
```

---

## 📁 项目结构

```
band-heart-rate/
├── src/
│   └── main.rs              # 主程序入口（蓝牙、Web 服务器、Tauri 事件系统）
├── frontend/
│   ├── index.html           # Tauri 桌面窗口 HTML
│   ├── style.css            # 暗色科技风样式
│   ├── app.ts               # TypeScript 前端逻辑（心率显示、统计、区间、事件监听）
│   ├── package.json         # 前端依赖管理
│   ├── tsconfig.json        # TypeScript 编译配置
│   └── dist/                # 编译后的 JS 输出
├── icons/
│   └── icon.ico             # 应用图标
├── capabilities/
│   └── default.json         # Tauri 权限配置
├── doc/
│   ├── 1.png                # 截图（待更新为桌面 UI 截图）
│   └── 2.gif                # 动图
├── .github/
│   └── workflows/
│       ├── ci.yml           # CI 工作流
│       └── release.yml      # GitHub Actions 自动构建发布
├── build.rs                 # Tauri 构建脚本
├── tauri.conf.json          # Tauri 配置
├── Cargo.toml               # Rust 项目配置与依赖
├── Cargo.lock               # 依赖锁定文件
├── README.md                # 中文文档
├── README_EN.md             # 英文文档
└── LICENSE                  # 开源许可证
```

---

## 🖼️ 截图

![桌面主界面](doc/1.png)

> 📸 截图展示了 Tauri 桌面应用的暗色科技风界面，包括心率大数字、区间标识、统计面板和连接状态。Web OBS 叠加层请参考 [Web UI 说明](#web-服务器obs)。

![运行演示](doc/2.gif)

---

## 支持的平台

使用 `bluest` crate。以下引用其说明：

> Bluest 是一个跨平台的 Rust 低功耗蓝牙（BLE）库。目前支持 Windows（版本 10 及以上）、MacOS/iOS 和 Linux。Android 支持正在计划中。

因此支持：

- Windows 10/11
- MacOS/iOS
- Linux

---

## 兼容设备

本项目兼容任何支持标准 BLE 心率服务（UUID 0x180D）的穿戴设备，包括但不限于：

- **小米手环**系列（已在 小米手环 10 上测试通过）
- **荣耀手环**系列
- **华为手环/手表**系列
- **Amazfit** 品牌设备
- **Apple Watch**
- 其他支持 BLE 心率广播的运动手表/胸带等

> 如果设备支持"心率广播"功能，在设备设置中开启后即可被本程序识别。

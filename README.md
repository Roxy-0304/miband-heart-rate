[English](README_EN.md) [中文](README.md)

## 免责声明

> 本项目 Fork 自 [Tnze/miband-heart-rate](https://github.com/Tnze/miband-heart-rate)，代码由 AI 编写。

# 小米手环心率演示程序

接收小米手环“运动心率广播”的演示程序。需要在手环设置-心率广播中开启广播功能。

欢迎二次开发。

## 支持的平台

使用 `bluest` crate。以下引用其说明：

> Bluest 是一个跨平台的 Rust 低功耗蓝牙（BLE）库。目前支持 Windows（版本 10 及以上）、MacOS/iOS 和 Linux。Android 支持正在计划中。

因此理论支持：

- Windows 10/11(测试通过)
- MacOS/iOS
- Linux

## 支持的小米手环

已在 小米手环 10 上测试通过。

## 功能特性

- **实时心率显示**：使用 `print!` + `flush()` 在同一行实时刷新
- **Web 界面**：在浏览器中实时显示心率数据和传感器接触状态
- **自定义样式**：支持在 Web 界面中注入自定义 CSS
- **跨平台支持**：Windows、macOS/iOS、Linux

### 访问 Web 界面

在浏览器中打开：`http://127.0.0.1:3030`

实时查看心率数据和传感器状态。

## 截图

![Alt text](doc/image.png)
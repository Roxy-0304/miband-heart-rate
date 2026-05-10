
## 免责声明

> 本项目 Fork 自 [Tnze/miband-heart-rate](https://github.com/Tnze/miband-heart-rate)，代码由 AI 编写。

# 小米手环心率演示程序

接收小米手环“运动心率广播”的演示程序。需要在手环设置-心率广播中开启广播功能。

欢迎二次开发。

## 支持的平台

使用 `bluest` crate。以下引用其说明：

> Bluest 是一个跨平台的 Rust 低功耗蓝牙（BLE）库。目前支持 Windows（版本 10 及以上）、MacOS/iOS 和 Linux。Android 支持正在计划中。

因此支持：

- Windows 10/11
- MacOS/iOS
- Linux

## 支持的小米手环

小米手环 10

已在 MiBand10/NFC 上测试通过。

## 功能特性

- **实时心率显示**：使用 `print!` + `flush()` 在同一行实时刷新，无滚动
- **Web 界面**：在浏览器中实时显示心率数据和传感器接触状态
- **自定义样式**：支持在 Web 界面中注入自定义 CSS
- **跨平台支持**：Windows、macOS/iOS、Linux 均可运行

### 访问 Web 界面

在浏览器中打开：`http://127.0.0.1:3030`

实时查看心率数据和传感器状态。

## 截图

![Alt text](doc/screenshot.png)

## 推荐的 CSS

```css
/* 1. 全局布局：背景透明 */
html, body {
    background-color: rgba(0, 0, 0, 0) !important;
    margin: 0;
    padding: 0;
    overflow: hidden;
    height: 100vh;
    display: flex;
    align-items: center;
}

/* 2. 【核心修改】隐藏逻辑：先让所有东西透明 */
body * {
    opacity: 0;
    transition: opacity 0.3s ease;
}

/* 3. 强制显示数字和心跳（无论它在哪里层级） */
#heart-rate, .heart-rate, .bpm-value, 
[class*="heart-rate"], [id*="heart-rate"], 
.value, .number {
    opacity: 1 !important;
    visibility: visible !important;
    color: #FF3B30 !important;
    font-family: "Arial Black", sans-serif;
    font-size: 85px !important;
    font-weight: 900;
    display: flex !important;
    align-items: center !important;
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.4);
}

/* 4. 左侧 SVG 爱心 */
#heart-rate::before, .heart-rate::before, .bpm-value::before,
[class*="heart-rate"]::before, .value::before {
    content: "";
    display: inline-block !important;
    width: 70px;
    height: 70px;
    margin-right: 15px;
    background-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="%23FF3B30"><path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/></svg>');
    background-repeat: no-repeat;
    background-size: contain;
    animation: heartBeat 1.2s infinite;
}

/* 5. 鼠标悬停安全网：移入时显示设置按钮 */
body:hover * {
    opacity: 1 !important;
}

/* 6. 心跳动画 */
@keyframes heartBeat {
    0% { transform: scale(1); }
    10% { transform: scale(1.1); }
    20% { transform: scale(1); }
}

/* 7. 彻底移除可能干扰的背景色块 */
div, section, main {
    background: transparent !important;
    box-shadow: none !important;
}
```

性能优化建议（解决数据延迟）

由于此类挂件基于浏览器内核运行，长时间后台挂载可能会触发 Edge 浏览器的“休眠”机制，导致心率刷新不流畅。
**操作步骤：**
1. 在浏览器地址栏输入：`edge://settings/system`
2. 找到 **“使用标签页休眠功能保存资源”** 选项。
3. 在下方 **“永不使这些站点进入休眠状态”** 列表中，点击“添加”。
4. 将你心率网页的 **URL 地址** 填入即可。
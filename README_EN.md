## Disclaimer

> This project is Forked from [Tnze/miband-heart-rate](https://github.com/Tnze/miband-heart-rate), code written by AI.

# MiBand Heart Rate Demo

A demo that receives the "Sports Heart Rate Broadcast" from Xiaomi Smart Band. Enabling the broadcast in the band's settings is required.

Welcome to do further development.

## Supported Platforms

Uses the `bluest` crate. Quoting from its description:

> Bluest is a cross-platform Bluetooth Low Energy (BLE) library for Rust. It currently supports Windows (version 10 and later), MacOS/iOS, and Linux. Android support is planned.

Thus supported:

- Windows 10/11
- MacOS/iOS
- Linux

## Supported MiBands

Xiaomi Smart Band 10

Tested on MiBand10/NFC.

## Features

- **Real-time heart rate display**: Uses `print!` + `flush()` to refresh on the same line without scrolling.
- **Web interface**: Displays real-time heart rate data and sensor contact status in a browser.
- **Custom styling**: Allows injecting custom CSS into the web interface.
- **Cross-platform**: Runs on Windows, macOS/iOS, and Linux.

### Accessing the Web Interface

Open in your browser: `http://127.0.0.1:3030`

View real-time heart rate data and sensor status.

## Screenshot

![Alt text](doc/screenshot.png)

## Recommended CSS

```css
/* 1. Global layout: transparent background */
html, body {
    background-color: rgba(0, 0, 0, 0) !important;
    margin: 0;
    padding: 0;
    overflow: hidden;
    height: 100vh;
    display: flex;
    align-items: center;
}

/* 2. [Core modification] Hide logic: make everything transparent first */
body * {
    opacity: 0;
    transition: opacity 0.3s ease;
}

/* 3. Force display of numbers and heartbeat (no matter where they are) */
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

/* 4. Left-side SVG heart */
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

/* 5. Mouse hover safety net: show settings button on hover */
body:hover * {
    opacity: 1 !important;
}

/* 6. Heartbeat animation */
@keyframes heartBeat {
    0% { transform: scale(1); }
    10% { transform: scale(1.1); }
    20% { transform: scale(1); }
}

/* 7. Completely remove possible interfering background blocks */
div, section, main {
    background: transparent !important;
    box-shadow: none !important;
}
```

## Performance Optimization Advice (Resolving Data Lag)

Since such widgets run on a browser engine, long-term background operation may trigger Edge browser's "sleep" mechanism, causing heart rate updates to become choppy.

**Steps:**
1. Enter in the browser address bar: `edge://settings/system`
2. Find the **"Save resources with sleeping tabs"** option.
3. Below, in the **"Never put these sites to sleep"** list, click "Add".
4. Enter the **URL** of your heart rate webpage.
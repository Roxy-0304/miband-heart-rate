use std::error::Error;
use std::io::Write;
use std::time::Instant;

use axum::{extract::State, response::Html, routing::get, Json, Router};
use bluest::{btuuid::bluetooth_uuid_from_u16, Adapter, Device, Uuid};
use futures_lite::stream::StreamExt;
use serde::Serialize;
use tokio::sync::watch;
use tokio::signal;
use tokio::time::{timeout, Duration};

const HRS_UUID: Uuid = bluetooth_uuid_from_u16(0x180D);
const HRM_UUID: Uuid = bluetooth_uuid_from_u16(0x2A37);

#[derive(Clone, Copy, Serialize)]
struct HeartRateReading {
    heart_rate: u16,
    sensor_contact: Option<bool>,
    connected: bool,
    scanning: bool,
}

#[derive(Clone)]
struct AppState {
    rx: watch::Receiver<HeartRateReading>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = watch::channel(HeartRateReading {
        heart_rate: 0,
        sensor_contact: None,
        connected: false,
        scanning: false,
    });

    tokio::spawn(async move {
        if let Err(err) = run_server(rx).await {
            eprint!("\nWeb server error: {err}\n");
            std::io::stderr().flush().unwrap();
        }
    });

    let adapter = Adapter::default()
        .await
        .ok_or("Bluetooth adapter not found")?;
    adapter.wait_available().await?;

    tokio::select! {
        _ = signal::ctrl_c() => {
            print!("Received shutdown signal, exiting...\n");
            std::io::stdout().flush().unwrap();
        }
        result = run_loop(adapter, tx) => {
            if let Err(e) = result {
                eprint!("\nLoop error: {e}\n");
                std::io::stderr().flush().unwrap();
            }
        }
    }

    Ok(())
}

async fn run_loop(
    adapter: Adapter,
    tx: watch::Sender<HeartRateReading>,
) -> Result<(), Box<dyn Error>> {
    let mut disconnect_time: Option<Instant> = None;
    
    loop {
        // Check if we've been disconnected for too long
        if let Some(time) = disconnect_time {
            let elapsed = time.elapsed().as_secs();
            if elapsed >= 120 {
                eprint!("\nScan timeout: No device found in 2 minutes, exiting...\n");
                std::io::stderr().flush().unwrap();
                return Err("Scan timeout: No device found in 2 minutes".into());
            }
        }
        
        // No connected device, try to scan
        if disconnect_time.is_none() {
            disconnect_time = Some(Instant::now());
        }
        
        // Update state to show we're scanning
        tx.send_replace(HeartRateReading {
            heart_rate: 0,
            sensor_contact: None,
            connected: false,
            scanning: true,
        });
        
        print!("Starting scan...\n");
        std::io::stdout().flush().unwrap();
        
        // Try scanning with shorter timeout
        match scan_device_with_timeout(&adapter, tx.clone()).await {
            Ok(device) => {
                print!("Device found, attempting to connect...\n");
                std::io::stdout().flush().unwrap();
                
                disconnect_time = None;
                if let Err(err) = handle_device(&adapter, &device, tx.clone()).await {
                    print!("Connection failed: {:?}\n", err);
                    std::io::stdout().flush().unwrap();
                    
                    // Check if the error is due to device disconnection or stopping broadcast
                    let err_msg = err.to_string();
                    if err_msg.contains("stopped broadcasting") || err_msg.contains("disconnected") {
                        // Device disconnected or stopped broadcasting, attempt to reconnect
                        print!("Device disconnected, attempting to reconnect...\n");
                        std::io::stdout().flush().unwrap();
                        
                        tx.send_replace(HeartRateReading {
                            heart_rate: 0,
                            sensor_contact: None,
                            connected: false,
                            scanning: false,
                        });
                        
                        // Reset disconnect time to restart the 2-minute timeout
                        disconnect_time = Some(Instant::now());
                        
                        // Wait a bit before attempting to reconnect
                        tokio::time::sleep(Duration::from_secs(2)).await;
                        continue; // Continue the loop to scan and reconnect
                    }
                    
                    tx.send_replace(HeartRateReading {
                        heart_rate: 0,
                        sensor_contact: None,
                        connected: false,
                        scanning: false,
                    });
                    disconnect_time = Some(Instant::now());
                    eprint!("\rConnection error: {err:?}                                                   ");
                    std::io::stderr().flush().unwrap();
                }
            }
            Err(err) => {
                print!("Scan failed: {:?}\n", err);
                std::io::stdout().flush().unwrap();
                
                eprint!("\rScan error: {err:?}                                                   ");
                std::io::stderr().flush().unwrap();
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn scan_device_with_timeout(adapter: &Adapter, tx: watch::Sender<HeartRateReading>) -> Result<Device, Box<dyn Error>> {
    print!("Starting scan\n");
    std::io::stdout().flush().unwrap();
    
    // Notify that scanning has started
    tx.send_replace(HeartRateReading {
        heart_rate: 0,
        sensor_contact: None,
        connected: false,
        scanning: true,
    });
    
    let mut scan = adapter.discover_devices(&[HRS_UUID]).await?;
    print!("Scan started\n");
    std::io::stdout().flush().unwrap();
    
    // Use a shorter timeout - 30 seconds instead of 120
    match timeout(Duration::from_secs(30), scan.next()).await {
        Ok(Some(Ok(device))) => {
            // Device found, stop scanning
            tx.send_replace(HeartRateReading {
                heart_rate: 0,
                sensor_contact: None,
                connected: false,
                scanning: false,
            });
            print!("Found Device: [{}] {:?}\n", device, device.name_async().await);
            std::io::stdout().flush().unwrap();
            Ok(device)
        },
        Ok(Some(Err(e))) => Err(Box::new(e)),
        Ok(None) => Err("No device found".into()),
        Err(_) => Err("Scan timeout: No device found in 30 seconds".into()),
    }
}

async fn run_server(rx: watch::Receiver<HeartRateReading>) -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(index))
        .route("/heart-rate", get(heart_rate))
        .with_state(AppState { rx });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await?;
    print!("Serving web UI at http://127.0.0.1:3030/\n");
    std::io::stdout().flush().unwrap();

    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <title>Mi Band Heart Rate</title>
    <style>
        body { font-family: Arial, sans-serif; padding: 2rem; }
        .value { font-size: 3rem; margin: 1rem 0; }
        .label { color: #555; }
    </style>
</head>
<body>
    <h1>Mi Band Heart Rate</h1>
    <div class="label">Latest heart rate:</div>
    <div id="rate" class="value">--</div>
    <div id="status" class="label" style="color: #d32f2f; font-weight: bold; margin: 1rem 0;"></div>
    <div id="sensor-contact-container" style="display: none;">
        <div class="label">Sensor contact:</div>
        <div id="contact">--</div>
    </div>
    <button id="show-settings-btn" onclick="showSettings()">Settings</button>
    <div id="settings-panel" style="display: none; margin-top: 1rem; border: 1px solid #ccc; padding: 1rem; border-radius: 8px; max-width: 560px;">
        <div>
            <button id="toggle-contact-btn" onclick="toggleSensorContact()">Show Sensor Contact</button>
        </div>

        <h2 style="margin-top: 1.5rem;">Custom CSS</h2>
        <textarea id="custom-css" rows="10" cols="50" placeholder="Enter your custom CSS here..."></textarea><br>
        <button onclick="applyCSS()">Apply CSS</button>
        <button onclick="hideSettings()">Close Settings</button>
    </div>

    <script>
        async function fetchRate() {
            try {
                const res = await fetch('/heart-rate');
                const data = await res.json();
                
                if (data.scanning) {
                    document.getElementById('rate').textContent = '--';
                    document.getElementById('status').textContent = '🔍 正在重新扫描设备...';
                    document.getElementById('status').style.color = '#1976d2';
                } else if (!data.connected) {
                    document.getElementById('rate').textContent = '--';
                    document.getElementById('status').textContent = '⚠ 蓝牙已断开连接';
                    document.getElementById('status').style.color = '#d32f2f';
                } else {
                    document.getElementById('rate').textContent = data.heart_rate;
                    document.getElementById('status').textContent = '✓ 已连接';
                    document.getElementById('status').style.color = '#388e3c';
                }
                
                document.getElementById('contact').textContent = data.sensor_contact === null ? 'unknown' : data.sensor_contact;
            } catch (err) {
                document.getElementById('rate').textContent = '--';
                document.getElementById('status').textContent = '✗ 网络错误';
                document.getElementById('status').style.color = '#d32f2f';
                document.getElementById('contact').textContent = 'error';
            }
        }
        setInterval(fetchRate, 1000);
        fetchRate();

        function applyCSS() {
            const css = document.getElementById('custom-css').value;
            let style = document.getElementById('custom-style');
            if (!style) {
                style = document.createElement('style');
                style.id = 'custom-style';
                document.head.appendChild(style);
            }
            style.textContent = css;
            localStorage.setItem('customCSS', css);
        }

        function setSensorContactVisibility(visible) {
            const container = document.getElementById('sensor-contact-container');
            const button = document.getElementById('toggle-contact-btn');
            container.style.display = visible ? 'block' : 'none';
            button.textContent = visible ? 'Hide Sensor Contact' : 'Show Sensor Contact';
            localStorage.setItem('showSensorContact', visible ? '1' : '0');
        }

        function toggleSensorContact() {
            const visible = document.getElementById('sensor-contact-container').style.display !== 'block';
            setSensorContactVisibility(visible);
        }

        function showSettings() {
            document.getElementById('settings-panel').style.display = 'block';
            document.getElementById('show-settings-btn').style.display = 'none';
        }

        function hideSettings() {
            document.getElementById('settings-panel').style.display = 'none';
            document.getElementById('show-settings-btn').style.display = 'inline-block';
        }

        window.onload = function() {
            const showContact = localStorage.getItem('showSensorContact') === '1';
            setSensorContactVisibility(showContact);

            const css = localStorage.getItem('customCSS');
            if (css) {
                document.getElementById('custom-css').value = css;
                applyCSS();
            }

            if (css || showContact) {
                showSettings();
            }
        };
    </script>
</body>
</html>"#,
    )
}

async fn heart_rate(State(state): State<AppState>) -> Json<HeartRateReading> {
    Json(*state.rx.borrow())
}

async fn handle_device(
    adapter: &Adapter,
    device: &Device,
    tx: watch::Sender<HeartRateReading>,
) -> Result<(), Box<dyn Error>> {
    print!("Attempting to connect to device: {}\n", device.id());
    std::io::stdout().flush().unwrap();
    
    // Connect
    if !device.is_connected().await {
        print!("Connecting device: {}\n", device.id());
        std::io::stdout().flush().unwrap();
        adapter.connect_device(&device).await?;
        print!("Device connected successfully\n");
        std::io::stdout().flush().unwrap();
    } else {
        print!("Device already connected\n");
        std::io::stdout().flush().unwrap();
    }

    // Discover services
    print!("Discovering services...\n");
    std::io::stdout().flush().unwrap();
    let heart_rate_services = device.discover_services_with_uuid(HRS_UUID).await?;
    let heart_rate_service = heart_rate_services
        .first()
        .ok_or("Device should has one heart rate service at least")?;

    // Discover characteristics
    print!("Discovering characteristics...\n");
    std::io::stdout().flush().unwrap();
    let heart_rate_measurements = heart_rate_service
        .discover_characteristics_with_uuid(HRM_UUID)
        .await?;
    let heart_rate_measurement = heart_rate_measurements
        .first()
        .ok_or("HeartRateService should has one heart rate measurement characteristic at least")?;

    print!("Setting up notifications...\n");
    std::io::stdout().flush().unwrap();
    let mut updates = heart_rate_measurement.notify().await?;
    
    // Send connected state
    tx.send_replace(HeartRateReading {
        heart_rate: 0,
        sensor_contact: None,
        connected: true,
        scanning: false,
    });
    
    print!("Starting to receive heart rate data...\n");
    std::io::stdout().flush().unwrap();
    
    // Track the last update time for timeout detection
    let mut last_update_time = Instant::now();
    let mut first_data_received = false;
    let initial_timeout = Duration::from_secs(30);  // 首次连接超时30秒
    let normal_timeout = Duration::from_secs(5);    // 后续超时5秒
    
    loop {
        // 根据是否收到第一个数据选择超时时间
        let timeout_duration = if !first_data_received {
            initial_timeout
        } else {
            normal_timeout
        };
        
        // Use timeout to wait for next update
        match timeout(timeout_duration - last_update_time.elapsed(), updates.next()).await {
            Ok(Some(Ok(heart_rate))) => {
                // 收到第一个数据后，标记为已接收
                if !first_data_received {
                    first_data_received = true;
                    print!("\nFirst heart rate data received, switching to normal timeout mode\n");
                    std::io::stdout().flush().unwrap();
                }
                
                // Reset timeout timer on successful update
                last_update_time = Instant::now();
                
                let flag = *heart_rate.get(0).ok_or("No flag")?;

                // Heart Rate Value Format
                let mut heart_rate_value = *heart_rate.get(1).ok_or("No heart rate u8")? as u16;
                if flag & 0b00001 != 0 {
                    heart_rate_value |= (*heart_rate.get(2).ok_or("No heart rate u16")? as u16) << 8;
                }

                // Sensor Contact Supported
                let mut sensor_contact = None;
                if flag & 0b00100 != 0 {
                    sensor_contact = Some(flag & 0b00010 != 0)
                }

                tx.send_replace(HeartRateReading {
                    heart_rate: heart_rate_value,
                    sensor_contact,
                    connected: true,
                    scanning: false,
                });

                print!("\rHeartRateValue: {heart_rate_value}, SensorContactDetected: {sensor_contact:?}                    ");
                std::io::stdout().flush().unwrap();
            }
            Ok(Some(Err(e))) => {
                // Notification error
                print!("\nNotification error: {:?}\n", e);
                std::io::stdout().flush().unwrap();
                break;
            }
            Ok(None) => {
                // Stream ended
                print!("\nHeart rate notifications stopped\n");
                std::io::stdout().flush().unwrap();
                break;
            }
            Err(_) => {
                // Timeout - no data received within timeout period
                print!("\nNo heart rate data received for {} seconds, attempting to reconnect...\n", timeout_duration.as_secs());
                std::io::stdout().flush().unwrap();
                break;
            }
        }
    }
    
    // 检查设备连接状态
    let is_connected = device.is_connected().await;
    
    if !is_connected {
        print!("Device disconnected, attempting to reconnect...\n");
        std::io::stdout().flush().unwrap();
        
        // 设备断开连接，更新状态并返回错误以触发重连
        tx.send_replace(HeartRateReading {
            heart_rate: 0,
            sensor_contact: None,
            connected: false,
            scanning: false,
        });
        
        // 断开设备连接以确保状态干净
        let _ = adapter.disconnect_device(&device).await;
        Err("Device disconnected, attempting to reconnect...".into())
    } else {
        // 设备仍连接但停止发送通知（停止广播）
        print!("Device stopped broadcasting, attempting to reconnect...\n");
        std::io::stdout().flush().unwrap();
        
        // 更新状态并返回错误以触发重连
        tx.send_replace(HeartRateReading {
            heart_rate: 0,
            sensor_contact: None,
            connected: false,
            scanning: false,
        });
        
        // 断开设备连接以确保状态干净
        let _ = adapter.disconnect_device(&device).await;
        Err("Device stopped broadcasting, attempting to reconnect...".into())
    }
}

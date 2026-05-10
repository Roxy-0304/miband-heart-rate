use std::error::Error;
use std::io::Write;

use axum::{extract::State, response::Html, routing::get, Json, Router};
use bluest::{btuuid::bluetooth_uuid_from_u16, Adapter, Device, Uuid};
use futures_lite::stream::StreamExt;
use serde::Serialize;
use tokio::sync::watch;
use tokio::signal;

const HRS_UUID: Uuid = bluetooth_uuid_from_u16(0x180D);
const HRM_UUID: Uuid = bluetooth_uuid_from_u16(0x2A37);

#[derive(Clone, Copy, Serialize)]
struct HeartRateReading {
    heart_rate: u16,
    sensor_contact: Option<bool>,
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
    loop {
        let device = {
            let connected_heart_rate_devices =
                adapter.connected_devices_with_services(&[HRS_UUID]).await?;
            if let Some(device) = connected_heart_rate_devices.into_iter().next() {
                device
            } else {
                print!("Starting scan\n");
                std::io::stdout().flush().unwrap();
                let mut scan = adapter.discover_devices(&[HRS_UUID]).await?;
                print!("Scan started\n");
                std::io::stdout().flush().unwrap();
                let device = scan.next().await.unwrap()?;
                print!("Found Device: [{}] {:?}\n", device, device.name_async().await);
                std::io::stdout().flush().unwrap();
                device
            }
        };

        if let Err(err) = handle_device(&adapter, &device, tx.clone()).await {
            eprint!("\rConnection error: {err:?}                                                   ");
            std::io::stderr().flush().unwrap();
        }
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
                document.getElementById('rate').textContent = data.heart_rate;
                document.getElementById('contact').textContent = data.sensor_contact === null ? 'unknown' : data.sensor_contact;
            } catch (err) {
                document.getElementById('rate').textContent = '--';
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
    // Connect
    if !device.is_connected().await {
        print!("Connecting device: {}\n", device.id());
        std::io::stdout().flush().unwrap();
        adapter.connect_device(&device).await?;
    }

    // Discover services
    let heart_rate_services = device.discover_services_with_uuid(HRS_UUID).await?;
    let heart_rate_service = heart_rate_services
        .first()
        .ok_or("Device should has one heart rate service at least")?;

    // Discover
    let heart_rate_measurements = heart_rate_service
        .discover_characteristics_with_uuid(HRM_UUID)
        .await?;
    let heart_rate_measurement = heart_rate_measurements
        .first()
        .ok_or("HeartRateService should has one heart rate measurement characteristic at least")?;

    let mut updates = heart_rate_measurement.notify().await?;
    while let Some(Ok(heart_rate)) = updates.next().await {
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
        });

        print!(
            "\rHeartRateValue: {heart_rate_value}, SensorContactDetected: {sensor_contact:?}                    "
        );
        std::io::stdout().flush().unwrap();
    }
    // Disconnect the device to ensure clean state for reconnection
    let _ = adapter.disconnect_device(&device).await;
    Err("No longer heart rate notify".into())
}

/// <reference types="@tauri-apps/api" />
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

// ===== Type Definitions =====

interface HeartRatePayload {
  heart_rate: number;
  sensor_contact: boolean | null;
  connected: boolean;
  scanning: boolean;
}

type ZoneName = "warmup" | "fatburn" | "aerobic" | "limit";

// ===== Constants =====

/** Maximum heart rate reference (220 - 30 default age) */
const MAX_HR = 190;
/** Heart rate zones [min, max) as percentage of MAX_HR */
const ZONES: { name: ZoneName; label: string; min: number; max: number }[] = [
  { name: "warmup", label: "热身", min: 0, max: 0.60 },
  { name: "fatburn", label: "燃脂", min: 0.60, max: 0.70 },
  { name: "aerobic", label: "有氧", min: 0.70, max: 0.80 },
  { name: "limit", label: "极限", min: 0.80, max: 1.00 },
];

const ZONE_COLORS: Record<ZoneName, string> = {
  warmup: "#4FC3F7",
  fatburn: "#66BB6A",
  aerobic: "#FFA726",
  limit: "#EF5350",
};

// ===== DOM References =====

const statusDot = document.getElementById("status-dot")!;
const statusText = document.getElementById("status-text")!;
const heartIcon = document.getElementById("heart-icon")!;
const bpmNumber = document.getElementById("bpm-number")!;
const zoneBadge = document.getElementById("zone-badge")!;
const statMin = document.getElementById("stat-min")!;
const statMax = document.getElementById("stat-max")!;
const statAvg = document.getElementById("stat-avg")!;
const btnReset = document.getElementById("btn-reset") as HTMLButtonElement;

// ===== State =====

// Stats (always recording — computed on the fly)
let statsMin = 0;
let statsMax = 0;
let statsSum = 0;
let statsCount = 0;

// Last known good BPM (for zone display)
let latestBpm = 0;
let isConnected = false;
let isScanning = false;

// ===== Heart Rate Zone Logic =====

function getZone(bpm: number): ZoneName | null {
  if (bpm <= 0) return null;
  const pct = bpm / MAX_HR;
  for (const z of ZONES) {
    if (pct >= z.min && pct < z.max) return z.name;
  }
  return "limit";
}

function getZoneLabel(zone: ZoneName | null): string {
  if (!zone) return "--";
  return ZONES.find(z => z.name === zone)?.label ?? "--";
}

// ===== Update UI =====

function updateStatus(connected: boolean, scanning: boolean) {
  isConnected = connected;
  isScanning = scanning;

  statusDot.className = "status-dot";
  statusText.className = "status-text";

  if (connected) {
    statusDot.classList.add("connected");
    statusText.classList.add("connected");
    statusText.textContent = "已连接";
    heartIcon.classList.remove("paused");
  } else if (scanning) {
    statusDot.classList.add("scanning");
    statusText.classList.add("scanning");
    statusText.textContent = "扫描中...";
    heartIcon.classList.add("paused");
  } else {
    statusDot.classList.add("disconnected");
    statusText.classList.add("disconnected");
    statusText.textContent = "断开连接";
    heartIcon.classList.add("paused");
  }
}

function updateHeartRate(bpm: number) {
  if (bpm <= 0) {
    bpmNumber.textContent = "--";
    bpmNumber.className = "bpm-number no-data";
    zoneBadge.className = "zone-badge no-data";
    zoneBadge.textContent = "--";
    heartIcon.classList.add("paused");
    return;
  }

  bpmNumber.textContent = String(bpm);
  heartIcon.classList.remove("paused");

  const zone = getZone(bpm);
  if (zone) {
    bpmNumber.className = `bpm-number zone-${zone}`;
    zoneBadge.className = `zone-badge zone-${zone}`;
    zoneBadge.textContent = getZoneLabel(zone);
  }
}

function updateStats(value: number) {
  // First data point
  if (statsCount === 0) {
    statsMin = value;
    statsMax = value;
    statsSum = value;
    statsCount = 1;
  } else {
    if (value < statsMin) statsMin = value;
    if (value > statsMax) statsMax = value;
    statsSum += value;
    statsCount++;
  }

  const avg = Math.round(statsSum / statsCount);

  statMin.textContent = String(statsMin);
  statMin.className = "stat-value min";
  statMax.textContent = String(statsMax);
  statMax.className = "stat-value max";
  statAvg.textContent = String(avg);
  statAvg.className = "stat-value avg";
}

function resetStats() {
  statsMin = 0;
  statsMax = 0;
  statsSum = 0;
  statsCount = 0;

  statMin.textContent = "--";
  statMin.className = "stat-value no-data min";
  statMax.textContent = "--";
  statMax.className = "stat-value no-data max";
  statAvg.textContent = "--";
  statAvg.className = "stat-value no-data avg";

  // Also reset the display (but keep connection state)
  latestBpm = 0;
  bpmNumber.textContent = "--";
  bpmNumber.className = "bpm-number no-data";
  zoneBadge.className = "zone-badge no-data";
  zoneBadge.textContent = "--";
  heartIcon.classList.add("paused");
}

function handleData(bpm: number) {
  if (bpm <= 0) return;

  latestBpm = bpm;

  updateHeartRate(bpm);
  updateStats(bpm);
}

// ===== Tauri Event Listener =====

/** Catch-up: fetch the latest reading from the Rust side and apply to UI */
async function catchUp() {
  try {
    const data = await invoke<HeartRatePayload>("get_latest_reading");
    updateStatus(data.connected, data.scanning);
    if (data.connected && data.heart_rate > 0) {
      handleData(data.heart_rate);
    }
  } catch {
    // ignore — app might not be ready
  }
}

async function init() {
  // Listen for heart rate updates from Rust backend
  await listen<HeartRatePayload>("hr-update", (event) => {
    const data = event.payload;

    // Update connection status
    updateStatus(data.connected, data.scanning);

    // Update heart rate display
    if (data.connected && data.heart_rate > 0) {
      handleData(data.heart_rate);
    } else if (!data.connected && !data.scanning) {
      // Disconnected — reset display but keep stats
      if (latestBpm !== 0) {
        bpmNumber.textContent = "--";
        bpmNumber.className = "bpm-number no-data";
        zoneBadge.className = "zone-badge no-data";
        zoneBadge.textContent = "--";
        latestBpm = 0;
      }
    }
  });

  // ===== Visibility: when window becomes visible, catch up on missed data =====
  document.addEventListener("visibilitychange", () => {
    if (document.visibilityState === "visible") {
      catchUp();
    }
  });

  // Also catch up on initial load (window starts visible)
  catchUp();

  // ===== Button Handlers =====

  btnReset.addEventListener("click", () => {
    resetStats();
  });
}

// ===== Entry Point =====

document.addEventListener("DOMContentLoaded", () => {
  init().catch(console.error);
});

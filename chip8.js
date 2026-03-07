import init, { WasmChip8 } from "./pkg/chip8_emu.js";

const KB_MAP = {
  1: 0x1,
  2: 0x2,
  3: 0x3,
  4: 0xc,
  q: 0x4,
  w: 0x5,
  e: 0x6,
  r: 0xd,
  a: 0x7,
  s: 0x8,
  d: 0x9,
  f: 0xe,
  z: 0xa,
  x: 0x0,
  c: 0xb,
  v: 0xf,
};

const W = 64,
  H = 32;
const TIMER_MS = 16; // ~60Hz

const canvas = document.getElementById("screen");
const ctx = canvas.getContext("2d");
const statusEl = document.getElementById("status");
const statusDot = document.getElementById("status-dot");
const romNameEl = document.getElementById("rom-name");
const cpfSlider = document.getElementById("cpf-slider");
const cpfVal = document.getElementById("cpf-val");
const logEl = document.getElementById("log");

// State
let chip8 = null;
let romBytes = null;
let running = false;
let rafId = null;
let cpf = 15;
let lastTimer = 0;

// Default pixel colors
let COLOR_ON = [0x39, 0xff, 0x14];
let COLOR_OFF = [0x00, 0x00, 0x00];

function log(msg, type = "") {
  const ts = performance.now().toFixed(0).padStart(7, "0");
  const entry = document.createElement("div");
  entry.className = "entry";
  entry.innerHTML = `<span class="ts">${ts}</span><span class="msg ${type}">${msg}</span>`;
  logEl.appendChild(entry);
  logEl.scrollTop = logEl.scrollHeight;
}

const imgData = ctx.createImageData(W, H);

function render(display) {
  const d = imgData.data;
  for (let i = 0; i < W * H; i++) {
    const on = display[i];
    d[i * 4] = on ? COLOR_ON[0] : COLOR_OFF[0];
    d[i * 4 + 1] = on ? COLOR_ON[1] : COLOR_OFF[1];
    d[i * 4 + 2] = on ? COLOR_ON[2] : COLOR_OFF[2];
    d[i * 4 + 3] = 255;
  }
  ctx.putImageData(imgData, 0, 0);
}

let fpsFrames = 0,
  fpsLast = 0;
const fpsEl = document.getElementById("fps-display");

function frame(ts) {
  if (!running) return;

  for (let i = 0; i < cpf; i++) {
    const display = chip8.emulate_cycle();
    if (chip8.draw_flag()) {
      render(display);
      chip8.clear_draw_flag();
    }
  }

  if (ts - lastTimer >= TIMER_MS) {
    chip8.update_timers();
    lastTimer = ts;
  }

  fpsFrames++;
  if (ts - fpsLast >= 1000) {
    fpsEl.textContent = `${fpsFrames} FPS`;
    fpsFrames = 0;
    fpsLast = ts;
  }

  rafId = requestAnimationFrame(frame);
}

function setStatus(s) {
  statusEl.textContent = s;
  statusDot.classList.toggle("running", s === "RUNNING");
}

function startEmulation() {
  if (!chip8 || running) return;
  running = true;
  lastTimer = performance.now();
  setStatus("RUNNING");
  log("Emulation started", "ok");
  rafId = requestAnimationFrame(frame);
  document.getElementById("btn-run").disabled = true;
  document.getElementById("btn-pause").disabled = false;
  document.getElementById("btn-step").disabled = true;
}

function pauseEmulation() {
  if (!running) return;
  running = false;
  cancelAnimationFrame(rafId);
  setStatus("PAUSED");
  log("Paused");
  document.getElementById("btn-run").disabled = false;
  document.getElementById("btn-pause").disabled = true;
  document.getElementById("btn-step").disabled = false;
}

function resetEmulation() {
  pauseEmulation();
  if (chip8) {
    chip8.reset();
    ctx.clearRect(0, 0, W, H);
    setStatus("READY");
    log("Reset", "ok");
  }
}

function stepEmulation() {
  if (running || !chip8) return;
  const display = chip8.emulate_cycle();
  chip8.update_timers();
  if (chip8.draw_flag()) {
    render(display);
    chip8.clear_draw_flag();
  }
}

function loadRom(file) {
  const reader = new FileReader();
  reader.onload = (e) => {
    try {
      romBytes = new Uint8Array(e.target.result);
      chip8 = new WasmChip8();
      chip8.load_rom(romBytes);
      romNameEl.textContent = `► ${file.name} (${romBytes.length}b)`;
      ["btn-run", "btn-step", "btn-reset"].forEach(
        (id) => (document.getElementById(id).disabled = false),
      );
      setStatus("READY");
      ctx.clearRect(0, 0, W, H);
      log(`Loaded: ${file.name}`, "ok");
    } catch (err) {
      log(`Error: ${err.message}`, "err");
    }
  };
  reader.onerror = () => log("Failed to read file", "err");
  reader.readAsArrayBuffer(file);
}

const keyEls = {};

// Build keypad UI
const KEYPAD_LAYOUT = [
  [0x1, "1"],
  [0x2, "2"],
  [0x3, "3"],
  [0xc, "4"],
  [0x4, "Q"],
  [0x5, "W"],
  [0x6, "E"],
  [0xd, "R"],
  [0x7, "A"],
  [0x8, "S"],
  [0x9, "D"],
  [0xe, "F"],
  [0xa, "Z"],
  [0x0, "X"],
  [0xb, "C"],
  [0xf, "V"],
];
const keypadEl = document.getElementById("keypad");
KEYPAD_LAYOUT.forEach(([hex, pc]) => {
  const el = document.createElement("div");
  el.className = "key";
  el.innerHTML = `${hex.toString(16).toUpperCase()}<span class="key-map">${pc}</span>`;
  keypadEl.appendChild(el);
  keyEls[hex] = el;
});

document.addEventListener("keydown", (e) => {
  const k = KB_MAP[e.key.toLowerCase()];
  if (k !== undefined) {
    e.preventDefault();
    if (chip8) chip8.key_down(k);
    if (keyEls[k]) keyEls[k].classList.add("active");
  }
});
document.addEventListener("keyup", (e) => {
  const k = KB_MAP[e.key.toLowerCase()];
  if (k !== undefined) {
    if (chip8) chip8.key_up(k);
    if (keyEls[k]) keyEls[k].classList.remove("active");
  }
});

document.getElementById("btn-run").addEventListener("click", startEmulation);
document.getElementById("btn-pause").addEventListener("click", pauseEmulation);
document.getElementById("btn-step").addEventListener("click", stepEmulation);
document.getElementById("btn-reset").addEventListener("click", resetEmulation);

cpfSlider.addEventListener("input", () => {
  cpf = parseInt(cpfSlider.value);
  cpfVal.textContent = cpf;
});

function hexToRgb(hex) {
  const n = parseInt(hex.slice(1), 16);
  return [(n >> 16) & 0xff, (n >> 8) & 0xff, n & 0xff];
}
document.getElementById("color-on").addEventListener("input", (e) => {
  COLOR_ON = hexToRgb(e.target.value);
});
document.getElementById("color-off").addEventListener("input", (e) => {
  COLOR_OFF = hexToRgb(e.target.value);
});

const dropZone = document.getElementById("drop-zone");
const fileInput = document.getElementById("file-input");

dropZone.addEventListener("click", () => fileInput.click());
fileInput.addEventListener("change", (e) => {
  if (e.target.files[0]) loadRom(e.target.files[0]);
});
dropZone.addEventListener("dragover", (e) => {
  e.preventDefault();
  e.dataTransfer.dropEffect = "copy";
  dropZone.classList.add("drag");
});
dropZone.addEventListener("dragleave", (e) => {
  if (!dropZone.contains(e.relatedTarget)) {
    dropZone.classList.remove("drag");
  }
});
dropZone.addEventListener("drop", (e) => {
  e.preventDefault();
  dropZone.classList.remove("drag");
  const file = e.dataTransfer.files[0];
  if (file) loadRom(file);
});

await init();
setStatus("AWAITING ROM");
log("WASM initialised", "ok");

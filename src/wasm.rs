use crate::Chip8;
use wasm_bindgen::prelude::*;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

#[wasm_bindgen]
pub struct WasmChip8 {
    inner: Chip8,
}

#[wasm_bindgen]
impl WasmChip8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut chip8 = Chip8::new();
        chip8.load_fontset(FONTSET.to_vec());
        WasmChip8 { inner: chip8 }
    }

    // Load ROM from raw bytes (JS passes a Uint8Array)
    pub fn load_rom(&mut self, rom: &[u8]) {
        let end = 0x200 + rom.len();
        self.inner.memory[0x200..end].copy_from_slice(rom);
    }

    // Returns owned Vec<u8> — wasm-bindgen can cross the boundary with this
    pub fn emulate_cycle(&mut self) -> Vec<u8> {
        self.inner.emulate_cycle().to_vec()
    }

    pub fn update_timers(&mut self) {
        self.inner.update_timers();
    }

    pub fn draw_flag(&self) -> bool {
        self.inner.draw_flag
    }

    pub fn clear_draw_flag(&mut self) {
        self.inner.draw_flag = false;
    }

    pub fn key_down(&mut self, key: u8) {
        if (key as usize) < 16 {
            self.inner.keypad[key as usize] = 1;
        }
    }

    pub fn key_up(&mut self, key: u8) {
        if (key as usize) < 16 {
            self.inner.keypad[key as usize] = 0;
        }
    }

    pub fn reset(&mut self) {
        self.inner = Chip8::new();
        self.inner.load_fontset(FONTSET.to_vec());
    }
}

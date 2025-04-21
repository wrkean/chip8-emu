use std::{fs::File, io::{self, Read, Result}};

const FONTSET_START_ADDR: usize = 0x50;
const PROGRAM_START_ADDR: usize = 0x200;
const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

#[allow(non_snake_case)]
pub struct Chip8 {
    stack: Vec<u16>,
    PC: u16,
    V: [u8; 16],
    memory: [u8; 4096],
    I: u16,
    delay_timer: u8,
    sound_timer: u8,

    // Public members to make them accessible later
    // in the main function
    pub keypad: [u8; 16],
    pub display: [u8; CHIP8_WIDTH * CHIP8_HEIGHT],
    pub draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            stack: Vec::new(),
            PC: 0x200,
            V: [0; 16],
            memory: [0; 4096],
            I: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            display: [0; CHIP8_WIDTH * CHIP8_HEIGHT],
            draw_flag: false,
        }
    }

    fn load_fontset(&mut self, fontset: Vec<u8>) {
        for (i, &byte) in fontset.iter().enumerate() {
            self.memory[FONTSET_START_ADDR + i] = byte;
        }
    }

    // ROM Loader
    fn load_rom(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;

        // Store raw data from the ROM to a 
        // temporary buffer for error handling
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        let end = PROGRAM_START_ADDR + buf.len();        // Would be the index of the last

        // Returns an error if the index goes beyond
        // bounds
        if end > self.memory.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "ROM too large to fit in memory"));
        }

        // The ROM is ok, store it to memory starting from
        // 0x200 up to end
        self.memory[PROGRAM_START_ADDR..end].copy_from_slice(&buf);

        Ok(())
    }

    pub fn init(&mut self, path: &str, fontset: Vec<u8>) -> Result<()> {
        self.load_rom(path)?;
        self.load_fontset(fontset);

        Ok(())
    }

    // Emulates the chip8 cycle.
    // Fetch -> Decode -> Execute
    #[allow(non_snake_case)]
    pub fn emulate_cycle(&mut self) {
        /* Fetch opcode from memory.
            Opcode is from memory[PC] to memory[PC + 1] as a u16
            Combines memory[PC] and memory[PC + 1] by first casting
            memory[PC] to u16 (previously u8) to make space when we
            shift bits to the left. Then we shift the bits 8 times to
            the left. The right half of the opcode is now zeroed out.
            Then we set the right half with memory[PC + 1].
        */
        let opcode = ((self.memory[self.PC as usize] as u16) << 8) | (self.memory[(self.PC + 1) as usize] as u16);

        // The X and Y from the opcode is always at the second
        // and third nibble of the opcode. We can assign it to a
        // variable. They are casted as usize because they are often
        // used in indexing with the V registers.
        let X = ((opcode & 0x0F00) >> 8) as usize;
        let Y = ((opcode & 0x00F0) >> 4) as usize;

        // NN and NNN are also in a consistent position in the
        // given opcode, so I made a variable.
        let NN = opcode & 0x00FF;
        let NNN = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x00FF {
                    0x00E0 => {
                        // 00E0: Clears the display
                        self.display = [0; CHIP8_WIDTH * CHIP8_HEIGHT];
                        self.PC += 2;
                    }
                    0x00EE => {
                        // 00EE: Returns from a subroutine
                        self.PC = self.stack.pop().unwrap();
                        self.PC += 2;
                    }
                    _ => {
                        eprintln!("Invalid opcode: {:#X}", opcode);
                    }
                }
            }
            0x1000 => {
                // 1NNN: Jumps to location NNN
                self.PC = NNN;
            }
            0x2000 => {
                // 2NNN: Calls subroutine from NNN
                self.stack.push(self.PC);
                self.PC = NNN;
            }
            0x3000 => {
                // 3XNN: Skips next instruction if
                // V[X] == NN
                if (self.V[X] as u16) == NN {
                    self.PC += 4;               // Skips next instruction
                } else {
                    self.PC += 2;
                }
            }
            0x4000 => {
                // 4XNN: Skips next instruction if
                // V[X] != NN
                if (self.V[X] as u16) != NN {
                    self.PC += 4;               // Skips next instruction
                } else {
                    self.PC += 2;
                }
            }
            0x5000 => {
                // 5XNN: Skips next instruction if V[X] == V[Y]
                if self.V[X] == self.V[Y] {
                    self.PC += 4;
                } else {
                    self.PC += 2;
                }
            }
            0x6000 => {
                // 6XNN: Sets V[X] to NN
                self.V[X] = NN as u8;
                self.PC += 2;
            }
            0x7000 => {
                // 7XNN: Adds NN to V[X]
                self.V[X] = self.V[X].wrapping_add(NN as u8);
                self.PC += 2;
            }
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        // 8XY0: Sets V[X] to V[Y]
                        self.V[X] = self.V[Y];
                        self.PC += 2;
                    }
                    0x0001 => {
                        // 8XY1: OR V[X] and V[Y] and
                        // store the result to V[X]
                        self.V[X] |= self.V[Y];
                        self.PC += 2;
                    }
                    0x0002 => {
                        // 8XY2: AND V[X] and V[Y] and
                        // store the result to V[X]
                        self.V[X] &= self.V[Y];
                        self.PC += 2;
                    }
                    0x0003 => {
                        // 8XY3: XOR V[X] and V[Y] and
                        // store the result to V[X]
                        self.V[X] ^= self.V[Y];
                        self.PC += 2;
                    }
                    0x0004 => {
                        // 8XY4: Add with carry
                        // if the sum is greater than 255 (size of u8),
                        // then we set the carry flag to V[0xF] (V[15])
                        let (sum, carry) = self.V[X].overflowing_add(self.V[Y]);
                        self.V[0xF] = if carry { 1 } else { 0 };
                        self.V[X] = sum;
                        self.PC += 2;
                    }
                    0x0005 => {
                        // 8XY5: Sub with borrow
                        // if V[X] is greater than V[Y], then
                        // set V[0xF] to 1, otherwise set to 0
                        let (diff, borrow) = self.V[X].overflowing_sub(self.V[Y]);
                        self.V[0xF] = if borrow { 0 } else { 1 };           // if borrow = true, then V[X] must be lesser than V[Y]
                        self.V[X] = diff;
                        self.PC += 2;
                    }
                    0x0006 => {
                        // 8XY6: If the least significant bit
                        // of V[X] is 1, then set V[0xF] to 1,
                        // otherwise 0. Then V[X] is right-shifted once
                        self.V[0xF] = self.V[X] & 0x1;
                        self.V[X] >>= 1;
                        self.PC += 2;
                    }
                    0x0007 => {
                        // 8XY7: Sub with borrow
                        // if V[Y] is greater than V[X], then
                        // set V[0xF] to 1, otherwise set to 0
                        let (diff, borrow) = self.V[Y].overflowing_sub(self.V[X]);
                        self.V[0xF] = if borrow { 0 } else { 1 };
                        self.V[X] = diff;
                        self.PC += 2;
                    }
                    0x000E => {
                        // 8XYE: If the most significant bit
                        // of V[X] is 1, then set V[0xF] to 1,
                        // otherwise 0. Then V[X] is left-shifted once
                        self.V[0xF] = (self.V[X] & 0x80) >> 7;
                        self.V[X] <<= 1;
                        self.PC += 2;
                    }
                    _ => {
                        eprintln!("Invalid opcode: {:#X}", opcode);
                    }
                }
            }
            0x9000 => {
                // 9XY0: Skips next instruction if V[X] is not
                // equal to V[Y]
                if self.V[X] != self.V[Y] {
                    self.PC += 4;
                } else {
                    self.PC += 2;
                }
            }
            0xA000 => {
                // ANNN: Set index register I to address NNN
                self.I = NNN;
                self.PC += 2;
            }
            0xB000 => {
                // BNNN: Jump to address NNN plus V[0]
                self.PC = NNN + self.V[0] as u16;
            }
            0xC000 => {
                // CXNN: Generates a random byte (0 - 255) and ANDs
                // it to NN, V[X] is then set to the result
                let rand_byte: u8 = rand::random();
                self.V[X] = rand_byte & NN as u8;
                self.PC += 2;
            }
            0xD000 => {
                // DXYN: Draw sprite at coordinate (V[X], V[Y])
                // with N bytes from memory I
                let x = self.V[X] as usize;
                let y = self.V[Y] as usize;
                let height = (opcode & 0x000F) as usize;
                self.V[0xF] = 0;

                for row in 0..height {
                    let sprite = self.memory[self.I as usize + row];
                    for col in 0..8 {
                        let pixel = (sprite >> (7 - col)) & 1;

                        let xcord = (x + col) % CHIP8_WIDTH;
                        let ycord = (y + row) % CHIP8_HEIGHT;
                        let index = ycord * CHIP8_WIDTH + xcord;

                        if pixel == 1 {
                            if self.display[index] == 1 {
                                self.V[0xF] = 1;
                            }
                            self.display[index] ^= 1;
                        }
                    }
                }

                self.draw_flag = true;
                self.PC += 2;
            }
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => {
                        // EX9E: Skip next instruction if keypad[V[X]] is
                        // pressed
                        if self.keypad[self.V[X] as usize] == 1 {
                            self.PC += 4;
                        } else {
                            self.PC += 2;
                        }
                    }
                    0x00A1 => {
                        // EXA1: Skip next instruction if keypad[V[X]] is
                        // not pressed
                        if self.keypad[self.V[X] as usize] == 0 {
                            self.PC += 4;
                        } else {
                            self.PC += 2;
                        }
                    }
                    _ => {
                        eprintln!("Invalid opcode: {:#X}", opcode);
                    }
                }
            }
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => {
                        // FX07: Set V[X] to the delay timer
                        self.V[X] = self.delay_timer;
                        self.PC += 2;
                    }
                    0x000A => {
                        // FX0A: Stop emulator until a key
                        // is pressed
                        let mut key_pressed = false;

                        for i in 0..16 {
                            if self.keypad[i] == 1 {
                                self.V[X] = i as u8;
                                key_pressed = true;
                                break;
                            }
                        }

                        // Effectively stops emulator
                        // until a key is pressed. (PC is
                        // only added when a key is pressed)
                        if key_pressed {
                            self.PC += 2;
                        }

                    }
                    0x0015 => {
                        // FX15: Set delay timer to V[X]
                        self.delay_timer = self.V[X];
                        self.PC += 2;
                    }
                    0x0018 => {
                        // FX18: Set sound timer to V[X]
                        self.sound_timer = self.V[X];
                        self.PC += 2;
                    }
                    0x001E => {
                        // FX1E: Sets I to I + V[X]
                        self.I = self.I.wrapping_add(self.V[X] as u16);
                        self.PC += 2;
                        
                    }
                    0x0029 => {
                        // FX29: Set I to the location of
                        // sprite for digit V[X]
                        let digit = self.V[X];

                        self.I = FONTSET_START_ADDR as u16 + (digit as u16 * 5);
                        self.PC += 2;
                    }
                    0x0033 => {
                        // FX33: Store BCD (Binary-Coded Decimal) representation
                        // of V[X] to memory[I], memory[I + 1], and memory[I + 2]
                        let value = self.V[X];

                        self.memory[self.I as usize]        = value / 100;
                        self.memory[self.I as usize + 1]    = (value % 100) / 10;
                        self.memory[self.I as usize + 2]    = value % 10;

                        self.PC += 2;
                    }
                    0x0055 => {
                        // FX55: Stores V[i] to V[X] into memory[I + i]
                        for i in 0..=X {
                            self.memory[self.I as usize + i] = self.V[i];
                        }
                        self.PC += 2;
                    }
                    0x0065 => {
                        // FX65: Stores memory[I + i] into V[i] to V[X]
                        for i in 0..=X {
                            self.V[i] = self.memory[self.I as usize + i];
                        }
                        self.PC += 2;
                    }
                    _ => {
                        eprintln!("Invalid opcode: {:#X}", opcode);
                    }
                }
            }
            _ => {
                eprintln!("Invalid opcode: {:#X}", opcode);
            }
        }
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}


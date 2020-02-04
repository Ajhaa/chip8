use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_sys::Array;
use rand::prelude::*;
use rand::Rng;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Chip {
    opcode: u16,

    memory: [u8; 4096],
    V: [u8; 16],

    pub I: u16,
    pub pc: usize,

    gfx: [u64; 32],

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],
    pub sp: usize,
}

fn extract_reg_and_byte(opcode: u16) -> (usize, u8) {
    let value = opcode & 0x00FF;
    let reg = (opcode & 0x0F00) >> 8;

    return (reg as usize, value as u8);
}

#[wasm_bindgen]
impl Chip {
    pub fn new() -> Chip {
        let mut chip = Chip {
            opcode: 0,
            memory: [0; 4096],
            V: [0; 16],
            I: 0,
            pc: 0x200,
            gfx: [0; 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
        };
        chip.init_digits();
        chip
    }

    pub fn display_as_str(&self) -> String {
        let mut display_str = String::new();
        let base: u64 = 2;
        for i in 0..32 {
            for j in 0..64 {
                let pixel = self.gfx[i] & base.pow(63 - j);
                display_str.push(if pixel != 0 { 'X' } else { '=' });
            }
            display_str.push('\n');
        }
        
        display_str
    }

    pub fn load_instructions(&mut self, list: Array) {
        let mut i = 0x200;
        for inst in list.iter() {
            let number = inst.as_f64().unwrap() as u16;
            let p1 = (number & 0xFF00) >> 8;
            let p2 = number & 0x00FF;  
            self.memory[i] = p1 as u8;
            self.memory[i+1] = p2 as u8;
            i += 2;
        }
    }

    pub fn trigger_cycle(&mut self) {
        self.cycle();
    }

    pub fn mem_dump(&self, start: usize, end: usize) -> Array {
        let dump = Array::new();
        for i in start..end {
            let val = self.memory[i];
            dump.push(&JsValue::from_str(&format!("{:X}: 0x{:X} {}", i, val, val)));
        }

        dump
    }
}

impl Chip {
    pub fn get_opcode(&self) -> u16 {
        let code = self.memory[self.pc] as u16;
        code << 8 | self.memory[self.pc + 1] as u16
    }

    pub fn reg_dump(&self) {
        let mut i = 0;
        for reg in &self.V {
            //println!("V{}: {}", i, reg);
            i += 1;
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.get_opcode();
        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x000F {
                    // CLS
                    0x0000 => {
                        self.gfx = [0; 32]
                    },
                    // RET
                    0x000E => {
                        self.pc = self.stack[self.sp] as usize;
                        self.sp -= 1;
                        return;
                    },
                    _ => ()
                }
            },
            // JP addr
            0x1000 => {
                let new_addr = (opcode & 0x0FFF) as usize;
                self.pc = new_addr;
                return;
            },
            // CALL addr
            0x2000 => {
                let new_pc = (opcode & 0x0FFF) as usize;

                self.sp += 1; 

                self.stack[self.sp] = self.pc as u16;
                self.pc = new_pc;
                return;
            },
            // SE Vx, byte
            0x3000 => {
                let (reg, value) = extract_reg_and_byte(opcode);
                if self.V[reg] == value {
                    self.pc += 2;
                }
            },
            // SNE vx, byte
            0x4000 => {
                let (reg, value) = extract_reg_and_byte(opcode);
                if self.V[reg] != value {
                    self.pc += 2;
                } 
            },
            // SE vx, vy
            0x5000 => {
                let reg = ((opcode & 0x0F00) >> 8) as usize;
                let reg2 = ((opcode & 0x00F0) >> 4) as usize;
                if self.V[reg] == self.V[reg2] {
                    self.pc += 2;
                } 
            }
            // LD Vx, byte
            0x6000 => {
                let (reg, value) = extract_reg_and_byte(opcode);
                self.V[reg] = value;
            },
            //ADD Vx, byte
            0x7000 => {
                let (reg, value) = extract_reg_and_byte(opcode);
                self.V[reg] = self.V[reg] + value;
            },
            0x8000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                match opcode & 0x000F {
                    // LD Vx, Vy
                    0x0000 => {
                        self.V[x] = self.V[y];
                    },
                    // OR Vx, Vy
                    0x0001 => {
                        self.V[x] |= self.V[y];
                    },
                    // AND Vx, Vy
                    0x0002 => {
                        self.V[x] &= self.V[y];
                    },
                    // XOR Vx, Vy
                    0x0003 => {
                        self.V[x] ^= self.V[y];
                    },
                    // ADD Vx, Vy
                    0x0004 => {
                        let sum = self.V[x] + self.V[y];

                        self.V[x] = sum;

                        if sum < self.V[x] {
                            self.V[0xF] = 1;
                        } else {
                            self.V[0xF] = 0;
                        }
                    },
                    // SUB Vx, Vy
                    0x0005 => {
                        let res = self.V[x] - self.V[y];
                        self.V[x] = res;
                        if self.V[x] > self.V[y] {
                            self.V[0xF] = 1;
                        } else {
                            self.V[0xF] = 0;
                        }
                    },
                    // SHR Vx
                    0x0006 => {
                        if self.V[x] % 2 == 0 {
                            self.V[0xF] = 0;
                        } else {
                            self.V[0xF] = 1;
                        }
                        self.V[x] /= 2;
                    },
                    // SUBN Vx, Vy
                    0x0007 => {
                        let res = self.V[y] - self.V[x];
                        self.V[x] = res;
                        if self.V[y] > self.V[x] {
                            self.V[0xF] = 1;
                        } else {
                            self.V[0xF] = 0;
                        }
                    },
                    0x000E => {
                        if self.V[x] % 2 == 0 {
                            self.V[0xF] = 0;
                        } else {
                            self.V[0xF] = 1;
                        }
                        self.V[x] *= 2;
                    },
                    _ => ()
                }
            },
            0x9000 => {
                let reg = ((opcode & 0x0F00) >> 8) as usize;
                let reg2 = ((opcode & 0x00F0) >> 4) as usize;
                if self.V[reg] != self.V[reg2] {
                    self.pc += 2;
                } 
            },
            // LD i, addr
            0xA000 => {
                self.I = opcode & 0x0FFF;
            },
            // JP V0, addr
            0xB000 => {
                let mut new_pc = (opcode & 0x0FFF) as usize;
                new_pc += self.V[0] as usize;
                self.pc = new_pc;
            },
            0xC000 => {
                let (reg, value) = extract_reg_and_byte(opcode);
                let random_byte = rand::random::<u8>();
                self.V[reg] = value & random_byte;
            },
            0xD000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let n = opcode & 0x000F;

                let shift_amount = 64 - 8 - self.V[x];
                log(&format!("{}", shift_amount));
                for i in 0..n {
                    let byte = (self.memory[(self.I+i) as usize] as u64) << shift_amount;
                    //TODO flag if pixel off
                    self.gfx[(self.V[y as usize] as usize) + (i as usize)] ^= byte;
                }

            },
            0xF000 => {
                match opcode & 0x00FF {
                    0x001E => {
                        let reg = ((opcode & 0x0F00) >> 8) as usize;
                        self.I += self.V[reg] as u16;

                    },
                    0x0055 => {
                        let end = (opcode & 0x0F00) >> 8;
                        for i in 0..end+1 {
                            self.memory[(self.I+i) as usize] = self.V[i as usize];
                        } 
                    },
                    _ => ()
                }
            },
            _ => ()
        }

        self.pc += 2;
    }

    fn init_digits(&mut self) {
        // pasted from http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
        let digits = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for i in 0..80 {
            self.memory[i] = digits[i];
        }
    }
}
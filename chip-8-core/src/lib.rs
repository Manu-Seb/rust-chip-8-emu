use rand::{Rng, random};

const RAM_SIZE: u32 = 4096;
const FONTSET_SIZE: u32 = 80;

const REG_NUMBERS: u32 = 16;
const STACK_SIZE: u16 = 16;
const NUM_KEYS: u16 = 16;

const START_ADDR: u16 = 0x200;

pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;

const FONTSET: [u8; FONTSET_SIZE as usize] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emu {
    program_counter: u16,
    ram: [u8; RAM_SIZE as usize],
    screen: [bool; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
    v_reg: [u8; REG_NUMBERS as usize],
    i_reg: u16,
    stack_pointer: u16,
    stack: [u16; STACK_SIZE as usize],
    keys: [bool; NUM_KEYS as usize],
    delay_timer: u8,
    sound_timer: u8,
}

impl Emu {
    pub fn new() -> Self {
        let mut res = Emu {
            program_counter: START_ADDR,
            ram: [0; RAM_SIZE as usize],
            screen: [false; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
            v_reg: [0; REG_NUMBERS as usize],
            i_reg: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE as usize],
            keys: [false; NUM_KEYS as usize],
            delay_timer: 0,
            sound_timer: 0,
        };
        res.ram[..FONTSET_SIZE as usize].copy_from_slice(&FONTSET);
        res
    }

    pub fn reset(&mut self) {
        self.program_counter = START_ADDR;
        self.ram = [0; RAM_SIZE as usize];
        self.screen = [false; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize];
        self.i_reg = 0;
        self.v_reg = [0; REG_NUMBERS as usize];
        self.stack_pointer = 0;
        self.stack = [0; STACK_SIZE as usize];
        self.delay_timer = 0;
        self.sound_timer = 0;

        self.ram[..FONTSET_SIZE as usize].copy_from_slice(&FONTSET);
    }

    pub fn push(&mut self, val: u16) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    pub fn tick(&mut self) {
        let op = self.fetch();

        self.execute(op);
    }

    fn fetch(&mut self) -> u16 {
        let first_digit = self.ram[self.program_counter as usize] as u16;
        let second_digit = self.ram[self.program_counter as usize + 1] as u16;
        let op_code = (first_digit << 8) | second_digit;
        self.program_counter += 2;
        op_code
    }

    fn execute(&mut self, op: u16) {
        let digit1: u16 = (op & 0xF000) >> 12;
        let digit2: u16 = (op & 0x0F00) >> 8;
        let digit3: u16 = (op & 0x00F0) >> 4;
        let digit4: u16 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => self.screen = [false; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
            (0, 0, 0xE, 0xE) => {
                let return_addr = self.pop();
                self.program_counter = return_addr;
            }
            (1, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.program_counter = nnn;
            }
            (2, _, _, _) => {
                self.push(self.program_counter);
                let nnn = op & 0x0FFF;
                self.program_counter = nnn;
            }
            (3, _, _, _) => {
                let x = digit2;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x as usize] == nn {
                    self.program_counter += 2;
                }
            }
            (4, _, _, _) => {
                let x = digit2;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x as usize] != nn {
                    self.program_counter += 2;
                }
            }
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.program_counter += 2;
                }
            }
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = nn;
            }
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
            }
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            }
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y];
            }
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
            }
            //adding from register y to register x
            //Store value of the carry to the last register
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vf, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_f = if carry { 1 } else { 0 };
                self.v_reg[x] = new_vf;
                self.v_reg[0xF] = new_f;
            }
            //similiar but with sub
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vf, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_f = if borrow { 0 } else { 1 };
                self.v_reg[x] = new_vf;
                self.v_reg[0xF] = new_f;
            }
            //for bitshift
            (8, _, _, 6) => {
                let x = digit2 as usize;
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            }
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vf, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_f = if borrow { 0 } else { 1 };
                self.v_reg[x] = new_vf;
                self.v_reg[0xF] = new_f;
            }
            (8, _, _, 0xE) => {
                let x = digit2 as usize;
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            }
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] != self.v_reg[y] {
                    self.program_counter += 2;
                }
            }
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
            }
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.program_counter = (self.v_reg[0] as u16) + nnn;
            }
            (0xC, _, _, _) => {
                let x = digit2;
                let nn = (op & 0xFF) as u8;
                let mut rng = rand::rng();
                let random_value: u8 = rng.random_range(0..255);
                self.v_reg[x as usize] = random_value & nn;
            }
            //DRAW
            (0xD, _, _, _) => {
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;
                let num_rows = digit4;

                let mut flipped = false;

                for y_line in 0..num_rows {
                    let addr = (self.i_reg + y_line) as u16;
                    let pixels = self.ram[addr as usize];

                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_coord + x_line) % SCREEN_WIDTH as u16;
                            let y = (y_coord + y_line) % SCREEN_HEIGHT as u16;
                            let idx = x + SCREEN_WIDTH as u16 * y;

                            flipped |= self.screen[idx as usize];
                            self.screen[idx as usize] ^= true;
                        }
                    }
                }

                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            }
            (0xE, _, 9, 0xE) => {
                let x = digit2;
                let vx = self.v_reg[x as usize];
                if self.keys[vx as usize] {
                    self.program_counter += 2;
                }
            }
            (0xE, _, 0xA, 1) => {
                let x = digit2;
                let vx = self.v_reg[x as usize];
                if !self.keys[vx as usize] {
                    self.program_counter += 2;
                }
            }
            (0xF, _, 0, 7) => {
                let x = digit2;
                self.v_reg[x as usize] = self.delay_timer;
            }
            (0xF, _, 0, 0xA) => {
                let x = digit2;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i as usize] {
                        self.v_reg[x as usize] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.program_counter -= 2;
                }
            }
            (0xF, _, 1, 5) => {
                let x = digit2;
                self.delay_timer = self.v_reg[x as usize];
            }
            (0xF, _, 1, 8) => {
                let x = digit2;
                self.sound_timer = self.v_reg[x as usize];
            }
            (0xF, _, 1, 0xE) => {
                let x = digit2;
                let vx = self.v_reg[x as usize] as u16;

                self.i_reg = self.i_reg.wrapping_add(vx);
            }
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                let c = self.v_reg[x] as u16;
                //because we store int he begining of the ram and also because each font is 5 bits
                //long
                self.i_reg = c * 5;
            }
            (0xF, _, 3, 3) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as f32;

                let hundreths = (vx / 100.0).floor() as u8;
                let tenths = ((vx / 10.0) % 10.0).floor() as u8;
                let ones = (vx % 10.0).floor() as u8;

                self.ram[self.i_reg as usize] = hundreths;
                self.ram[(self.i_reg + 1) as usize] = tenths;
                self.ram[(self.i_reg + 2) as usize] = ones;
            }
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;
                let idx = self.i_reg as usize;
                for i in 0..=x {
                    self.ram[idx + i] = self.v_reg[i];
                }
                // self.i_reg += (x as u16) + 1;
            }
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                let idx = self.i_reg as usize;
                for i in 0..=x {
                    self.v_reg[i] = self.ram[idx + i];
                }
                // self.i_reg += (x as u16) + 1;
            }
            (_, _, _, _) => unimplemented!("DIDNT IMPL"),
        }
    }

    pub fn tick_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP");
            }
            self.sound_timer -= 1;
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start_idx = START_ADDR as usize;
        let end_idx = START_ADDR as usize + data.len();
        self.ram[start_idx..end_idx].copy_from_slice(data);
    }
}

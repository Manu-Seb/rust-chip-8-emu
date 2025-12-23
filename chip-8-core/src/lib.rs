pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

const RAM_SIZE: u32 = 4096;
const FONTSET_SIZE: u32 = 80;

const REG_NUMBERS: u32 = 16;
const STACK_SIZE: u16 = 16;

const START_ADDR: u16 = 0x200;

const SCREEN_HEIGHT: u16 = 32;
const SCREEN_WIDTH: u16 = 64;

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
    screen : [[bool; SCREEN_WIDTH as usize] ; SCREEN_HEIGHT as usize],
    v_reg: [u16; REG_NUMBERS as usize],
    i_reg: u16,
    stack_pointer: u16,
    stack: [u16; STACK_SIZE as usize],
    delay_timer: u16,
    sound_timer: u16,
}

impl Emu {
    pub fn new() -> Self {
        let mut res = Emu {
            program_counter: START_ADDR,
            ram: [0; RAM_SIZE as usize],
            screen : [[false; SCREEN_WIDTH as usize] ; SCREEN_HEIGHT as usize],
            v_reg: [0; REG_NUMBERS as usize],
            i_reg: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE as usize],
            delay_timer: 0,
            sound_timer: 0,
        };
        res.ram[..FONTSET_SIZE as usize].copy_from_slice(&FONTSET);
        res
    }

    pub fn reset(&mut self) {
        self.program_counter = START_ADDR;
        self.ram = [0; RAM_SIZE as usize];
        self.screen = [[false ; SCREEN_WIDTH as usize] ; SCREEN_HEIGHT as usize];
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
            (0, 0, 0xE, 0) => self.screen = [[false ; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize],
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
            self.delay_timer -= 1;
        }
    }
}

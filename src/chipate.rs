use std::{thread, time};

/// Chipate Module
/// Rust emulation of the Chip-8
/// http://www.multigesturearticles/how-to-write-an-emulator-chip-8-interpreter/
#[allow(dead_code)]
pub struct Chipate {
    // Opcodes
    opcode: u16,

    // 4K Memory
    memory: [u8; 4096],

    // Registers
    v: [u8; 16],

    // Index
    i: u8,

    // Program Counter
    pc: u8,

    // The systems memory map:
    // 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    // 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    // 0x200-0xFFF - Program ROM and work RAM

    gfx: [u8; 64 * 32],

    // Timers
    delay_timer: u8,
    sound_timer: u8,

    // Stack
    stack: [u8; 16],
    sp: u8,

    // Keypad
    key: [u8; 16],

    program: &'static str,
}

impl Chipate {
    pub fn init(&mut self) {

        debug!("Initialize Chip");
    }

    pub fn load_program(&mut self, program: &str) {
        debug!("Loading program {}", program);
    }

    pub fn emulate_cycle(&mut self) {
        debug!("Cycle Begin");

        self.fetch_opcode();

        let one_second = time::Duration::from_secs(1);
        thread::sleep(one_second);

        debug!("Cycle End");
    }

    pub fn draw_screen(&mut self) {
        debug!("Drawing to Screen")
    }

    pub fn set_keys(&mut self) {
        debug!("Saving Key State")
    }

    pub fn setup_testing_memory(&mut self) {
        debug!("Setting test memory");

        self.memory[self.pc as usize] = 0xA2;
        self.memory[(self.pc + 1) as usize] = 0xF0;

        debug!("location: 0x{:x} data: 0x{:x}", self.pc, self.memory[self.pc as usize]);
        debug!("location: 0x{:x} data: 0x{:x}", self.pc + 1, self.memory[(self.pc + 1) as usize]);
    }

    pub fn fetch_opcode(&mut self) {
    }
}

pub fn new_chipate() -> Chipate {
    debug!("Creating New Chip");

    let chip = Chipate {
        opcode: 0,
        memory: [0; 4096],
        v: [0; 16],
        i: 0,
        pc: 0,
        gfx: [0; 64 * 32],
        delay_timer: 0,
        sound_timer: 0,
        stack: [0; 16],
        sp: 0,
        key: [0; 16],
        program: ""
    };

    return chip;
}

/// Chipate Module
/// Rust emulation of the Chip-8
/// http://www.multigesturearticles/how-to-write-an-emulator-chip-8-interpreter/
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

        println!("Initialize Chip");
    }

    pub fn load_program(&mut self, program: &str) {
        println!("Loading program {}", program);
    }

    pub fn emulate_cycle() {
        println!("Cycle Begin");
        println!("Cycle End");
    }
}

pub fn newChipate() -> Chipate {
    println!("Creating New Chip");

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

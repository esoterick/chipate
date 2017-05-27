use std::{thread, time};
use std::env;
use std::io::prelude::*;
use std::fs::File;

/// Chipate Module
/// Rust emulation of the Chip-8
/// http://www.multigesturearticles/how-to-write-an-emulator-chip-8-interpreter/
#[allow(dead_code)]
pub struct Chipate {
    // Opcodes
    opcode: u16,

    // 4K Memory
    memory: [u8; 4096],
    // The systems memory map:
    // 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    // 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    // 0x200-0xFFF - Program ROM and work RAM

    // Registers
    v: [u8; 16],

    // Index
    i: u16,

    // Program Counter
    pc: u16,

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
        // pc     = 0x200;  // Program counter starts at 0x200
        self.pc = 0x200;
        // opcode = 0;      // Reset current opcode
        self.opcode = 0x0;
        // I      = 0;      // Reset index register
        self.i = 0x0;
        // sp     = 0;      // Reset stack pointer
        self.sp = 0x0;

        // // Clear display
        // // Clear stack
        // // Clear registers V0-VF
        // // Clear memory

        // // Load fontset
        // for(int i = 0; i < 80; ++i)
        //     memory[i] = chip8_fontset[i];

        // // Reset timers
    }

    pub fn load_program(&mut self, program: &str) {
        debug!("Loading program {}", program);

        // We assume that we are in a valid directory.
        let mut p = env::current_dir().unwrap();
        p.push("programs");
        p.push(program);

        let mut f = File::open(p).expect("Unable to open file");
        let mut b = Vec::new();

        f.read_to_end(&mut b).expect("Unable to read file");

        for i in 0..b.len() {
            self.memory[i + 512] = b[i];
        }

    }

    pub fn emulate_cycle(&mut self) {
        debug!("Cycle Begin");

        self.fetch_opcode();
        self.decode_opcode();

        let one_second = time::Duration::from_millis(1000);
        thread::sleep(one_second);

        debug!("Cycle End");
    }

    pub fn draw_screen(&mut self) {
        debug!("Drawing to Screen")
    }

    pub fn set_keys(&mut self) {
        debug!("Saving Key State")
    }

    // pub fn setup_testing_memory(&mut self) {
    //     debug!("Setting test memory");

    //     self.memory[self.pc as usize] = 0xA2;
    //     self.memory[(self.pc + 1) as usize] = 0xF0;

    //     debug!("Location: 0x{:X} data: 0x{:X}",
    //            self.pc,
    //            self.memory[self.pc as usize]);
    //     debug!("Location: 0x{:X} data: 0x{:X}",
    //            self.pc + 1,
    //            self.memory[(self.pc + 1) as usize]);
    // }

    pub fn fetch_opcode(&mut self) {
        let op_a = self.memory[self.pc as usize];
        self.opcode = op_a as u16;
        self.opcode = self.opcode << 8;
        // debug!("location: 0x{:X} data: 0x{:X}", self.pc, self.opcode);

        let op_b = self.memory[(self.pc + 1) as usize] as u16;
        // debug!("location: 0x{:X} data: 0x{:X}", self.pc, op_b);

        self.opcode = self.opcode | op_b;
        debug!("Opcode: 0x{:X}", self.opcode);
    }

    pub fn decode_opcode(&mut self) {
        let op = self.opcode & 0xF000;
        debug!("Decode: 0x{:X}", op);

        match op {
            0x2000 => self._2nnn_opcode(),
            0x6000 => self._6xnn_opcode(),
            0x7000 => self._7xnn_opcode(),
            0xA000 => self._annn_opcode(),
            0xF000 => self._f_opcodes(),
            _ => {
                // Using the catch all as a NOOP
                info!("Catch all: 0x{:X}", self.opcode);
                self.increase_pc();
            }
        }
    }

    pub fn increase_pc(&mut self) {
        self.pc += 2;
        debug!("Program Counter: 0x{:X}", self.pc);
    }

    /// 2NNN 	Flow 	*(0xNNN)() 	Calls subroutine at NNN.
    pub fn _2nnn_opcode(&mut self) {
        info!("2NNN: 0x{:X}", self.opcode);
        self.stack[self.sp as usize];
        self.sp += 1;
        self.pc = self.opcode & 0x0FFF;
        self.increase_pc();
    }

    /// 6XNN 	Const 	Vx = NN 	Sets VX to NN.
    pub fn _6xnn_opcode(&mut self) {
        info!("6XNN: 0x{:X}", self.opcode);
        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 12;
        let data = self.opcode & 0x00FF;
        self.v[reg as usize] = data as u8;
        self.increase_pc();
    }

    /// 7XNN 	Const 	Vx += NN 	Adds NN to VX.
    pub fn _7xnn_opcode(&mut self) {
        info!("7XNN: 0x{:X}", self.opcode);
        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 12;
        self.v[reg as usize] = (self.opcode & 0x00FF) as u8 + self.v[reg as usize] as u8;
        self.increase_pc();
    }

    /// ANNN 	MEM 	I = NNN 	Sets I to the address NNN.
    pub fn _annn_opcode(&mut self) {
        info!("ANNN: 0x{:X}", self.opcode);
        self.i = self.opcode & 0x0FFF;
        self.increase_pc();
    }

    /// FX07 	Timer 	Vx = get_delay() 	Sets VX to the value of the delay timer.
    pub fn _fx07_opcode(&mut self) {
        info!("FX07: 0x{:X}", self.opcode);

        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 12;
        self.v[reg as usize] = self.delay_timer;

        self.increase_pc();
    }

    /// FX0A 	KeyOp 	Vx = get_key() 	A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)
    pub fn _fx0a_opcode(&mut self) {
        info!("FX0A: 0x{:X}", self.opcode);
        self.increase_pc();
    }

    /// FX15 	Timer 	delay_timer(Vx) 	Sets the delay timer to VX.
    pub fn _fx15_opcode(&mut self) {
        info!("FX15: 0x{:X}", self.opcode);

        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 12;
        self.delay_timer = self.v[reg as usize];

        self.increase_pc();
    }

    /// FX18 	Sound 	sound_timer(Vx) 	Sets the sound timer to VX.
    pub fn _fx18_opcode(&mut self) {
        info!("FX18: 0x{:X}", self.opcode);

        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 12;
        self.sound_timer = self.v[reg as usize];

        self.increase_pc();
    }

    /// FX1E 	MEM 	I +=Vx 	Adds VX to I.[3]
    pub fn _fx1e_opcode(&mut self) {
        info!("FX1E: 0x{:X}", self.opcode);
        self.increase_pc();
    }

    /// FX29 	MEM 	I=sprite_addr[Vx] 	Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    pub fn _fx29_opcode(&mut self) {
        info!("FX29: 0x{:X}", self.opcode);
        self.increase_pc();
    }

    /// FX33 	BCD 	....  Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
    pub fn _fx33_opcode(&mut self) {
        info!("FX33: 0x{:X}", self.opcode);

        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 12;
        let v = self.v[reg as usize];

        debug!("FX33: {}", v);
        self.increase_pc();
    }

    /// FX55 	MEM 	reg_dump(Vx,&I) 	Stores V0 to VX (including VX) in memory starting at address I.[4]
    pub fn _fx55_opcode(&mut self) {
        info!("FX55: 0x{:X}", self.opcode);
        self.increase_pc();
    }

    /// FX65 	MEM 	reg_load(Vx,&I) 	Fills V0 to VX (including VX) with values from memory starting at address I.[4]
    pub fn _fx65_opcode(&mut self) {
        info!("FX65: 0x{:X}", self.opcode);
        self.increase_pc();
    }

    pub fn _f_opcodes(&mut self) {
        let sub_op = self.opcode & 0x00FF;
        debug!("Decode: 0x{:X}", sub_op);

        match sub_op{
            0x0007 => self._fx07_opcode(),
            0x000a => self._fx0a_opcode(),
            0x0015 => self._fx15_opcode(),
            0x0018 => self._fx18_opcode(),
            0x001e => self._fx1e_opcode(),
            0x0029 => self._fx29_opcode(),
            0x0033 => self._fx33_opcode(),
            0x0055 => self._fx55_opcode(),
            0x0065 => self._fx65_opcode(),
            _ => {
                // Using the catch all as a NOOP
                info!("Catch all 0xFxxx: 0x{:X}", self.opcode);
                self.increase_pc();
            }
        }
    }

    pub fn new() -> Chipate {
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
            program: "",
        };

        return chip;
    }
}

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
    stack: Vec<u16>,
    // sp: u8,

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
        // self.sp = 0x0;

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
        self.fetch_opcode();
        self.decode_opcode();

        let one_second = time::Duration::from_millis(15);
        thread::sleep(one_second);
    }

    pub fn draw_screen(&mut self) {
        // debug!("Drawing to Screen")
    }

    pub fn set_keys(&mut self) {
        // debug!("Saving Key State")
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
    }

    pub fn decode_opcode(&mut self) {
        let op = self.opcode & 0xF000;

        match op {
            0x0000 => self._0_opcodes(),
            0x2000 => self._2nnn_opcode(),
            0x6000 => self._6xnn_opcode(),
            0x7000 => self._7xnn_opcode(),
            0xA000 => self._annn_opcode(),
            0xE000 => self._e_opcodes(),
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

    pub fn _0_opcodes(&mut self) {
        let sub_op = self.opcode & 0x00FF;

        match sub_op{
            0x00E0 => self._00e0_opcode(),
            0x00EE => self._00ee_opcode(),
            _ => {
                /// 0NNN 	Call 		Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
                info!("Calls RCA 1802 program at address NNN. 0x{:X}", self.opcode);
                self.increase_pc();
            }
        }
    }

    /// 00E0 	Display 	disp_clear() 	Clears the screen.
    pub fn _00e0_opcode(&mut self) {
        info!("TODO: Clear Display");
        self.increase_pc();
    }

    /// 00EE 	Flow 	return; 	Returns from a subroutine.
    pub fn _00ee_opcode(&mut self) {
        info!("Return from sub routine");
        let addr = self.stack.pop();
        self.pc = addr.unwrap() as u16;
        debug!("Returning to {:X}", addr.unwrap());
        self.increase_pc();
    }

    /// 1NNN 	Flow 	goto NNN; 	Jumps to address NNN.
    pub fn _1nnn_opcode(&mut self) {
        info!("1NNN: 0x{:X}", self.opcode);
        let addr = self.opcode & 0x0FFF;
        self.pc = addr;
        debug!("Jumping to 0x{:X}", addr);
    }

    /// 2NNN 	Flow 	*(0xNNN)() 	Calls subroutine at NNN.
    pub fn _2nnn_opcode(&mut self) {
        info!("2NNN: 0x{:X}", self.opcode);
        self.stack.push(self.pc);
        debug!("Pushing Callback to Stack: 0x{:X}", self.pc);
        // self.sp += 1;
        self.pc = self.opcode & 0x0FFF;
        debug!("Calls subroutine at: 0x{:X}", (self.opcode & 0x0FFF));
    }

    /// 6XNN 	Const 	Vx = NN 	Sets VX to NN.
    pub fn _6xnn_opcode(&mut self) {
        info!("6XNN: 0x{:X}", self.opcode);
        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 8;

        let nn = self.opcode & 0x00FF;

        self.v[reg as usize] = nn as u8;
        self.increase_pc();
        debug!("Set V{:X} (V{}) 0x{:X}", reg, reg, nn);
    }

    /// 7XNN 	Const 	Vx += NN 	Adds NN to VX.
    pub fn _7xnn_opcode(&mut self) {
        info!("7XNN: 0x{:X}", self.opcode);

        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 8;

        let nn = (self.opcode & 0x00FF) as u8;
        info!("0x{:X}", self.opcode);

        self.v[reg as usize] += nn;
        self.increase_pc();
        debug!("Add {:X} to V{:X} (V{}) = {:X}", nn, reg, reg, self.v[reg as usize]);
    }

    /// 8XY0	Assign	Vx=Vy	Sets VX to the value of VY.
    pub fn _8xy0_opcode(&mut self) {
        info!("8XY0: 0x{:X}", self.opcode);
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        self.v[x as usize] = self.v[y as usize];

        self.increase_pc();
        debug!("Assign	Vx=Vy	Sets VX to the value of VY");
    }

    /// 8XY1	BitOp	Vx=Vx|Vy	Sets VX to VX or VY. (Bitwise OR operation)
    pub fn _8xy1_opcode(&mut self) {
        info!("8XY1: 0x{:X}", self.opcode);

        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        self.v[x as usize] |= self.v[y as usize];

        self.increase_pc();
        debug!("BitOp	Vx=Vx|Vy	Sets VX to VX or VY. (Bitwise OR operation)");
    }

    /// 8XY2	BitOp	Vx=Vx&Vy	Sets VX to VX and VY. (Bitwise AND operation)
    pub fn _8xy2_opcode(&mut self) {
        info!("8XY2: 0x{:X}", self.opcode);

        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        self.v[x as usize] &= self.v[y as usize];

        self.increase_pc();
        debug!("8XY2	BitOp	Vx=Vx&Vy	Sets VX to VX and VY. (Bitwise AND operation)");
    }

    /// 8XY3	BitOp	Vx=Vx^Vy	Sets VX to VX xor VY.
    pub fn _8xy3_opcode(&mut self) {
        info!("8XY3: 0x{:X}", self.opcode);

        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        self.v[x as usize] ^= self.v[y as usize];

        self.increase_pc();
        debug!("8XY3	BitOp	Vx=Vx^Vy	Sets VX to VX xor VY.");
    }

    /// 8XY4	Math	Vx += Vy	Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
    pub fn _8xy4_opcode(&mut self) {
        info!("8XY4: 0x{:X}", self.opcode);

        let mut buf: u32;

        // TODO: Working
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        if self.v[x as usize] > 0xFF - self.v[y as usize] {
            self.v[0xF as usize] = 1;
        } else {
            self.v[0xF as usize] = 0;
        }

        // Easy way to add our buffers without overflow
        buf = self.v[x as usize] as u32 + self.v[y as usize] as u32;
        self.v[x as usize] = buf as u8;

        self.increase_pc();
    }

    /// 8XY5	Math	Vx -= Vy	VY is subtracted from VX. VF is set to 0 when there's
    /// a borrow, and 1 when there isn't.
    pub fn _8xy5_opcode(&mut self) {
        info!("8XY5: 0x{:X}", self.opcode);
        self.increase_pc();
    }

    /// 8XY6	BitOp	Vx >> 1	Shifts VX right by one. VF is set to the value of the
    /// least significant bit of VX before the shift.[2]
    pub fn _8xy6_opcode(&mut self) {
        info!("8XY6: 0x{:X}", self.opcode);
        self.increase_pc();
    }

    /// 8XY7	Math	Vx=Vy-Vx	Sets VX to VY minus VX. VF is set to 0 when there's a borrow,
    ///and 1 when there isn't.
    pub fn _8xy7_opcode(&mut self) {
        info!("8XY7: 0x{:x}", self.opcode);
        self.increase_pc();
    }

    /// 8XYE	BitOp	Vx << 1	Shifts VX left by one. VF is set to the value of the most significant
    /// bit of VX before the shift.[2]
    pub fn _8xye_opcode(&mut self) {
        info!("8XYE: 0x{:x}", self.opcode);
        self.increase_pc();
    }

    pub fn _8_opcodes(&mut self) {
        let sub_op = self.opcode & 0x000F;
        debug!("Decode: 0x{:X}", sub_op);

        match sub_op{
            0x0000 => self._8xy0_opcode(),
            0x0001 => self._8xy1_opcode(),
            0x0002 => self._8xy2_opcode(),
            0x0003 => self._8xy3_opcode(),
            0x0004 => self._8xy4_opcode(),
            0x0005 => self._8xy5_opcode(),
            0x0006 => self._8xy6_opcode(),
            0x0007 => self._8xy7_opcode(),
            0x000e => self._8xye_opcode(),
            _ => {
                // Using the catch all as a NOOP
                info!("Catch all 0x8xxx: 0x{:X}", self.opcode);
                self.increase_pc();
            }
        }
    }


    /// ANNN 	MEM 	I = NNN 	Sets I to the address NNN.
    pub fn _annn_opcode(&mut self) {
        info!("ANNN: 0x{:X}", self.opcode);
        self.i = self.opcode & 0x0FFF;
        self.increase_pc();
        debug!("Set I: {:X}", self.i);
    }


    pub fn _e_opcodes(&mut self) {
        let sub_op = self.opcode & 0x00FF;
        debug!("Decode: 0x{:X}", sub_op);

        match sub_op{
            0x009E => self._ex9e_opcode(),
            0x00A1 => self._exa1_opcode(),
            _ => {
                // Using the catch all as a NOOP
                info!("Catch all 0xENNN: 0x{:X}", self.opcode);
                self.increase_pc();
            }
        }
    }

    /// EX9E 	KeyOp 	if(key()==Vx) 	Skips the next instruction if the key stored in VX is pressed.
    /// (Usually the next instruction is a jump to skip a code block)
    pub fn _ex9e_opcode(&mut self) {
        info!("EX9E: 0x{:X}", self.opcode);
        debug!("TODO: check for key");
    }

    /// EXA1 	KeyOp 	if(key()!=Vx) 	Skips the next instruction if the key stored in VX isn't pressed.
    /// (Usually the next instruction is a jump to skip a code block)
    pub fn _exa1_opcode(&mut self) {
        info!("EXA1: 0x{:X}", self.opcode);
        debug!("TODO: check for key");
        self.increase_pc();
        self.increase_pc();
    }

    /// FX07 	Timer 	Vx = get_delay() 	Sets VX to the value of the delay timer.
    pub fn _fx07_opcode(&mut self) {
        info!("FX07: 0x{:X}", self.opcode);

        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 8;
        self.v[reg as usize] = self.delay_timer;

        self.increase_pc();
    }

    /// FX0A 	KeyOp 	Vx = get_key() 	A key press is awaited, and then stored in VX.
    /// (Blocking Operation. All instruction halted until next key event)
    pub fn _fx0a_opcode(&mut self) {
        info!("FX0A: 0x{:X}", self.opcode);
        info!("Implement Blocking Operation");
        self.increase_pc();
    }

    /// FX15 	Timer 	delay_timer(Vx) 	Sets the delay timer to VX.
    pub fn _fx15_opcode(&mut self) {
        info!("FX15: 0x{:X}", self.opcode);

        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 8;
        self.delay_timer = self.v[reg as usize];

        self.increase_pc();
    }

    /// FX18 	Sound 	sound_timer(Vx) 	Sets the sound timer to VX.
    pub fn _fx18_opcode(&mut self) {
        info!("FX18: 0x{:X}", self.opcode);

        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 8;
        self.sound_timer = self.v[reg as usize];

        self.increase_pc();
    }

    /// FX1E 	MEM 	I +=Vx 	Adds VX to I.[3]
    pub fn _fx1e_opcode(&mut self) {
        info!("FX1E: 0x{:X}", self.opcode);

        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 8;
        self.i +=  self.v[reg as usize] as u16;

        self.increase_pc();
    }

    /// FX29 	MEM 	I=sprite_addr[Vx] 	Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    pub fn _fx29_opcode(&mut self) {
        info!("FX29: 0x{:X}", self.opcode);
        info!("TODO: Display Function");
        self.increase_pc();
    }

    /// FX33 	BCD 	....  Stores the binary-coded decimal representation of VX, with the most
    /// significant of three digits at the address in I, the middle digit at I plus 1, and the least
    /// significant digit at I plus 2. (In other words, take the decimal representation of VX, place
    /// the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones
    /// digit at location I+2.)
    /// I sat and tried doing this a couple of ways but TJA's seemed cleaner
    /// RAM[I] = (V[((opcode&0x0F00)>>8)]/100);
    /// RAM[I+1] = ((V[((opcode&0x0F00)>>8)]/10)%10);
    /// RAM[I+2] = ((V[((opcode&0x0F00)>>8)]%100)%10);
    /// PC+=2;
    pub fn _fx33_opcode(&mut self) {
        info !("FX33: 0x{:X}", self.opcode);

        let mut reg = self.opcode & 0x0F00;
        reg = reg >> 8;

        self.memory[self.i as usize] = self.v[reg as usize] / 100;
        debug!("BCD: {}", self.memory[self.i as usize]);
        self.memory[(self.i + 1) as usize] = self.v[reg as usize] / 10 % 10;
        debug!("BCD: {}", self.memory[(self.i + 1) as usize]);
        self.memory[(self.i + 2) as usize] = self.v[reg as usize] / 10 % 10;
        debug!("BCD: {}", self.memory[(self.i + 2) as usize]);

        self.increase_pc();
    }

    /// FX55 	MEM 	reg_dump(Vx,&I) 	Stores V0 to VX (including VX) in memory starting at address I.[4]
    pub fn _fx55_opcode(&mut self) {
        info!("FX55: 0x{:X}", self.opcode);
        self.increase_pc();
    }

    /// FX65 	MEM 	reg_load(Vx,&I) 	Fills V0 to VX (including VX) with values from memory
    /// starting at address I.[4]
    pub fn _fx65_opcode(&mut self) {
        info!("FX65: 0x{:X}", self.opcode);
        self.increase_pc();
    }

    pub fn _f_opcodes(&mut self) {
        let sub_op = self.opcode & 0x00FF;

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
            stack: Vec::new(),
            // sp: 0,
            key: [0; 16],
            program: "",
        };

        return chip;
    }
}

// pub fn bcd(n: &u16) -> Vec<u16> {
//     let s = &format!("{}", n);
//     let mut v: Vec<u16> = vec![0u16,0u16,0u16];
//     info!("{}",s);

//     for b in s.as_bytes().iter() {
//         v.push(*b as u16);
//     }
//     v.reverse();
//     info!("v {:?}",v);
//     let dif = v.len() - 3;
//     v
// }

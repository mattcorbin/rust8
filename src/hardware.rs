use rand::prelude::random;

use crate::constants::{
    DISPLAY_MAX_X,
    DISPLAY_MAX_Y,
    OPCODE_SIZE,
    PROG_MEM_START,
    SPRITE_STARTING_LOC,
    SPRITES
};

struct Display {
    pub pixels: [[u8; DISPLAY_MAX_X]; DISPLAY_MAX_Y],
    pub update_required: bool
}

pub struct CPU {
    memory: [u8; 4096],
    v: [u8; 16],
    i: usize,
    pc: u16,
    sp: u8,
    stack: [u16; 24],
    keypad: [bool; 16],
    keypad_wait: bool,
    keypad_register: usize,
    display: Display,
    delay: u8,
    sound: u8
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = CPU {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: PROG_MEM_START,
            sp: 0,
            stack: [0; 24],
            keypad: [false; 16],
            keypad_wait: false,
            keypad_register: 0,
            display: Display{
                pixels: [[0; DISPLAY_MAX_X]; DISPLAY_MAX_Y],
                update_required: false
            },
            delay: 0,
            sound: 0
        };
        for item in 0..SPRITES.len() {
            cpu.memory[SPRITE_STARTING_LOC + item] = SPRITES[item];
        }
        return cpu;
    }

    fn next(&mut self) {
        self.pc += OPCODE_SIZE;
    }

    fn skip(&mut self) {
        self.pc += 2 * OPCODE_SIZE;
    }

    fn decode_and_run_instruction(&mut self, instruction: u16) {
        let nibbles: (u8, u8, u8, u8) = (
            ((instruction & 0xF000) >> 12) as u8,
            ((instruction & 0x0F00) >> 8) as u8,
            ((instruction & 0x00F0) >> 4) as u8,
            (instruction & 0x000F) as u8,
        );

        let addr: u16 = instruction & 0x0FFF;
        let kk: u8 = (instruction & 0x00FF) as u8;
        let x: usize = nibbles.1 as usize;
        let y: usize = nibbles.2 as usize;
        let n: usize = nibbles.3 as usize;

        match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x00, _, _, _) => self.op_0nnn(addr),
            (0x01, _, _, _) => self.op_1nnn(addr),
            (0x02, _, _, _) => self.op_2nnn(addr),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xkk(x, kk),
            (0x07, _, _, _) => self.op_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8xy6(x, y),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8xye(x, y),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_annn(addr),
            (0x0b, _, _, _) => self.op_bnnn(addr),
            (0x0c, _, _, _) => self.op_cxkk(x, kk),
            (0x0d, _, _, _) => self.op_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx66(x),
            _ => self.next()
        }
    }

    /// 0x00E0 - CLS
    /// Clears the display
    /// Sets all pixels in the display to off (0)
    fn op_00e0(&mut self) {
        for x in 0..DISPLAY_MAX_X {
            for y in 0..DISPLAY_MAX_Y {
                self.display.pixels[x][y] = 0;
            }
        }
        self.next();
    }

    /// 0x00EE - RET
    /// Return from a subroutine
    /// Sets the program counter to the most recent address on the stack
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    /// 0x0nnn - SYS nnn
    /// Legacy version of jump
    /// Sets the program counter to the 12 bit address passed in (nnn)
    fn op_0nnn(&mut self, addr: u16) {
        self.pc = addr;
    }

    /// 0x1nnn - JP nnn
    /// Jump to an address (nnn)
    /// Sets the program counter to the 12 bit address passed in (nnn)
    fn op_1nnn(&mut self, addr: u16) {
        self.pc = addr;
    }

    /// 0x2nnn - CALL nnn
    /// Call function at an address (nnn)
    /// Adds a new entry to the call stack of the current program counter,
    /// Then sets the program counter to the 12 bit address passed in (nnn)
    fn op_2nnn(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc + OPCODE_SIZE;
        self.sp += 1;
        self.pc = addr;
    }

    /// 0x3xkk - SE Vx, kk
    /// Skip next instruction of the value in equals the value passed in (kk)
    fn op_3xkk(&mut self, x: usize, kk: u8) {
        let check = self.v[x] == kk;
        match check {
            true => self.skip(),
            false => self.next()
        }
    }

    /// 0x4xkk - SNE Vx, kk
    /// Skip next instruction of the value in Vx does not equal the value passed in (kk)
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        let check = self.v[x] != kk;
        match check {
            true => self.skip(),
            false => self.next()
        }
    }

    /// 0x5xyx - SE Vx, Vy
    /// Skip next instruction of the value in Vx equals the value in Vy
    fn op_5xy0(&mut self, x: usize, y: usize) {
        let check = self.v[x] == self.v[y];
        match check {
            true => self.skip(),
            false => self.next()
        }
    }

    /// 0x6xkk - LD Vx, kk
    /// Set the register Vx to be the byte passed in (kk)
    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
        self.next();
    }

    /// 0x7xkk - ADD Vx, kk
    /// Add the byte passed in (kk) to the value contained in Vx, and store in Vx
    fn op_7xkk(&mut self, x: usize, kk: u8) {
        self.v[x] += kk;
        self.next();
    }

    /// 0x8xy0 - LD Vx, Vy
    /// Store the value in register Vy in the register Vx
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
        self.next();
    }

    /// 0x8xy1 - OR Vx, Vy
    /// Perform a bitwise OR of the value in register Vx with the value in register Vy,
    /// Then store the result in Vx
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
        self.next();
    }

    /// 0x8xy2 - AND Vx, Vy
    /// Perform a bitwise AND of the value in register Vx with the value in register Vy,
    /// Then store the result in Vx
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
        self.next();
    }

    /// 0x8xy3 - XOR Vx, Vy
    /// Perform a bitwise XOR of the value in register Vx with the value in register Vy,
    /// Then store the result in Vx
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
        self.next();
    }

    /// 0x8xy4 - ADD Vx, Vy
    /// Add the values of registers Vx and Vy, and store in Vx
    /// Additionally, if an overflow occurs, register VF is set to 1.
    /// Set VF to 0 otherwise
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let retval = self.v[x].checked_add(self.v[y]);
        match retval {
            Some(_) => self.v[0xF] = 0,
            None => self.v[0xF] = 1
        }
        self.v[x] += self.v[y];
        self.next();
    }

    /// 0x8xy5 - SUB Vx, Vy
    /// Subtract the value of register Vy from the value of register Vx, and store in Vx
    /// Additionally, if a borrow occurs, register VF is set to 0.
    /// Set VF to 0 otherwise
    fn op_8xy5(&mut self, x: usize, y: usize) {
        let check = self.v[x] < self.v[y];
        if check {
            self.v[0xF] = 0;
        } else {
            self.v[0xF] = 1;
        }
        self.v[x] -= self.v[y];
        self.next();
    }

    /// 0x8xy6 - SHR Vx, Vy
    /// Shift the value stored in register Vy right by one bit, and store the value in Vx
    /// Additionally, set register VF to the least significant bit of the value in Vy
    /// Prior to performing the shift.
    fn op_8xy6(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y] >> 1;
        self.v[0xF] = self.v[y] & 0b0000_0001;
        self.next();
    }

    /// 0x8xy7 - SUBN Vx, Vy
    /// Subtract the value of register Vx from the value of register Vy, and store in Vx
    /// Additionally, if a borrow occurs, register VF is set to 0.
    /// Set VF to 0 otherwise
    fn op_8xy7(&mut self, x: usize, y: usize) {
        let check = self.v[x] < self.v[y];
        if check {
            self.v[0xF] = 0;
        } else {
            self.v[0xF] = 1;
        }
        self.v[x] = self.v[y] - self.v[x];
        self.next();
    }

    /// 0x8xyE - SHL Vx, Vy
    /// Shifts the value in register Vy to the left by one, then stores the result in Vx
    /// Additionally, sets VF to be the most significant bit of Vy pre-shift.
    fn op_8xye(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y] << 1;
        self.v[0xF] = self.v[y] & 0b1000_0000;
        self.next();
    }

    /// 0x4xkk - SNE Vx, Vy
    /// Skip next instruction of the value in Vx does not equal the value in Vy
    fn op_9xy0(&mut self, x: usize, y: usize) {
        let check = self.v[x] != self.v[y];
        if check {
            self.pc += 2 * OPCODE_SIZE;
        }
        self.next();
    }

    /// 0xAnnn - LD I, nnn
    /// Sets the register I to the 12 bit address passed in (nnn)
    fn op_annn(&mut self, addr: u16) {
        self.i = addr as usize;
        self.next();
    }

    /// 0xBnnn - JP V0, nnn
    /// Sets the program counter to the 12 bit address passed in (nnn) offset by the value in V0
    fn op_bnnn(&mut self, addr: u16) {
        self.pc = addr + self.v[0] as u16;
        self.next();
    }

    /// 0xCxkk - RND Vx, kk
    /// Set the register Vx to be a random number bitwise ANDed with the byte passed in (kk)
    fn op_cxkk(&mut self, x: usize, kk: u8) {
        let rng: u8 = random();
        self.v[x] = rng & kk;
        self.next();
    }

    /// 0xDxyn - DRW Vx, Vy, n
    /// Read in n bytes from memory, starting at the address stored in the register I
    /// The bytes read in are then displayed on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen, and if this causes any pixels to be erased,
    /// VF is set to 1, otherwise VF is set to 0.
    /// If the sprite is positioned so part of it would be outside of the display range, it will
    /// Wrap to the opposite side of the screen.
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        self.v[0xF] = 0;
        for byte in 0..n {
            let y = (self.v[y] as usize + byte) % DISPLAY_MAX_Y;
            for bit in 0..8 {
                let x =  (self.v[x] as usize + bit) % DISPLAY_MAX_X;
                let pixel = (self.memory[i + byte] >> (7 - bit)) & 1;
                self.v[0xF] |= item & self.display.pixels[y][x];
                self.display.pixels[y][x] ^= pixel;
            }
        }
        self.display.update_required = true;
        self.next();
    }

    /// 0xEx9E - SKP Vx
    /// Skips the next instruction if the key corresponding to the value stored in Vx is pressed
    fn op_ex9e(&mut self, x: usize) {
        let check = self.keypad[self.v[x] as usize];
        if check {
            self.pc += OPCODE_SIZE;
        }
        self.next();
    }

    /// 0xExA1 - SKNP Vx
    /// Skips the next instruction if the key corresponding to the value stored in Vx is not pressed
    fn op_exa1(&mut self, x: usize) {
        let check = self.keypad[self.v[x] as usize];
        if !check {
            self.pc += OPCODE_SIZE;
        }
        self.next();
    }

    /// 0xFx07 - LD Vx, DT
    /// Sets Vx to be the value currently stored in the delay timer register
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.delay;
        self.next();
    }

    /// 0xFx0A - LD Vx, K
    /// Blocks execution waiting for a key press, then stores the value of the key pressed in Vx
    fn op_fx0a(&mut self, x: usize) {
        self.keypad_wait = true;
        self.keypad_register = x;
        self.next();
    }

    /// 0xFx15 - LD DT, Vx
    /// Sets the delay timer register to be the value stored in the register Vx
    fn op_fx15(&mut self, x: usize) {
        self.delay = self.v[x];
        self.next();
    }

    /// 0xFx18 - LD ST, Vx
    /// Sets the sound timer register to be the value stored in the register Vx
    fn op_fx18(&mut self, x: usize) {
        self.sound = self.v[x];
        self.next();
    }

    /// 0xFx1E - ADD I, Vx
    /// Adds together the contents of the I register and the Vx register, and stores in I.
    fn op_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as usize;
        self.next();
    }

    /// 0xFx29 - LD F, Vx
    /// Stores the location of the sprite corresponding to the value in Vx into the I register
    fn op_fx29(&mut self, x: usize) {
        self.i = SPRITE_STARTING_LOC + self.v[x] as usize;
        self.next();
    }

    /// 0xFx33 - LD B, Vx
    /// Stores the BCD representation of the value in register Vx at memory locations I, I+1 and I+2
    /// BCD representation is hundreds value in the first location,
    /// Tens value in the next location, ones value in the final location.
    fn op_fx33(&mut self, x: usize) {
        let i = self.i as usize;
        let vx = self.v[x];
        let hundreds = vx / 100;
        self.memory[i] = hundreds;
        let tens = (vx - 100 * hundreds) / 10;
        self.memory[i + 1] = tens;
        let ones = vx - 100 * hundreds - 10 * tens;
        self.memory[i + 2] = ones;
        self.next();
    }

    /// 0xFx55 - LD [I], Vx
    /// Stores registers V0 through Vx into memory starting at the location stored in the I register
    fn op_fx55(&mut self, x: usize) {
        let i = self.i as usize;
        let end = x + 1;
        for register in 0..end {
            self.memory[i + register] = self.v[register];
        }
        self.next();
    }

    /// 0xFx66 - LD Vx, [I]
    /// Loads values from memory starting at the location stored in the I register to the registers
    /// V0 through Vx
    fn op_fx66(&mut self, x: usize) {
        let i = self.i as usize;
        let end = x + 1;
        for register in 0..end {
            self.v[register] = self.memory[i + register];
        }
        self.next();
    }
}
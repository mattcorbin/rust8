pub const PROG_MEM_START: u16 = 0x200;
pub const RESERVED_MEM_START: u16 = 0xEA0;
pub const DISPLAY_MEM_START: u16 = 0xF00;
pub const DISPLAY_MAX_X: u8 = 0x3F;
pub const DISPLAY_MAX_Y: u8 = 0x1F;

pub struct Keyboard {

}

pub struct Display {
    pub pixels: [[u8; 0x1F]; 0x3F]
}

pub struct CPU {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub i: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub keyboard: Keyboard,
    pub display: Display,
    pub delay: u8,
    pub sound: u8
}
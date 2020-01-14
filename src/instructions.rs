use super::hardware::CPU;
use super::hardware::Keyboard;
use super::hardware::Display;

fn cls(cpu: CPU) -> Result<(), &str> {

}

fn ret(cpu: CPU) -> Result<(), &str> {

}

fn sys(cpu: CPU, addr: u16) -> Result<(), &str> {

}

fn jp(mut cpu: CPU, addr: u16, offset: u8) -> Result<(), &str> {
    cpu.pc = addr + offset as u16;
    Ok(())
}

fn call(cpu: CPU, addr: u16) -> Result<(), &str> {

}

fn se(cpu: CPU, test: u8, expected: u8) -> Result<(), &str> {

}

fn sei(cpu: CPU, test: u8, expected: u8) -> Result<(), &str> {

}

fn sne(cpu: CPU, test: u8, expected: u8) -> Result<(), &str> {

}

fn ld(cpu: CPU, destination: u8, value: u8) -> Result<(), &str> {

}

fn ldi(cpu: CPU, destination: u8, value: u8) -> Result<(), &str> {

}

fn ldw(cpu: CPU, destination: u8, value: u8) -> Result<(), &str> {

}

fn add(cpu: CPU, base: u8, operand: u8) -> Result<(), &str> {

}

fn or(cpu: CPU, base: u8, operand: u8) -> Result<(), &str> {

}

fn and(cpu: CPU, base: u8, operand: u8) -> Result<(), &str> {

}

fn xor(cpu: CPU, base: u8, operand: u8) -> Result<(), &str> {

}

fn sub(cpu: CPU, base: u8, operand: u8) -> Result<(), &str> {

}

fn shr(cpu: CPU, base: u8, operand: u8) -> Result<(), &str> {

}

fn shl(cpu: CPU, base: u8, operand: u8) -> Result<(), &str> {

}

fn subn(cpu: CPU, base: u8, operand: u8) -> Result<(), &str> {

}

fn rnd(cpu: CPU, destination: u8) -> Result<(), &str> {

}

fn drw(cpu: CPU, x: u8, y: u8, count: u8) -> Result<(), &str> {

}

fn skp(cpu: CPU, test: u8) -> Result<(), &str> {

}

fn sknp(cpu: CPU, test: u8) -> Result<(), &str> {

}

fn nibbles_to_bytes(nibbles: Vec<u8>) -> u16 {
    let starting_offset = 4 * (vec.len() - 1);
    let mut retval = 0x0000;
    for nibble in &nibbles {
        retval = (retval << 4) | *nibble as u16;
    }
    retval
}

pub fn decode_and_run_instruction(cpu: CPU, instruction: u16) -> Result<(), &str> {
    let error_message = Err("Unrecognized instruction");
    let bytes: [u8; 2] = instruction.to_be_bytes();
    let nibbles: [u8; 4] = [
        (instruction & 0xF) as u8,
        ((instruction >> 4) & 0xF) as u8,
        ((instruction >> 8) & 0xF) as u8,
        ((instruction >> 12) & 0xF) as u8,
    ];
    match nibbles[0] {
        0x0 => match nibbles[2] {
            0xE => match nibbles[3] {
                0x0 => cls(cpu),
                0xE => ret(cpu),
                _ => sys(cpu, nibbles_to_bytes(nibbles[1..3].to_vec()))
            },
            _ => sys(cpu, nibbles_to_bytes(nibbles[1..3].to_vec()))
        },
        0x1 => jp(cpu, nibbles_to_bytes(nibbles[1..3].to_vec()), None),
        0x2 => call(cpu, nibbles_to_bytes(nibbles[1..3].to_vec())),
        0x3 => se(cpu, nibbles[1], nibbles_to_bytes(nibbles[2..3].to_vec) as u8),
        0x4 => sne(cpu, nibbles[1], nibbles_to_bytes(nibbles[2..3].to_vec) as u8),
        0x5 => match nibbles[3] {
            0x0 => se(cpu, nibbles[1], ),
            _ => error_message
        },
        0x6 => ld,
        0x7 => add,
        0x8 => match nibbles[3] {
            0x0 => ld,
            0x1 => or,
            0x2 => and,
            0x3 => xor,
            0x4 => add,
            0x5 => sub,
            0x6 => shr,
            0x7 => sub,
            0xE => shl,
            _ => error_message
        },
        0x9 => match nibbles[3] {
            0x0 => sne,
            _ => error_message
        },
        0xA => ld,
        0xB => jp,
        0xC => rnd,
        0xD => drw,
        0xE => match nibbles[2] {
            0x9 => match nibbles[3] {
                0xE => skp,
                _ => error_message
            },
            0xA => match nibbles[3] {
                0x1 => sknp,
                _ => error_message
            }
            _ => error_message
        },
        0xF => match nibbles[2] {
            0x0 => match nibbles[3] {
                0x7 => ld,
                0xA => ld,
                _ => error_message
            },
            0x1 => match nibbles[3] {
                0x5 => ld,
                0x8 => ld,
                0x3 => add,
                _ => error_message
            },
            0x2 => match nibbles[3] {
                0x9 => ld,
                _ => error_message
            },
            0x3 => match nibbles[3] {
                0x3 => ld,
                _ => error_message
            },
            0x5 => match nibbles[3] {
                0x5 => ld,
                _ => error_message
            },
            0x6 => match nibbles[3] {
                0x5 => ld,
                _ => error_message
            },
            _ => error_message
        },
        _ => error_message
    }
}
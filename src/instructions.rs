use crate::hardware::{CPU, Display, DISPLAY_MAX_X, DISPLAY_MAX_Y, Keyboard};

fn cls(&mut cpu: CPU) -> Result<(), &str> {
    for x in range(0, DISPLAY_MAX_X) {
        for y in range(0, DISPLAY_MAX_Y) {
            cpu.display.pixels[x][y] = 0;
        }
    }
    Ok(())
}

fn ret(&mut cpu: CPU) -> Result<(), &str> {
    let sp = cpu.sp;
    if sp == 0 {
        return Err("Stack is empty, nothing to return to");
    }
    cpu.pc = cpu.stack[sp];
    cpu.sp -= 1;
    Ok(())
}

fn sys(&mut cpu: CPU, addr: u16) -> Result<(), &str> {
    cpu.pc = addr;
    Ok(())
}

fn jp(&mut cpu: CPU, addr: u16, offset: u8) -> Result<(), &str> {
    cpu.pc = addr + offset as u16;
    Ok(())
}

fn call(&mut cpu: CPU, addr: u16) -> Result<(), &str> {
    if cpu.sp >= 15 {
        return Err("Stack is full");
    }
    cpu.sp += 1;
    cpu.stack[cpu.sp] = cpu.pc;
    cpu.pc = addr;
    Ok(())
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

pub fn decode_and_run_instruction(&mut cpu: CPU, instruction: u16) -> Result<(), &str> {

}
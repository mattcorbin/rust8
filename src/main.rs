use crate::hardware::{CPU, Display, Keyboard};

mod instructions;
mod hardware;

fn main() {
    let mut cpu = CPU {
        memory: [0; 4096],
        v: [0; 16],
        i: 0,
        pc: 0,
        sp: 0,
        stack: [0; 16],
        keyboard: Keyboard {

        },
        display: Display {
            pixels: [[0; 0x1F]; 0x3F]
        },
        delay: 0,
        sound: 0
    };
}

use super::*;



fn pop(console: &mut Console) -> u8 {
    0
}

fn pushin_p(console: &mut Console) {
    let p: u8 = 0;

}

pub fn interpret_opcode(console: &mut Console, opcode: Opcode) {
/*  0 = accumulator
    1 = immed
    2 = implied
    3 = relative
    4 = absolute
    5 = zero page
    6 = indirect
    7 = absolute indexed x
    8 = absolute indexed y
    9 = zero page indexed
    10 = indexed indirect
    11 = indirect indexed */

    let cpu = &mut console.CPU;
    let mem = &console.Memory;

    

    let push = |data: u8| {
        mem.write(console, cpu.SP as usize, data);
        cpu.SP -= 1;
    };

    let pull= || {
        cpu.SP += 1;
        mem.read(console, cpu.SP as usize);
    };

    let pushin_p = || {
        let p: u8 = 0b00011000;
        p |= cpu.carry as u8;
        p |= (cpu.zero as u8) << 1;
        p |= (cpu.interupt_disable as u8) << 2;
        p |= (cpu.decimal as u8) << 3;
        p |= (cpu.overflow as u8) << 6;
        p |= (cpu.negative as u8) << 7;

        push(p);
    }

    let pullin_p = || {
        let p = pull();
        cpu.carry = (p & 0b00000001) > 0;
        cpu.zero = (p & 0b00000010) > 0;
        cpu.interupt_disable = (p & 0b00000100) > 0;
        cpu.decimal = (p & 0b00001000) > 0;
        cpu.overflow = (p & 0b01000000) > 0;
        cpu.negative = (p & 0b10000000) > 0;
    }
    /*
    ---- ----
    NVss DIZC
    |||| ||||
    |||| |||+- Carry
    |||| ||+-- Zero
    |||| |+--- Interrupt Disable
    |||| +---- Decimal
    ||++------ No CPU effect, see: the B flag
    |+-------- Overflow
    +--------- Negative
    */


    match opcode.code {
        0x00 => { // BRK
            push((cpu.PC >> 8) as u8);
            push((cpu.PC & 0x00FF) as u8);
            pushin_p();
            cpu.interupt_disable = true;
            cpu.PC = 0;
            cpu.PC |= mem.read(console, 0xFFFE) as u16;
            cpu.PC |= (mem.read(console, 0xFFFF) as u16) << 8;
        }
        0x01 | 0x05 | 0x09 | 0x0D | 0x11 | 0x15 | 0x19 | 0x1D => { // ORA
            match opcode.address_mode {
                1 => {
                    cpu.A |= (cpu.PC + 1)
                }
                4 => {
                    cpu.A |= mem.read(console, ((cpu.PC + 1) | (cpu.PC + 2) << 8) as usize);
                }
                5 => {
                    cpu.A |= |= mem.read(console, (cpu.PC + 1) as usize);
                }
            }
            cpu.A |= mem.read(console, )
        }
    }
}
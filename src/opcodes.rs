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
    9 = zero page indexed x
    10 = zero page indexed y
    11 = indexed indirect
    12 = indirect indexed */

    let cpu = &mut console.CPU;
    let mem = &console.Memory;
    let pc = cpu.PC;

    let addr = match opcode.address_mode {
        0 => 0,

        1 => pc+1,

        2 => 0,

        3 => {
            match mem.read(console, pc+1) {
                x if x < 128 => {
                    pc+2+(x as u16)
                }
                x => {
                    pc+2-(!((x as u16 | 0xFF00)-1))
                }
            }
        },

        4 => mem.read16(console, pc+1),

        5 => mem.read(console, pc+1) as u16,

        6 => mem.read16(console,mem.read16(console, pc+1)),

        7 => mem.read16(console, pc+1) + (cpu.X as u16),

        8 => mem.read16(console, pc+1) + (cpu.Y as u16),

        9 => ((mem.read(console, pc+1) + cpu.X) as u16) % 0xFF,

        10 => ((mem.read(console, pc+1) + cpu.X) as u16) % 0xFF,

        11 => mem.read16(console, mem.read16(console, pc+1+(cpu.X as u16)))

        12 => mem.read16(console, mem.read16(console, pc+1)) + (cpu.Y as u16)
    };

   
    

    let push = |data: u8| {
        mem.write(console, cpu.SP as u16, data);
        cpu.SP -= 1;
    };

    let pull = || {
        cpu.SP += 1;
        mem.read(console, cpu.SP as u16);
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
    };

    let pullin_p = || {
        let p = pull();
        cpu.carry = (p & 0b00000001) > 0;
        cpu.zero = (p & 0b00000010) > 0;
        cpu.interupt_disable = (p & 0b00000100) > 0;
        cpu.decimal = (p & 0b00001000) > 0;
        cpu.overflow = (p & 0b01000000) > 0;
        cpu.negative = (p & 0b10000000) > 0;
    };
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
                    cpu.A |= (cpu.PC + 1);
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
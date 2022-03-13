use std::collections::btree_map::Values;

use super::*;

static cyclecount: [u8; 256] = [
7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6,
2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6,
2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, 
6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6,
2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6,
2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4,
2, 6, 2, 6, 4, 4, 4, 4, 2, 5, 2, 5, 5, 5, 5, 5,
2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4,
2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4,
2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6,
2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6,
2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7
];

fn pop(console: &mut Console) -> u8 {
    0
}

fn pushin_p(console: &mut Console) {
    let p: u8 = 0;

}

pub fn interpret_opcode(console: &mut Console, opcode: u8) {

    if console.CPU.pause > 0 {
        console.CPU.pause -= 1;
        return;
    } else {
        console.CPU.pause = cyclecount[opcode as usize] - 1; // lazy (probably should fix the LUT)
    }

    let cpu = &mut console.CPU;
    let mem = &console.Memory;
    let pc = cpu.PC;

    let aaa = (opcode & 0b11100000) >> 5;
    let bbb = (opcode & 0b00011100) >> 2;
    let cc = opcode & 0b00000011;

    let addr: u16 = match cc {
        0b00 => {
            match bbb {
                0b000 => { // immed
                    if aaa >> 5 > 0b011 {
                        pc+1
                    } else if opcode == 0x20 {
                        mem.read16(console, mem.read16(console, pc+1))
                    } else {
                        0
                    }
                }
                0b001 => { // zero page
                    mem.read16(console, pc+1)
                }
                0b011 => { // absolute
                    if opcode == 0x6C {
                        mem.read16(console, mem.read16(console, pc+1))
                    } else {
                        mem.read16(console, pc+1)
                    }
                }
                0b101 => { // zp indexed x
                    ((mem.read(console, pc+1) + cpu.X) as u16) % 0xFF
                }
                0b111 => { // abs indexed x
                    mem.read16(console, pc+1) + (cpu.X as u16)
                }
                _ => {
                    0
                }
            }
        }
        0b01 => {
            match bbb {
                0b000 => { // (zp,X)
                    mem.read16(console, (mem.read(console, pc+1) as u16 + cpu.X as u16)% 0xFF) as u16
                }
                0b001 => { // zp
                    mem.read(console, pc+1) as u16
                }
                0b010 => { // immed
                    pc+1
                }
                0b011 => { // abs
                    mem.read16(console, pc+1)
                }
                0b100 => { // (zp), Y
                    let temp = mem.read16(console, pc+1);
                    if ((cpu.Y as u16 + (temp & 0xFF)) & 0x100) > 0 {
                        cpu.pause += 1;
                    }
                    mem.read16(console, temp as u16) + cpu.Y as u16
                }
                0b101 => { // zp,X
                    (mem.read(console, pc+1) as u16 + cpu.X as u16) % 0xFF
                }
                0b110 => { // abs, Y
                    let temp = mem.read16(console, pc+1);
                    if ((cpu.Y as u16 + (temp & 0xFF)) & 0x100) > 0 {
                        cpu.pause += 1;
                    }
                    temp + (cpu.Y as u16)
                }
                0b111 => { // abs, X
                    let temp = mem.read16(console, pc+1);
                    if ((cpu.X as u16 + (temp & 0xFF)) & 0x100) > 0 {
                        cpu.pause += 1;
                    }
                    temp + (cpu.X as u16)
                }
            }
        }
        0b10 => {
            match bbb {
                0b000 => {
                    if aaa >> 5 > 0b011 {
                        pc+1
                    } else {
                        0
                    }
                }
                0b001 => { // zp
                    mem.read(console, pc+1) as u16
                }
                0b011 => { // abs
                    mem.read16(console, pc+1)
                }
                0b101 => {
                    if opcode == 0x96 || opcode == 0xB6 {
                        (mem.read(console, pc+1) as u16 + cpu.Y as u16) % 0xFF // zp, Y
                    } else {
                        (mem.read(console, pc+1) as u16 + cpu.X as u16) % 0xFF // zp, X
                    }
                }
                0b111 => {
                    if opcode == 0x9E || opcode == 0xBE {
                        mem.read16(console, pc+1) + (cpu.Y as u16)
                    } else {
                        mem.read16(console, pc+1) + (cpu.X as u16)
                    }
                }
                _ => {
                    0
                }
            }
        }
        0b11 => {
            unimplemented!("Undocumented opcode used (unimplemented)")
        }
        
    };

    let push = |data: u8| {
        mem.write(console, cpu.SP as u16, data);
        cpu.SP -= 1;
    };

    let pull = || {
        cpu.SP += 1;
        mem.read(console, cpu.SP as u16)
    };

    let pushin_p = || {
        let p: u8 = 0x18;
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


    // ALU ops
    let ORA = || {
        cpu.A |= mem.read(console, addr);
        cpu.negative = (cpu.A & 0x80) > 0;
        cpu.zero = cpu.A == 0;
    };

    let AND = || {
        cpu.A &= mem.read(console, addr);
        cpu.negative = (cpu.A & 0x80) > 0;
        cpu.zero = cpu.A == 0;
    };

    let EOR = || {
        cpu.A ^= mem.read(console, addr);
        cpu.negative = (cpu.A & 0x80) > 0;
        cpu.zero = cpu.A == 0;
    };

    let ADC = || {
        let value = mem.read(console, addr);
        let temp = cpu.A as u16 + value as u16;
        cpu.A = (temp & 0x00FF) as u8;
        cpu.carry = temp > 0xFF;
        cpu.negative = (temp & 0x80) > 0;
        cpu.overflow = ((!(cpu.A ^ value) & (cpu.A ^ (temp & 0x00FF) as u8)) & 0x80) > 0;
        cpu.zero = temp == 0;
    };

    let STA = || {
        mem.write(console, addr, cpu.A);
    };

    let LDA = || {
        cpu.A = mem.read(console, addr);
    };

    let CMP = || {
        let temp = mem.read(console, addr);
        cpu.negative = (((0x0100 | cpu.A as u16) - temp as u16) & 0x80) > 0;
        cpu.zero = cpu.A == temp;
        cpu.carry = cpu.A >= temp;
    };

    let SBC = || {
        let value = mem.read(console, addr);
        let temp: u16 = (0x0100 | cpu.A as u16) - value as u16;
        cpu.A = (temp & 0xFF) as u8;
        cpu.carry = (temp & 0x0100) > 1;
        cpu.overflow = ((cpu.A ^ value) & (cpu.A ^ (temp & 0xFF) as u8)) > 0;
        cpu.negative = (temp & 0x80) > 0;
        cpu.zero = temp == 0;
    };

    match opcode {
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
            ORA();
        }
        0x02 | 0x22 | 0x42 | 0x62 | 0x12 | 0x32 | 0x52 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => { // STP
            panic!("STP inst");
        }
        0x04 | 0x0C | 0x14 | 0x1A | 0x1C | 0x34 | 0x3A | 0x3C | 0x44 | 0x54 | 0x5A | 0x5C => { // NOP

        }
        0x06 | 0x0A | 0x0E | 0x16 | 0x1E => { // ASL

        }
        0x08 => { // PHP
            pushin_p();
        }
        0x10 => { // BPL

        }
        0x18 => { // CLC

        }
        0x20 => { // JSR

        }
        0x21 | 0x25 | 0x29 | 0x2D | 0x31 | 0x35 | 0x39 | 0x3D => { // AND
            AND();
        }
        0x24 | 0x2C => { // BIT

        }
        0x26 | 0x2A | 0x2E | 0x36 | 0x3E => { // ROL

        }
        0x28 => { // PLP
            pullin_p();
        }
        0x30 => { // BMI

        }
        0x38 => { // SEC

        }
        0x40 => { // RTI

        }
        0x41 | 0x45 | 0x49 | 0x4D | 0x51 | 0x55 | 0x59 | 0x5D => {
            EOR();
        }
        0x46 => { // LSR

        }
        0x48 => { // PHA
            push(cpu.A);
        }
        0x4C | 0x6C => { // JMP
            cpu.PC = addr;
        }
        0x5C => { // BVC

        }
        0x58 => { // CLI

        }
        0x60 => { // RTS

        }
        0x61 | 0x65 | 0x69 | 0x6D | 0x71 | 0x75 | 0x79 | 0x7D => {
            ADC();
        }
        0x66 => { // ROR

        }
        0x70 => { // BVS
            
        }
        0x78 => { // SEI

        }
        0x84 => { // STY

        }
        0x88 => { // DEY

        }

    }
}
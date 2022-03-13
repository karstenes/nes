use super::*;

fn pop(console: &mut Console) -> u8 {
    0
}

fn pushin_p(console: &mut Console) {
    let p: u8 = 0;

}

pub fn interpret_opcode(console: &mut Console, opcode: u8) {

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
                    mem.read16(console, (mem.read(console, pc+1)) as u16) + cpu.Y as u16
                }
                0b101 => { // zp,X
                    (mem.read(console, pc+1) as u16 + cpu.X as u16) % 0xFF
                }
                0b110 => { // abs, Y
                    mem.read16(console, pc+1) + (cpu.Y as u16)
                }
                0b111 => { // abs, X
                    mem.read16(console, pc+1) + (cpu.X as u16)
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
            cpu.A |= mem.read(console, addr);
        }
    }
}
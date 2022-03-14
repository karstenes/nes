use super::*;
use memory;

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

pub fn interpret_opcode(console: &mut Console, opcode: u8) {

    if console.CPU.pause > 0 {
        console.CPU.pause -= 1;
        console.CPU.PC += 1;
        return;
    } else {
        console.CPU.pause = cyclecount[opcode as usize] - 1; // lazy (probably should fix the LUT)
    }

    let pc = console.CPU.PC;

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
                        let temp = memory::read16(console, pc+1);
                        memory::read16(console, temp)
                    } else {
                        0
                    }
                }
                0b001 => { // zero page
                    memory::read16(console, pc+1)
                }
                0b011 => { // absolute
                    if opcode == 0x6C {
                        let temp = memory::read16(console, pc+1);
                        memory::read16(console, temp)
                    } else {
                        memory::read16(console, pc+1)
                    }
                }
                0b101 => { // zp indexed x
                    ((memory::read(console, pc+1) + console.CPU.X) as u16) % 0xFF
                }
                0b111 => { // abs indexed x
                    memory::read16(console, pc+1) + (console.CPU.X as u16)
                }
                _ => {
                    0
                }
            }
        }
        0b01 => {
            match bbb {
                0b000 => { // (zp,X)
                    let temp = memory::read(console, pc+1);
                    memory::read16(console, ( temp as u16 + console.CPU.X as u16)% 0xFF) as u16
                }
                0b001 => { // zp
                    memory::read(console, pc+1) as u16
                }
                0b010 => { // immed
                    pc+1
                }
                0b011 => { // abs
                    memory::read16(console, pc+1)
                }
                0b100 => { // (zp), Y
                    let temp = memory::read16(console, pc+1);
                    if ((console.CPU.Y as u16 + (temp & 0xFF)) & 0x100) > 0 {
                        console.CPU.pause += 1;
                    }
                    memory::read16(console, temp as u16) + console.CPU.Y as u16
                }
                0b101 => { // zp,X
                    (memory::read(console, pc+1) as u16 + console.CPU.X as u16) % 0xFF
                }
                0b110 => { // abs, Y
                    let temp = memory::read16(console, pc+1);
                    if ((console.CPU.Y as u16 + (temp & 0xFF)) & 0x100) > 0 {
                        console.CPU.pause += 1;
                    }
                    temp + (console.CPU.Y as u16)
                }
                0b111 => { // abs, X
                    let temp = memory::read16(console, pc+1);
                    if ((console.CPU.X as u16 + (temp & 0xFF)) & 0x100) > 0 {
                        console.CPU.pause += 1;
                    }
                    temp + (console.CPU.X as u16)
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
                    memory::read(console, pc+1) as u16
                }
                0b011 => { // abs
                    memory::read16(console, pc+1)
                }
                0b101 => {
                    if opcode == 0x96 || opcode == 0xB6 {
                        (memory::read(console, pc+1) as u16 + console.CPU.Y as u16) % 0xFF // zp, Y
                    } else {
                        (memory::read(console, pc+1) as u16 + console.CPU.X as u16) % 0xFF // zp, X
                    }
                }
                0b111 => {
                    if opcode == 0x9E || opcode == 0xBE {
                        memory::read16(console, pc+1) + (console.CPU.Y as u16)
                    } else {
                        memory::read16(console, pc+1) + (console.CPU.X as u16)
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
        _ => {
            panic!("unreachable")
        }
    };

    macro_rules! push{
        ($data:expr)=>{
            {
            let temp = console.CPU.SP;
            memory::write(console, temp as u16, $data);
            console.CPU.SP -= 1;
            }
        }
    }

    macro_rules! pull {
        ()=>{
            {
            console.CPU.SP += 1;
            memory::read(console, console.CPU.SP as u16)
            }
        }
    }

    macro_rules! pushin_p {
        ()=>{
        {
        let mut p: u8 = 0x00;
        p |= console.CPU.carry as u8;
        p |= (console.CPU.zero as u8) << 1;
        p |= (console.CPU.interupt_disable as u8) << 2;
        p |= (console.CPU.decimal as u8) << 3;
        p |= (console.CPU.break_cmd as u8) << 4;
        p |= (console.CPU.overflow as u8) << 6;
        p |= (console.CPU.negative as u8) << 7;

        push!(p);
        }
        }
    }

    macro_rules! pullin_p {
        () => {
        {
        let p = pull!();
        console.CPU.carry = (p & 0b00000001) > 0;
        console.CPU.zero = (p & 0b00000010) > 0;
        console.CPU.interupt_disable = (p & 0b00000100) > 0;
        console.CPU.decimal = (p & 0b00001000) > 0;
        console.CPU.overflow = (p & 0b01000000) > 0;
        console.CPU.negative = (p & 0b10000000) > 0;
        }
        }
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


    // ALU ops
    macro_rules! ORA {
        ()=>{
        console.CPU.A |= memory::read(console, addr);
        console.CPU.negative = (console.CPU.A & 0x80) > 0;
        console.CPU.zero = console.CPU.A == 0;
        }
    }

    macro_rules! AND{
        ()=>{
        console.CPU.A &= memory::read(console, addr);
        console.CPU.negative = (console.CPU.A & 0x80) > 0;
        console.CPU.zero = console.CPU.A == 0;
        }
    }

    macro_rules! EOR{
        ()=>{
        console.CPU.A ^= memory::read(console, addr);
        console.CPU.negative = (console.CPU.A & 0x80) > 0;
        console.CPU.zero = console.CPU.A == 0;
        }
    }

   macro_rules! ADC{
        ()=>{
        let value = memory::read(console, addr);
        let temp = console.CPU.A as u16 + value as u16;
        console.CPU.A = (temp & 0x00FF) as u8;
        console.CPU.carry = temp > 0xFF;
        console.CPU.negative = (temp & 0x80) > 0;
        console.CPU.overflow = ((!(console.CPU.A ^ value) & (console.CPU.A ^ (temp & 0x00FF) as u8)) & 0x80) > 0;
        console.CPU.zero = temp == 0;
        }
    }

    macro_rules! STA{
        ()=>{
            memory::write(console, addr, console.CPU.A);
        }
    }

    macro_rules! LDA {
        ()=>{
        console.CPU.A = memory::read(console, addr);
        }
    }

   macro_rules! CMP{
        ($reg:expr)=>{
            {
            let temp = memory::read(console, addr);
            console.CPU.negative = (((0x0100 | $reg as u16) - temp as u16) & 0x80) > 0;
            console.CPU.zero = $reg == temp;
            console.CPU.carry = $reg >= temp;
            }
        }
        
    }

    macro_rules! SBC{
        ()=>{
            {
            let value = memory::read(console, addr);
            let temp: u16 = (0x0100 | console.CPU.A as u16) - value as u16;
            console.CPU.A = (temp & 0xFF) as u8;
            console.CPU.carry = (temp & 0x0100) > 1;
            console.CPU.overflow = ((console.CPU.A ^ value) & (console.CPU.A ^ (temp & 0xFF) as u8)) > 0;
            console.CPU.negative = (temp & 0x80) > 0;
            console.CPU.zero = temp == 0;
            }
        }
    }
    

    match opcode {
        0x00 => { // BRK
            push!(((console.CPU.PC & 0xFF00) >> 8) as u8);
            push!((console.CPU.PC & 0x00FF) as u8);
            console.CPU.break_cmd = true;
            pushin_p!();
            console.CPU.break_cmd = false;
            console.CPU.interupt_disable = true;
            console.CPU.PC = memory::read16(console, 0xFFFE);

        }
        0x01 | 0x05 | 0x09 | 0x0D | 0x11 | 0x15 | 0x19 | 0x1D => { // ORA
            ORA!();
        }
        0x02 | 0x22 | 0x42 | 0x62 | 0x12 | 0x32 | 0x52 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => { // STP
            panic!("STP inst");
        }
        0x04 | 0x0C | 0x14 | 0x1A | 0x1C | 0x34 | 0x3A | 0x3C | 0x44 | 0x54 | 0x5A | 0x5C => { // NOP

        }
        0x06 | 0x0A | 0x0E | 0x16 | 0x1E => { // ASL
            if addr == 0 {
                console.CPU.carry = (console.CPU.A & 0x80) != 0;
                console.CPU.A <<= 1;
                console.CPU.negative = (console.CPU.A & 0x80) != 0;
                console.CPU.zero = console.CPU.A == 0; 
            } else {
                let mut temp = memory::read(console, addr);
                console.CPU.carry = (temp & 0x80) != 0;
                temp <<= 1;
                console.CPU.negative = (temp & 0x80) != 0;
                console.CPU.zero = temp == 0;
                memory::write(console, addr, temp);
            }
        }
        0x08 => { // PHP
            pushin_p!();
        }
        0x10 => { // BPL
            if !console.CPU.negative {
                console.CPU.PC += memory::read(console, addr) as u16;
                return;
            }
        }
        0x18 => { // CLC
            console.CPU.carry = false;
        }
        0x20 => { // JSR

        }
        0x21 | 0x25 | 0x29 | 0x2D | 0x31 | 0x35 | 0x39 | 0x3D => { // AND
            AND!();
        }
        0x24 | 0x2C => { // BIT
            let temp = console.CPU.A & memory::read(console, addr);
            console.CPU.zero = temp == 0;
            console.CPU.negative = (temp & 0x80) != 0;
            console.CPU.overflow = (temp & 0x40) != 0;
        }
        0x26 | 0x2A | 0x2E | 0x36 | 0x3E => { // ROL

        }
        0x28 => { // PLP
            pullin_p!();
        }
        0x30 => { // BMI
            if console.CPU.negative {
                console.CPU.PC += memory::read(console, addr) as u16;
                return;
            }
        }
        0x38 => { // SEC

        }
        0x40 => { // RTI

        }
        0x41 | 0x45 | 0x49 | 0x4D | 0x51 | 0x55 | 0x59 | 0x5D => {
            EOR!();
        }
        0x46 | 0x4A | 0x4E | 0x56 | 0x5E => { // LSR

        }
        0x48 => { // PHA
            push!(console.CPU.A);
        }
        0x4C | 0x6C => { // JMP
            console.CPU.PC = addr;
        }
        0x50 => { // BVC
            if !console.CPU.overflow {
                console.CPU.PC += memory::read(console, addr) as u16;
                return;
            }

        }
        0x58 => { // CLI
            console.CPU.interupt_disable = false;
        }
        0x60 => { // RTS

        }
        0x61 | 0x65 | 0x69 | 0x6D | 0x71 | 0x75 | 0x79 | 0x7D => {
            ADC!();
        }
        0x66 | 0x6A | 0x6E | 0x76 | 0x7E => { // ROR

        }
        0x68 => { // PLA

        }
        0x70 => { // BVS
            if console.CPU.overflow {
                console.CPU.PC += memory::read(console, addr) as u16;
                return;
            }
        }
        0x78 => { // SEI

        }
        0x80 | 0x85 | 0x8D | 0x91 | 0x95 | 0x99 | 0x9D => { // STA

        }
        0x84 | 0x8C | 0x94 => { // STY

        }
        0x86 | 0x8E | 0x96 => { // STX

        }
        0x88 => { // DEY

        }
        0x8A => { // TXA

        }
        0x90 => { // BCC
            if !console.CPU.carry {
                console.CPU.PC += memory::read(console, addr) as u16;
                return;
            }
        }
        0x98 => { // TYA

        }
        0x9A => { // TXS

        }
        0x9C => { // SHY

        }
        0x9E => { // SHX

        }
        0xA0 | 0xA4 | 0xAC | 0xB4 | 0xBC => { // LDY

        }
        0xA1 | 0xA5 | 0xA9 | 0xAD | 0xB1 | 0xB5 | 0xB9 | 0xBD => { // LDA
            LDA!();
        }
        0xA2 | 0xA6 | 0xAE | 0xB6 | 0xBE => { // LDX

        }
        0xA8 => { // TAY

        }
        0xAA => { // TAX

        }
        0xB0 => { // BCS
            if console.CPU.carry {
                console.CPU.PC += memory::read(console, addr) as u16;
                return;
            }
        }
        0xB8 => { // CLV
            console.CPU.overflow = false;
        }
        0xBA => { // TSX

        }
        0xC0 | 0xC4 | 0xCC => { // CPY
            CMP!(console.CPU.Y);
        }
        0xC1 | 0xC5 | 0xC9 | 0xCD | 0xD1 | 0xD5 | 0xD9 | 0xDD => { // CMP
            CMP!(console.CPU.A);
        }
        0xC6 | 0xCE | 0xD6 | 0xDE => { // DEC
            let temp = memory::read(console, addr) - 1;
            console.CPU.negative = (temp & 0x80) != 0;
            console.CPU.zero = temp == 0;
            memory::write(console, addr, temp);
        }
        0xC8 => { // INY

        }
        0xCA => { // DEX
            console.CPU.X -= 1;
            console.CPU.negative = (console.CPU.X & 0x80) != 0;
            console.CPU.zero = console.CPU.X == 0;
        }
        0xD0 => { // BNE
            if !console.CPU.zero {
                console.CPU.PC += memory::read(console, addr) as u16;
                return;
            }
        }
        0xD8 => { // CLD
            console.CPU.decimal = false;
        }
        0xE0 | 0xE4 | 0xEC => { // CPX
            CMP!(console.CPU.X);
        }
        0xE1 | 0xE5 | 0xE9 | 0xED | 0xF1 | 0xF5 | 0xF9 | 0xFD => { // SBC
            SBC!();
        }
        0xE6 | 0xEE | 0xF6 | 0xFE => { // INC

        }
        0xE8 => { // INX

        }
        0xF0 => { // BEQ
            if console.CPU.zero {
                console.CPU.PC += memory::read(console, addr) as u16;
                return;
            }
        }
        0xF8 => { // SED

        }
    }
    console.CPU.PC += 1;
}
#![allow(non_camel_case_types, non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
///
/// 
/// 
/// 
/// 
/// 
/// 


use std::{io::Read, time, thread};

mod mapper;
mod memory;
mod opcodes;
mod ppu;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct CPU {
    A: u8,
    X: u8,
    Y: u8,
    PC: u16,
    SP: u8,
    carry: bool,
    zero: bool,
    interupt_disable: bool,
    decimal: bool,
    break_cmd: bool,
    overflow: bool,
    negative: bool,
    pause: u8,
    jump: bool
}

impl CPU {
    pub fn new() -> Self {
        CPU { A: 0, X: 0, Y: 0, PC: 0xFFFF, SP: 0xFD, carry: false, zero: false, interupt_disable: true, decimal: false, break_cmd: false, overflow: false, negative: false, pause: 0, jump: false } 
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

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut p = "".to_string();
        p.push(if self.negative {'N'} else {'n'});
        p.push(if self.overflow {'V'} else {'v'});
        p.push('S');
        p.push(if self.break_cmd {'B'} else {'b'});
        p.push(if self.decimal {'D'} else {'d'});
        p.push(if self.interupt_disable {'I'} else {'i'});
        p.push(if self.zero {'Z'} else {'z'});
        p.push(if self.carry {'C'} else {'c'});
        write!(f, "A:{:02X} X:{:02X} Y:{:02X} SP:{:02X} P:{}", self.A, self.X, self.Y, self.SP, p)
    }
}



#[derive(Debug)]
pub struct APU {

}

impl APU {
    pub fn new() -> Self {
        APU {}
    }
}



#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct iNES {
    // header
    head: usize,
    PGRSIZE: u8,
    CHRSIZE: u8, // 0 = CHR RAM

    // Flags 6
    mirror: bool, // false = horizontal, true = vertical
    bat_PRGRAM: bool,
    has_trainer: bool,
    ignore_mirror: bool,

    mapper: mapper::Mapper,

    // Flags 7
    unisystem: bool,
    playchoice10: bool,
    nes2: bool,

    // Flags 9
    region: bool, // false: NTSC, true: PAL

    // body
    trainer: Vec<u8>,
    PGR: Vec<u8>,
    CHR: Vec<u8>
}

#[derive(Debug)]
pub struct Console {
    CPU: CPU,
    PPU: ppu::PPU,
    APU: APU,
    Memory: memory::Memory,
    Game: iNES,
    cycles: usize,
}

impl Console {
    pub fn new(game: iNES) -> Console {
        Console{
        CPU: CPU::new(),
        PPU: ppu::PPU::new(),
        APU: APU::new(),
        Memory: memory::Memory::new(),
        Game: game,
        cycles: 0
        }
    }
}

#[allow(non_snake_case)]
fn read_iNES(path: String) -> Result<iNES, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut rom: Vec<u8> = vec![0;file.metadata().unwrap().len() as usize];
    file.read(&mut rom)?;
    
    let mut temp32 = 0;
    for (pos, val) in rom[0..4].iter().cloned().enumerate() {
        temp32 |= (val as u64) << 24 - (pos * 8);
    }
    println!("{:x}", temp32);
    if temp32 != 0x4e45531a {
        panic!("bad header");
    }

    Ok( iNES {
        head: 0x4e45531a,
        PGRSIZE: rom[4],
        CHRSIZE: rom[5],


        mirror: (rom[6] & 0x01) != 0, //
        bat_PRGRAM: (rom[6] & 0x02) != 0,
        has_trainer: (rom[6] & 0x04) != 0,
        ignore_mirror: (rom[6] & 0x08) != 0,
        
        mapper: mapper::Mapper::new((rom[7] & 0xF0) | ((rom[6] & 0xF0)>>4)),

        unisystem: (rom[7] & 0x01) != 0,
        playchoice10: (rom[7] & 0x02) != 0,
        nes2: (rom[7] & 0x04) != 0,

        region: (rom[9] & 0x01) != 0,


        trainer: vec![0;if rom[6] & 0x4 != 0 {8192} else {0}],
        PGR: rom[(16+(if rom[6] & 0x4 != 0 {512} else {0}))..(16+(if rom[6] & 0x4 != 0 {512} else {0})+(rom[4] as usize*16384))].to_vec(),
        CHR: rom[(16+(if rom[6] & 0x4 != 0 {512} else {0})+(rom[4] as usize*16384))..(16+(if rom[6] & 0x4 != 0 {512} else {0})+(rom[4] as usize*16384)+(rom[5] as usize * 8192))].to_vec()
    })
}

fn main() {    
    let dk = read_iNES("dk.nes".to_string()).expect("Read Error");
    let mut nes = Console::new(dk);

    println!("Start Vector 0x{:x}", memory::read16(&mut nes, 0xFFFC));
    nes.CPU.PC = memory::read16(&mut nes, 0xFFFC);
    
    //println!("{:?}", nes.Game.PGR[0..16].as_ref());

    loop {
        let pc = nes.CPU.PC;
        let opcode = memory::read(&mut nes, pc);
        let opcode2 = memory::read(&mut nes, pc+1);
        let opcode3 = memory::read(&mut nes, pc+2);
        if nes.CPU.pause == 0 {
            //println!("{:?}", nes.CPU);
            println!("{:} ${:X}: {:02X} {:02X} {:02X} ", nes.CPU, pc, opcode, opcode2, opcode3);
            
            //thread::sleep(time::Duration::from_millis(1));
        }
        opcodes::interpret_opcode(&mut nes, opcode);

        if nes.cycles%4 == 0 {
            ppu::stepPPU(&mut nes);
            println!("[{}, {}]", nes.PPU.scanline, nes.PPU.cycle);
        }

        nes.cycles += 1
    }    
}

#![allow(non_camel_case_types, non_snake_case)]
///
/// 
/// 
/// 
/// 
/// 
/// 


use std::{io::Read};

use tokio::time;

use memory::Memory;

mod mapper;
mod memory;
mod opcodes;

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
    fn new() -> Self {
        CPU { A: 0, X: 0, Y: 0, PC: 0xFFFF, SP: 0xFD, carry: false, zero: false, interupt_disable: true, decimal: false, break_cmd: false, overflow: false, negative: false, pause: 0, jump: false } 
    }
}

#[derive(Debug)]
struct PPU {
    palette: Vec<u8>,
    nametable: Vec<u8>,
    oam: Vec<u8>,

    // PPUCTRL write 0x2000
    nmi_enable: bool,
    master_slave: bool,
    sprite_height: bool,
    background_tile_select: bool,
    sprite_tile_select: bool,
    increment: bool,
    nametable_select: u8,

    // PPUMASK write 0x2001
    blue_emphasis: bool,
    green_emphasis: bool,
    red_emphasis: bool,
    sprite_enable: bool,
    bg_enable: bool,
    sprite_left_column_enable: bool,
    bg_left_column_enable: bool,
    grayscale: bool,

    // PPUSTATUS  read 0x2002
    vblank: bool,
    s0_hit: bool,
    sprite_overflow: bool,

    oamaddr: usize,


    // regs
    v: u16,
    t: u16,
    x: u8,
    w: bool,
}

impl PPU {
    fn new() -> Self {
        PPU {
            palette: vec![],
            nametable: vec![],
            oam: vec![],

            // PPUCTRL write 0x2000
            nmi_enable: false,
            master_slave: false,
            sprite_height: false,
            background_tile_select: false,
            sprite_tile_select: false,
            increment: false,
            nametable_select: 0,

            // PPUMASK write 0x2001
            blue_emphasis: false,
            green_emphasis: false,
            red_emphasis: false,
            sprite_enable: false,
            bg_enable: false,
            sprite_left_column_enable: false,
            bg_left_column_enable: false,
            grayscale: false,

            // PPUSTATUS  read 0x2002
            vblank: false,
            s0_hit: false,
            sprite_overflow: false,

            oamaddr: 0,


            // regs
            v: 0,
            t: 0,
            x: 0,
            w: false,
        }
    }
}

#[derive(Debug)]
struct APU {

}

impl APU {
    fn new() -> Self {
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
    PPU: PPU,
    APU: APU,
    Memory: memory::Memory,
    Game: iNES,
}

impl Console {
    fn new(game: iNES) -> Console {
        Console{
        CPU: CPU::new(),
        PPU: PPU::new(),
        APU: APU::new(),
        Memory: memory::Memory::new(),
        Game: game
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

async fn tick(nes: &mut Console) {
    let opcode = memory::read(nes, nes.CPU.PC);
    let opcode2 = memory::read(nes, nes.CPU.PC+1);
    let opcode3 = memory::read(nes, nes.CPU.PC+2);
    println!("0x{:x} 0x{:x} 0x{:x}", opcode, opcode2, opcode3);
    opcodes::interpret_opcode(nes, opcode);
    if nes.CPU.pause == 0 {
        println!("{:?}", nes.CPU);
    }
}

#[tokio::main]
async fn main() {    
    let dk = read_iNES("dk.nes".to_string()).expect("Read Error");
    let mut nes = Console::new(dk);

    nes.CPU.PC = memory::read16(&mut nes, 0xFFFC);

    let mut interval = time::interval(time::Duration::from_secs(1));
    
    //println!("{:?}", nes.Game.PGR[0..16].as_ref());

    loop {
        interval.tick().await;
        tick(&mut nes).await;
    }    
}

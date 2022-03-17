#![allow(non_camel_case_types, non_snake_case)]
///
/// 
/// 
/// 
/// 
/// 
/// 


use std::io::Read;

mod mapper;
mod memory;
mod opcodes;

#[allow(non_snake_case)]
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
    interupt: bool,
    pause: u8
}

impl CPU {
    fn new() -> Self {
        CPU {
            A: 0, X: 0, Y: 0, PC: 0, SP: 0, carry: false, zero: false, interupt_disable: false, decimal: false, break_cmd: false, overflow: false,  negative: false, interupt: false, pause: 0
        }
    }
}

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
        unimplemented!();
    }
}

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
    PRG: Vec<u8>,
    CHR: Vec<u8>
}


pub struct Console {
    CPU: CPU,
    PPU: PPU,
    APU: APU,
    Memory: memory::Memory,
    Game: iNES,
}

#[allow(non_snake_case)]
fn read_iNES(path: String) -> Result<iNES, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut rom: Vec<u8> = vec![0;file.metadata().unwrap().len() as usize];
    file.read(&mut rom)?;
    

    /*for (pos, val) in rom[0..4].iter().cloned().enumerate() {
        if val != (0x4e45531a >> 48-(8*pos)) as u8 {
            panic!("bad header");
        }
    }*/

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
        PRG: rom[(16+(if rom[6] & 0x4 != 0 {512} else {0}))..(16+(if rom[6] & 0x4 != 0 {512} else {0})+(rom[4] as usize*16384))].to_vec(),
        CHR: rom[(16+(if rom[6] & 0x4 != 0 {512} else {0})+(rom[4] as usize*16384))..(16+(if rom[6] & 0x4 != 0 {512} else {0})+(rom[4] as usize*16384)+(rom[5] as usize * 8192))].to_vec()
    })
}

fn main() {    
    let dk = read_iNES("dk.nes".to_string()).expect("Read Error");

    print!("{:?}", dk.mapper);
}

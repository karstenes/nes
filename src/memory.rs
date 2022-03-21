use super::*;
use mapper;
use ppu::PPU;

#[derive(Debug)]
pub struct Memory {
    pub ram: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: vec![0;0x800]
        }
    }
}

pub fn read(console: &mut Console, index: u16) -> u8 {
        match index {
        index if index < 0x2000 => {
            console.Memory.ram[(index%0x800) as usize]
        }
        index if index < 0x4000 => {
            readPPU(&mut console.PPU, (index%0x8 + 0x2000) as usize)
        }
        index if index < 0x6000 => {
            unimplemented!();
        }
        index if (index >= 0x6000) => {
            mapper::read(&console.Game, index)
        }
        _ => {
            panic!("bad read addr: 0x{:x}", index);
        }
    }
}

pub fn read16(console: &mut Console, index: u16) -> u16 {
    let hi = read(console, index+1) as u16;
    let lo = read(console, index) as u16;
    println!("hi: {:02X} lo: {:02X}", hi, lo);
    let temp = (hi<<8) | lo;
    temp
}

pub fn write(console: &mut Console, index: u16, data: u8) {
    
    #[cfg(debug_assertions)]
    println!("Writing {:#04X} at {:#06X}", data, index);

    match index {
        index if index < 0x2000 => {
            console.Memory.ram[(index%0x800) as usize] = data;
        }
        index if index < 0x4000 => {
            writePPU(&mut console.PPU, (index%0x8 + 0x2000) as usize, data);
        }
        index if (index >= 0x6000) => {
            mapper::write(&mut console.Game, index, data);
        }
        _ => {
            panic!("bad write addr 0x{:x}", index);
        }
    }
}

pub fn write16(console: &mut Console, index: u16, data: u16) {
    write(console, index+1, ((data & 0xFF00) >> 8) as u8);
    write(console, index, (data & 0x00FF) as u8);
}

fn readPPU(ppu: &mut PPU, index: usize) -> u8 {
    match index {
        0x2002 => {
            ppu.w = false;
            0u8 | ((ppu.vblank as u8) << 7) | ((ppu.s0_hit as u8) << 6) | ((ppu.sprite_overflow as u8) << 5)
        }
        0x2004 => {
            ppu.oam[ppu.oamaddr]
        }
        0x2007 => {
            let temp = 0;
            if ppu.increment {
                ppu.v += 32;
            } else {
                ppu.v += 1;
            }
            temp
        }
        _ => {
            panic!("bad PPU memory location");
        }
    }
}


#[inline(always)]
fn writePPUCTRL(ppu: &mut PPU, data: u8) {
    ppu.nametable_select = data & 0b00000011;
    ppu.increment = (data & 0b00000100) > 0;
    ppu.sprite_tile_select = (data & 0b00001000) > 0;
    ppu.background_tile_select = (data & 0b00010000) > 0;
    ppu.sprite_height = (data & 0b00100000) > 0;
    ppu.master_slave = (data & 0b01000000) > 0;
    ppu.nmi_enable = (data & 0b10000000) > 0;

    ppu.t = (ppu.t & 0xF3FF) | (((data & 0b00000011) as u16) << 10);
}

#[inline(always)]
fn writePPUMASK(ppu: &mut PPU, data: u8) {
    ppu.grayscale = (data & 0b00000001) > 0;
    ppu.bg_left_column_enable = (data & 0b00000010) > 0;
    ppu.sprite_left_column_enable = (data & 0b00000100) > 0;
    ppu.bg_enable = (data & 0b00001000) > 0;
    ppu.sprite_enable = (data & 0b00010000) > 0;
    ppu.red_emphasis = (data & 0b00100000) > 0;
    ppu.green_emphasis = (data & 0b01000000) > 0;
    ppu.blue_emphasis = (data & 0b10000000) > 0;
}

#[inline(always)]
fn writePPUSCROLL(ppu: &mut PPU, data: u8) {
    if !ppu.w {
        ppu.t = (ppu.t & 0xFFE0) | (data as u16 >> 3);
        ppu.x = data & 0b00000111;
        ppu.w = true;
    } else {
        ppu.t = (ppu.t & 0xF3E0) | (((data & 0b00000111) as u16) << 13);
        ppu.t = (ppu.t & 0xFC1F) | (((data & 0b11111000) as u16) << 5);
        ppu.w = false;
    }
}

#[inline(always)]
fn writePPUADDR(ppu: &mut PPU, data: u8) {
    if !ppu.w {
        ppu.t = (ppu.t & 0x80FF) | (((data & 0b00111111) as u16) << 8);
        ppu.w = true;
    } else {
        ppu.t = (ppu.t & 0xFF00) | (data as u16);
        ppu.v = ppu.t;
        ppu.w = false;
    }
}

fn writePPU(ppu: &mut PPU, index: usize, data: u8) {
    match index {
        0x2000 => {
            writePPUCTRL(ppu, data);
        },
        0x2001 => {
            writePPUMASK(ppu, data);
        },
        0x2003 => {
            // write OAMADDR
            ppu.oamaddr = data as usize;
        },
        0x2004 => {
            // write OAMDATA
            ppu.oam[ppu.oamaddr] = data;
            ppu.oamaddr += 1;
        },
        0x2005 => {
            writePPUSCROLL(ppu, data);
        },
        0x2006 => {
            writePPUADDR(ppu, data);
        },
        0x2007 => {
            if ppu.increment {
                ppu.v += 32;
            } else {
                ppu.v += 1;
            }
            writePPUADDR(ppu, data);
        }
        _ => {
            panic!("bad ppu write addr")
        }
    }
}


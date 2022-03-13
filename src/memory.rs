#![allow(non_snake_case)]

use super::*;

pub struct Memory {
    ram: Vec<u8>,
    rom: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            ram: vec![0_u8; 2048],
            rom: vec![0_u8; 32768]
        }
    }

    pub fn read(self, console: &mut Console, index: u16) -> u8 {
        match index {
            index if index < 0x2000 => {
                self.ram[(index%0x800) as usize]
            }
            index if index < 0x4000 => {
                readPPU(&mut console.ppu, (index%0x8 + 0x2000) as usize)
            }
            index if index < 0x8000 => {
                return 0
            }
            _ => {
                panic!("bad read addr");
            }
        }
    }

    pub fn read16(self, console: &mut Console, index: u16) -> u16 {
        ((self.read(console, index+1) as u16) << 8) | (self.read(console, index) as u16)
    }

    pub fn write(self, console: &mut Console, index: u16, data: u8) {
        match index {

            _ => {
                panic!("bad write addr");
            }
        }
    }
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
fn writePPUCTRL(ppu: &mut PPU, index: usize, data: u8) {
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
fn writePPUMASK(ppu: &mut PPU, index: usize, data: u8) {
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
fn writePPUSCROLL(ppu: &mut PPU, index: usize, data: u8) {
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
fn writePPUADDR(ppu: &mut PPU, index: usize, data: u8) {
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
            writePPUCTRL(ppu, index, data);
        },
        0x2001 => {
            writePPUMASK(ppu, index, data);
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
            writePPUSCROLL(ppu, index, data);
        },
        0x2006 => {
            writePPUADDR(ppu, index, data);
        },
        0x2007 => {
            if ppu.increment {
                ppu.v += 32;
            } else {
                ppu.v += 1;
            }
            unimplemented!();
        }
        _ => {
            panic!("bad ppu write addr")
        }
    }
}
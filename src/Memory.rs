use super::*;

pub struct Memory {
    ram: Vec<u8>,
    rom: Vec<u8>,
}

impl Memory {
    pub fn new () -> Self {
        Memory {
            ram: vec![0_u8; 2048],
            rom: vec![0_u8; 32768]
        }
    }

    pub fn read(self, console: &Console, index: usize) -> u8 {
        match index {
            index if index < 0x2000 => {
                self.ram[index%0x800]
            },
            index if index < 0x4000 => {
                readPPU(console.PPU, index%0x8 + 0x2000)
            },
            index if index < 0x8000 => {
                return 0
            }
        }
    }

    pub fn write(self, console: &mut Console) {

    }
}


pub fn readPPU(ppu: &PPU, index: usize) -> u8 {
    match index {
        0x2002 => {
            0
        },
        _ => {
            panic!("bad PPU memory location");
        }
    }
}

pub fn writePPU(ppu: &mut PPU, index: usize, data: u8) {
    match index {
        0x2000 => {
            ppu.nametable_select = data & 0b00000011;
            ppu.increment = (data & 0b00000100) > 0;
            ppu.sprite_tile_select = (data & 0b00001000) > 0;
            ppu.background_tile_select = (data & 0b00010000) > 0;
            ppu.sprite_height = (data & 0b00100000) > 0;
            ppu.master_slave = (data & 0b01000000) > 0;
            ppu.nmi_enable = (data & 0b10000000) > 0;
        },
        0x2001 => {
            ppu.grayscale = (data & 0b00000001) > 0;
            ppu.bg_left_column_enable = (data & 0b00000010) > 0;
            ppu.sprite_left_column_enable = (data & 0b00000100) > 0;
            ppu.bg_enable = (data & 0b00001000) > 0;
            ppu.sprite_enable = (data & 0b00010000) > 0;
            ppu.red_emphasis = (data & 0b00100000) > 0;
            ppu.green_emphasis = (data & 0b01000000) > 0;
            ppu.blue_emphasis = (data & 0b10000000) > 0;
        },
        0x2003 => {
            ppu.oamaddr = data as usize;
        },
        0x2004 => {
            ppu.oam[ppu.oamaddr] = data;
            ppu.oamaddr += 1;
        },
        0x2005 => {
            if !ppu.scrollstate {
                

                ppu.scrollstate = true;
            } else {


                ppu.scrollstate = false;
            }
        }

    }
}


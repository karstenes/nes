use sdl2::{render::{Canvas, Texture}, video::Window};

use super::*;
use memory;

static SYSTEM_PALLETE: [(u8,u8,u8); 64] = [
   (0x80, 0x80, 0x80), (0x00, 0x3D, 0xA6), (0x00, 0x12, 0xB0), (0x44, 0x00, 0x96), (0xA1, 0x00, 0x5E),
   (0xC7, 0x00, 0x28), (0xBA, 0x06, 0x00), (0x8C, 0x17, 0x00), (0x5C, 0x2F, 0x00), (0x10, 0x45, 0x00),
   (0x05, 0x4A, 0x00), (0x00, 0x47, 0x2E), (0x00, 0x41, 0x66), (0x00, 0x00, 0x00), (0x05, 0x05, 0x05),
   (0x05, 0x05, 0x05), (0xC7, 0xC7, 0xC7), (0x00, 0x77, 0xFF), (0x21, 0x55, 0xFF), (0x82, 0x37, 0xFA),
   (0xEB, 0x2F, 0xB5), (0xFF, 0x29, 0x50), (0xFF, 0x22, 0x00), (0xD6, 0x32, 0x00), (0xC4, 0x62, 0x00),
   (0x35, 0x80, 0x00), (0x05, 0x8F, 0x00), (0x00, 0x8A, 0x55), (0x00, 0x99, 0xCC), (0x21, 0x21, 0x21),
   (0x09, 0x09, 0x09), (0x09, 0x09, 0x09), (0xFF, 0xFF, 0xFF), (0x0F, 0xD7, 0xFF), (0x69, 0xA2, 0xFF),
   (0xD4, 0x80, 0xFF), (0xFF, 0x45, 0xF3), (0xFF, 0x61, 0x8B), (0xFF, 0x88, 0x33), (0xFF, 0x9C, 0x12),
   (0xFA, 0xBC, 0x20), (0x9F, 0xE3, 0x0E), (0x2B, 0xF0, 0x35), (0x0C, 0xF0, 0xA4), (0x05, 0xFB, 0xFF),
   (0x5E, 0x5E, 0x5E), (0x0D, 0x0D, 0x0D), (0x0D, 0x0D, 0x0D), (0xFF, 0xFF, 0xFF), (0xA6, 0xFC, 0xFF),
   (0xB3, 0xEC, 0xFF), (0xDA, 0xAB, 0xEB), (0xFF, 0xA8, 0xF9), (0xFF, 0xAB, 0xB3), (0xFF, 0xD2, 0xB0),
   (0xFF, 0xEF, 0xA6), (0xFF, 0xF7, 0x9C), (0xD7, 0xE8, 0x95), (0xA6, 0xED, 0xAF), (0xA2, 0xF2, 0xDA),
   (0x99, 0xFF, 0xFC), (0xDD, 0xDD, 0xDD), (0x11, 0x11, 0x11), (0x11, 0x11, 0x11)
];

#[derive(Debug, Clone)]
struct image {
    data: Vec<u8>
}


impl image {
    fn new(width: usize, height: usize) -> image {
        image { data: vec![0;width*height*3] }
    }
    fn write(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let index = (y * 3 * 256) + (x*3);
        self.data[index] = rgb.0;
        self.data[index+1] = rgb.1;
        self.data[index+2] = rgb.2;
    }
}

#[derive(Debug)]
pub struct PPU {
    frame: image,

    pub palette: Vec<u8>,
    pub nametable: Vec<u8>,
    pub oam: Vec<u8>,

    nametablebyte: u8,


    // PPUCTRL write 0x2000
    pub nmi_enable: bool,
    pub master_slave: bool,
    pub sprite_height: bool,
    pub background_tile_select: bool,
    pub sprite_tile_select: bool,
    pub increment: bool,
    pub nametable_select: u8,

    // PPUMASK write 0x2001
    pub blue_emphasis: bool,
    pub green_emphasis: bool,
    pub red_emphasis: bool,
    pub sprite_enable: bool,
    pub bg_enable: bool,
    pub sprite_left_column_enable: bool,
    pub bg_left_column_enable: bool,
    pub grayscale: bool,

    // PPUSTATUS  read 0x2002
    pub vblank: bool,
    pub s0_hit: bool,
    pub sprite_overflow: bool,

    pub oamaddr: usize,


    // regs
    pub v: u16,
    pub t: u16,
    pub x: u8,
    pub w: bool,

    pub scanline: usize,
    pub cycle: usize
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            frame: image::new(256, 240),

            palette: vec![],
            nametable: vec![],
            oam: vec![],

            nametablebyte: 0,

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

            scanline: 0,
            cycle: 0,
        }
    }
}

pub fn stepPPU(console: &mut Console, canvas: &mut Canvas<Window>, Texture: &mut Texture) {
    let scanline = console.PPU.scanline;
    let cycle = console.PPU.cycle;
    match scanline {
        line if line == 261 => {
            match cycle {
                cycle if cycle == 0 => {
                    canvas.copy(&Texture, None, None);
                    canvas.present();
                }
                cycle if cycle < 257 => {

                }
                cycle if cycle < 321 => {

                }
                cycle if cycle < 337 => {

                }
                cylce if cycle <= 340 => {

                }
                _ => {
                    panic!("bad cycle number {}", cycle);
                }
            };
        }
        line if line < 240 => {
            match cycle {
                cycle if cycle == 0 => {
                    
                }
                cycle if cycle < 257 => {
                    let addr: u16 = (0x2000 + cycle/8 + line/8).try_into().unwrap();
                    let char = memory::read(console, addr);
                    println!("{:02X}", char);
                    //let pixel = memory:read(console, char+)

                }
                cycle if cycle < 321 => {

                }
                cycle if cycle < 337 => {

                }
                cylce if cycle <= 340 => {

                }
                _ => {
                    panic!("bad cycle number {}", cycle);
                }
            };
        }
        line if line == 241 && cycle == 1 => {
            console.PPU.vblank = true;
        }
        line if line <= 260 => {
            // do nothing
        }
        _ => {
            panic!("bad scanline number {}", scanline);
        }
    };

    if cycle < 340 {
        console.PPU.cycle += 1;
    } else {
        if scanline < 260 {
            console.PPU.scanline += 1;
            console.PPU.cycle = 0;
        } else {
            console.PPU.scanline = 0;
            console.PPU.cycle = 0;
        }
    }
    
}
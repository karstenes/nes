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

    pub patterntable0: Vec<u8>,
    pub patterntable1: Vec<u8>,
    pub nametable0: Vec<u8>,
    pub nametable1: Vec<u8>,
    pub nametable2: Vec<u8>,
    pub nametable3: Vec<u8>,
    pub palette: Vec<u8>,
    pub oam: Vec<u8>,

    pub oam_transfer: bool,

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

    // P``TATUS  read 0x2002
    pub vblank: bool,
    pub s0_hit: bool,
    pub sprite_overflow: bool,

    pub oamaddr: usize,
    oamdmaaddr: usize,
    pub oamdmapage: u8,

    pub scroll_lowwrite: bool,
    pub addr_lowwrite: bool,

    // regs
    pub scroll: u16,
    pub addr: u16,

    pub v: u16,
    pub t: u16,
    pub x: u8,
    pub w: bool,

    pub scanline: usize,
    pub cycle: usize,

    pub nmi_occured: bool
}

impl PPU {
    pub fn new(chr: Vec<u8>) -> Self {
        let mut ppu = PPU {
            frame: image::new(256, 240),

            patterntable0: vec![0; 0x1000],
            patterntable1: vec![0; 0x1000],
            nametable0: vec![0; 0x400],
            nametable1: vec![0; 0x400],
            nametable2: vec![0; 0x400],
            nametable3: vec![0; 0x400],
            palette: vec![0; 0x20],
            oam: vec![0; 256],

            oam_transfer: false,

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
            vblank: true,
            s0_hit: false,
            sprite_overflow: false,

            oamaddr: 0,
            oamdmaaddr: 0,
            oamdmapage: 0,

            scroll_lowwrite: false,
            addr_lowwrite: false,

            // regs
            scroll: 0,
            addr: 0,

            v: 0,
            t: 0,
            x: 0,
            w: false,

            scanline: 0,
            cycle: 0,

            nmi_occured: false
        };
        ppu.patterntable0.copy_from_slice(&chr[0..0x1000]);
        ppu.patterntable1.copy_from_slice(&chr[0x1000..0x2000]);
        ppu
    }
}

pub fn stepPPU(console: &mut Console, canvas: &mut Canvas<Window>, Texture: &mut Texture) {
    let scanline = console.PPU.scanline;
    let cycle = console.PPU.cycle;
    

    if console.PPU.oam_transfer {
        let newdata = memory::read(console, (((console.PPU.oamdmapage as u16)&0x00FF)<<8) | console.PPU.oamdmaaddr as u16);
        console.PPU.oam[console.PPU.oamdmaaddr] = newdata;
        //println!("OAM DMA tranfer of {:02X} from {:04X} to {:02X}", newdata, (((console.PPU.oamdmapage as u16)&0x00FF)<<8) | console.PPU.oamdmaaddr as u16, console.PPU.oamdmaaddr);
        console.PPU.oamdmaaddr += 1;
        if console.PPU.oamdmaaddr > 0xFF {
            console.PPU.oam_transfer = false;
            console.PPU.oamdmaaddr = 0;
        }
    }
    match scanline {
        line if line == 261 => {
            match cycle {
                cycle if cycle == 0 => {
                    console.PPU.vblank = false;
                    console.PPU.nmi_occured = false;
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

                    //let addr: u16 = (0x2000 + cycle/8 + line/8).try_into().unwrap();
                    // let char = memory::readPPUADDR(&mut console.PPU, addr as usize);
                    let tile = console.PPU.nametable0[cycle/8 + (scanline/8)*32];
                    //println!("line: {}, cycle: {}, {:02X} at {:X}", scanline, cycle, char, cycle/8 + scanline*4);
                    let char = &console.PPU.patterntable0[(tile * 16) as usize..=(tile * 16 + 15) as usize];
                    // let pixellow = console.PPU.patterntable0[(tile as usize * 16) + line%8] >> ((cycle - 1) % 8);
                    // let pixelhigh = console.PPU.patterntable0[(tile as usize * 16) + 8 + line%8] >> ((cycle - 1) % 8);
                    let low = char[line%8];
                    let high = char[line%8 + 8];

                    let pixellow = low>>((cycle-1)%8);
                    let pixelhigh = high>>((cycle-1)%8);

                    let value = ((pixelhigh & 0b1)<< 1) | (pixellow & 0b1);
                    let rgb = match value {
                        0 => SYSTEM_PALLETE[0x01],
                        1 => SYSTEM_PALLETE[0x23],
                        2 => SYSTEM_PALLETE[0x27],
                        3 => SYSTEM_PALLETE[0x30],
                        _ => panic!("can't be"),
                    };
                    //println!("Rendering x:{}, y:{}, color:{:?}", cycle-1, line, rgb);
                    console.PPU.frame.write(cycle-1, line, rgb);
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
            console.PPU.nmi_occured = true;

            //println!("rendering frame!");
            println!("{:?}", console.PPU.nametable0);
            //println!("{:?}", console.PPU.frame.data);
            Texture.update(None, &console.PPU.frame.data, 256*3).unwrap();
            canvas.copy(&Texture, None, None).unwrap();
            canvas.present();
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
        if scanline < 261 {
            console.PPU.scanline += 1;
            console.PPU.cycle = 0;
        } else {
            console.PPU.scanline = 0;
            console.PPU.cycle = 0;
        }
    }
    
}
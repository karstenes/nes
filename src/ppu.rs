use super::*;

#[derive(Debug, Clone, Copy)]
struct pixel {
    r: u8,
    g: u8,
    b: u8
}

impl pixel {
    fn new() -> pixel {
        pixel {r: 0, g: 0, b: 0}
    }
}

struct image {
    im: Vec<Vec<pixel>>
}


impl image {
    fn new(width: usize, height: usize) -> image {
        image { im: vec![vec![pixel::new(); width]; height] }
    }
    fn read(&self, x: usize, y: usize) -> &pixel {
        &self.im[y][x]
    }
    fn write(&mut self, x: usize, y: usize, value: pixel) {
        self.im[y][x] = value;
    }
}

#[derive(Debug)]
pub struct PPU {
    pub palette: Vec<u8>,
    pub nametable: Vec<u8>,
    pub oam: Vec<u8>,

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

            scanline: 0,
            cycle: 0,
        }
    }
}

pub fn stepPPU(console: &mut Console) {
    let scanline = console.PPU.scanline;
    let cycle = console.PPU.cycle;
    match scanline {
        line if line == 261 => {
            match cycle {
                cycle if cycle == 0 => {

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
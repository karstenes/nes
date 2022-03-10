///
/// 
/// 
/// 
/// 
/// 
/// 

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
    pause: u64
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

pub struct Console {
    CPU: CPU,
    PPU: PPU,
    APU: APU,
    Memory: memory::Memory
}

fn main() {
    let mut console = Console {CPU: CPU::new(), PPU: PPU::new(), APU: APU::new(), Memory: memory::Memory::new()};
        
}

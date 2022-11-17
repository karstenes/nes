use super::*;
use mapper;
use ppu::PPU;
use apu::APU;

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
            if index == 0xFD {println!("Read from 0xFD: {:02X}", console.Memory.ram[(index%0x800) as usize])};
            console.Memory.ram[(index%0x800) as usize]
        }
        index if index < 0x4000 => {
            readPPU(&mut console.PPU, (index%0x8 + 0x2000) as usize)
        }
        index if index == 0x4016 || index == 0x4017 => {
            println!("controlers unimplemented");
            0
        }
        index if index < 0x4016 => {
            readAPU(&console.APU, index)
        }
        index if index < 0x6000 => {
            panic!("Unimplemented test I/O functionality");
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
    (hi<<8) | lo
}

pub fn write(console: &mut Console, index: u16, data: u8) {
    
    #[cfg(debug_prints)]
    println!("Writing {:#04X} at {:#06X}", data, index);

    match index {
        index if index < 0x2000 => {
            if index == 0xFD {println!("Write to 0xFD: {:02X}", data)};
            console.Memory.ram[(index%0x800) as usize] = data;
        }
        index if index == 0x4016 || index == 0x4017 => {
            println!("controlers unimplemented");
        }
        index if index < 0x4000 => {
            writePPU(&mut console.PPU, (index%0x8 + 0x2000) as usize, data);
        }
        index if index == 0x4014 => {
            writePPU(&mut console.PPU, index as usize, data);
        }
        index if index < 0x4016 => {
            writeAPU(&mut console.APU, index, data);
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

#[inline(always)]
pub fn readPPUADDR(ppu: &mut PPU, index: usize) -> u8 {
    match index {
        index if index < 0x1000 => {
            ppu.patterntable0[index]
        },
        index if index < 0x2000 => {
            ppu.patterntable1[(index % 0x1000)]
        },
        index if index < 0x2400 => {
            ppu.nametable0[(index % 0x400)]
        },
        index if index < 0x2800 => {
            ppu.nametable1[(index % 0x400)]
        },
        index if index < 0x2C00 => {
            ppu.nametable2[(index % 0x400)]
        },
        index if index < 0x3000 => {
            ppu.nametable3[(index % 0x400)]
        }
        index if index < 0x3400 => {
            ppu.nametable0[(index % 0x400)]
        },
        index if index < 0x3800 => {
            ppu.nametable1[(index % 0x400)]
        },
        index if index < 0x3C00 => {
            ppu.nametable2[(index % 0x400)]
        },
        index if index < 0x3F00 => {
            ppu.nametable3[(index % 0x400)]
        }
        index if index < 0x4000 => {
            ppu.palette[(index % 0x20)]
        },
        _ => panic!("Bad PPU mem read addr ${:04X}", ppu.addr)
    }
}

pub fn readPPU(ppu: &mut PPU, index: usize) -> u8 {
    match index {
        //index if index 
        0x2002 => {
            ppu.w = false;
            let val = 0u8 | ((ppu.vblank as u8) << 7) | ((ppu.s0_hit as u8) << 6) | ((ppu.sprite_overflow as u8) << 5);
            ppu.vblank = false;
            val
        }
        0x2004 => {
            ppu.oam[ppu.oamaddr]
        }
        0x2007 => {
            let data = readPPUADDR(ppu, index);
            if ppu.increment {
                ppu.addr += 32;
            } else {
                ppu.addr += 1;
            }
            data
        }
        _ => {
            panic!("bad PPU memory location: {:02X}", index);
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
fn writePPUADDR(ppu: &mut PPU, data: u8, index: usize) {
    println!("PPUMEM Write ${:04X}: 0x{:02X}", index, data);
    match index {
        index if index < 0x1000 => {
            ppu.patterntable0[index] = data;
        },
        index if index < 0x2000 => {
            ppu.patterntable1[(index % 0x1000)] = data;
        },
        index if index < 0x2400 => {
            ppu.nametable0[(index % 0x400)] = data;
        },
        index if index < 0x2800 => {
            ppu.nametable1[(index % 0x400)] = data;
        },
        index if index < 0x2C00 => {
            ppu.nametable2[(index % 0x400)] = data;
        },
        index if index < 0x3000 => {
            ppu.nametable3[(index % 0x400)] = data;
        }
        index if index < 0x3400 => {
            ppu.nametable0[(index % 0x400)] = data;
        },
        index if index < 0x3800 => {
            ppu.nametable1[(index % 0x400)] = data;
        },
        index if index < 0x3C00 => {
            ppu.nametable2[(index % 0x400)] = data;
        },
        index if index < 0x3F00 => {
            ppu.nametable3[(index % 0x400)] = data;
        }
        index if index < 0x4000 => {
            ppu.palette[(index % 0x20)] = data;
        },
        _ => panic!("Bad PPU mem write addr ${:04X}", ppu.addr)
    }
}

pub fn writePPU(ppu: &mut PPU, index: usize, data: u8) {
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
            if ppu.addr_lowwrite {
                ppu.addr = (ppu.addr & 0xFF00) | data as u16;
            } else {
                ppu.addr = (ppu.addr & 0x00FF) | ((data as u16) << 8);
            }
            ppu.addr = ppu.addr % 0x4000;
            ppu.addr_lowwrite = !ppu.addr_lowwrite;
        },
        0x2007 => {
            println!("Writing {:02X} to PPU addr {:04X}", data, ppu.addr);
            if ppu.addr < 0x4000 {writePPUADDR(ppu, data, ppu.addr as usize)};
            if ppu.increment {
                ppu.addr += 32;
            } else {
                ppu.addr += 1;
            }
        }
        0x4014 => {
            ppu.oam_transfer = true;
        }
        _ => {
            panic!("bad ppu write addr ${:04X}", index)
        }
    }
}

fn readAPU(apu: &APU, index: u16) -> u8 {
    match index {
        _ => {
            panic!("Bad APU memory read ${:04X}", index);
        }

    }
}

fn writeAPU(apu: &mut APU, index: u16, data: u8) {
    match index {
        0x4000 => {
            apu.Pulse1.duty = (data & 0b11000000) >> 6;
            apu.Pulse1.halt = (data & 0b00100000) != 0;
            apu.Pulse1.volume = data & 0x0F;
        }
        0x4004 => {
            apu.Pulse2.duty = (data & 0b11000000) >> 6;
            apu.Pulse2.halt = (data & 0b00100000) != 0;
            apu.Pulse2.volume = data & 0x0F;
        }
        0x4001 => {
            apu.Pulse1.sweep_en = (data & 0b10000000) != 0;
            apu.Pulse1.sweep_period = (data & 0b01110000) >> 4;
            apu.Pulse1.sweep_negate = (data & 0b00001000) != 0;
            apu.Pulse1.sweep_shift = data & 0b00000111;
        }
        0x4005 => {
            apu.Pulse2.sweep_en = (data & 0b10000000) != 0;
            apu.Pulse2.sweep_period = (data & 0b01110000) >> 4;
            apu.Pulse2.sweep_negate = (data & 0b00001000) != 0;
            apu.Pulse2.sweep_shift = data & 0b00000111;
        }
        0x4002 => {
            apu.Pulse1.timer = (apu.Pulse1.timer & 0xFF00) | (data as u16);
        }
        0x4006 => {
            apu.Pulse2.timer = (apu.Pulse2.timer & 0xFF00) | (data as u16);
        }
        0x4003 => {
            apu.Pulse1.timer = (apu.Pulse1.timer & 0x00FF) | (((data & 0b00000111) as u16) << 8);
            apu.Pulse1.length_counter_load = (data & 0b11111000) >> 3;
        }
        0x4007 => {
            apu.Pulse2.timer = (apu.Pulse2.timer & 0x00FF) | (((data & 0b00000111) as u16) << 8);
            apu.Pulse2.length_counter_load = (data & 0b11111000) >> 3;
        }
        0x4008 => {
            apu.Triangle.control = (data & 0b10000000) != 0;
            apu.Triangle.counter_reload = data & 0b01111111;
        }
        0x400A => {
            apu.Triangle.timer = (apu.Triangle.timer & 0xFF00) | (data as u16);
        }
        0x400B => {
            apu.Triangle.timer = (apu.Triangle.timer & 0x00FF) | (((data & 0b00000111) as u16) << 8);
            apu.Triangle.length_counter_load = (data & 0b11111000) >> 3;
        }
        0x400C => {
            apu.Noise.halt = (data & 0b00100000) != 0;
            apu.Noise.constant_volume = (data & 0b00010000) != 0;
            apu.Noise.volume = data & 0x0F;
        }
        0x400E => {
            apu.Noise.mode = (data & 0b10000000) != 0;
            apu.Noise.period = data & 0x0F;
        }
        0x400F => {
            apu.Noise.length_counter_load = (data & 0b11111000) >> 3;
        }
        0x4010 => {
            unimplemented!("DMC $4010");
        }
        0x4011 => {
            unimplemented!("DMC $4011");
        }
        0x4012 => {
            unimplemented!("DMC $4012");
        }
        0x4013 => {
            unimplemented!("DMC $4013");
        }
        0x4015 => {
            apu.dmc_en = (data & 0b00010000) != 0;
            apu.noise_en = (data & 0b00001000) != 0;
            apu.triangle_en = (data & 0b00000100) != 0;
            apu.pulse2_en = (data & 0b00000010) != 0;
            apu.pulse1_en = (data & 0b00000001) != 0;
        }
        0x4017 => {
            apu.fc_mode = (data & 0b10000000) != 0;
            apu.irq_inhibit = (data & 0b01000000) != 0;
        }
        _ => {
            panic!("Bad APU memory write of {:#04X} to ${:04X}", data, index);
        }
        
    };
}


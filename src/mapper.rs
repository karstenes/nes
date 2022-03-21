use super::*;

#[derive(Debug)]
pub enum Mapper {
    Map1,
    Map2(MMC1),
    Map3(UxROM),
}

impl Mapper {
    pub fn new(mapper_type: u8) -> Mapper {
        match mapper_type {
            0 => {
                Mapper::Map1
            }
            1 => {
                Mapper::Map2(MMC1{})
            }
            2 => {
                Mapper::Map3(UxROM{})
            }
            _ => {
                panic!("unknown mapper");
            }
        }
    }
}

#[derive(Debug)]
pub struct MMC1 {

}

#[derive(Debug)]
pub struct UxROM {

}

pub fn read(cart: &iNES, index: u16) -> u8 {
    match &cart.mapper {
        Mapper::Map1 => {
            match index {
                index if (index >= 0x6000) && (index < 0x8000) => {
                    0
                }
                index if (index >= 0x8000) && (index < 0xC000) => {
                    cart.PGR[(index % 0x4000) as usize]
                }
                index if (index >= 0xC000) => {
                    if cart.PGRSIZE == 1 {
                        cart.PGR[(index % 0x4000) as usize]
                    } else {
                        cart.PGR[(index % 0x4000 + 0x4000) as usize]
                    }
                }
                _ => {
                    panic!("Mapper: Unknown address ${:02x}", index);
                }
            }
        }
        Mapper::Map2(map) => {
            match index {
                index if index < 0x2000 => {
                    0
                }
                index if (index >= 0x6000) && (index < 0x8000) => {
                    0
                }
                index if (index >= 0x8000) && (index < 0xC000) => {
                    0
                }
                index if (index >= 0xC000) && (index < 0xFFFF) => {
                    0
                }
                _ => {
                    panic!("Bad mapper address {}", index);
                }
            }
        }
        _ => {
            unimplemented!("Unknown mapper {:?}", &cart.mapper);
        }
    }
}

pub fn write(cart: &mut iNES, index: u16, data: u8) {
    match &cart.mapper {
        Mapper::Map1 => {
            panic!("Bad write to NROM cart (No writeable memory)");
        }
        _ => {
            panic!("Unimplemented mapper {:?}", cart.mapper);
        }
    }
}
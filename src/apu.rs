use super::*;

#[derive(Debug)]
pub struct Pulse {
    pub duty: u8,
    pub halt: bool,
    pub constant_volume: bool,
    pub volume: u8,

    pub sweep_en: bool,
    pub sweep_period: u8,
    pub sweep_negate: bool,
    pub sweep_shift: u8,

    pub timer: u16,
    pub length_counter_load: u8,
}

impl Pulse {
    fn new() -> Pulse {
        Pulse { duty: 0,
            halt: false,
            constant_volume: false,
            volume: 0,
            sweep_en: false,
            sweep_period: 0,
            sweep_negate: false,
            sweep_shift: 0,
            timer: 0,
            length_counter_load: 0 
        }
    }
}

#[derive(Debug)]

pub struct Triangle {
    pub control: bool,
    pub counter_reload: u8,
    pub timer: u16,
    pub length_counter_load: u8,
}

impl Triangle {
    fn new() -> Triangle {
        Triangle { 
            control: false, 
            counter_reload: 0, 
            timer: 0, 
            length_counter_load: 0
        }
    }
}

#[derive(Debug)]
pub struct Noise {
    pub halt: bool,
    pub constant_volume: bool,
    pub volume: u8,
    pub mode: bool,
    pub period: u8,
    pub length_counter_load: u8,
}

impl Noise {
    fn new() -> Noise {
        Noise { 
            halt: false, 
            constant_volume: false, 
            volume: 0, 
            mode: false, 
            period: 0, 
            length_counter_load: 0 
        }
    }
}

#[derive(Debug)]
pub struct DMC {
    pub irq_en: bool,
    pub loop_flag: bool,
    pub rate_index: u8,
    pub direct_load: u8,
    pub sample_address: u8,
    pub sample_length: u8,
}

impl DMC {
    fn new() -> DMC {
        DMC { 
            irq_en: false, 
            loop_flag: false, 
            rate_index: 0, 
            direct_load: 0, 
            sample_address: 0, 
            sample_length: 0 
        }
    }
}


#[derive(Debug)]
pub struct APU {
    pub dmc_en: bool,
    pub noise_en: bool,
    pub triangle_en: bool,
    pub pulse1_en: bool,
    pub pulse2_en: bool,
    pub fc_mode: bool,
    pub irq_inhibit: bool,

    pub Pulse1: Pulse,
    pub Pulse2: Pulse,
    pub Triangle: Triangle,
    pub Noise: Noise,
    pub DMC: DMC,
}

impl APU {
    pub fn new() -> Self {
        APU {
            dmc_en: false,
            noise_en: false,
            triangle_en: false,
            pulse1_en: false,
            pulse2_en: false,
            fc_mode: false,
            irq_inhibit: false,

            Pulse1: Pulse::new(),
            Pulse2: Pulse::new(),
            Triangle: Triangle::new(),
            Noise: Noise::new(),
            DMC: DMC::new(),
        }
    }
}


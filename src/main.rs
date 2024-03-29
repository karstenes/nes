#![allow(non_camel_case_types, non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]
///
/// 
/// 
/// 
/// 
/// 
/// 

static OPNAMES: &[&str] = &["BRK", "ORA", "STP", "SLO", "NOP", "ORA", "ASL", "SLO", "PHP", "ORA", "ASL", "ANC", "NOP", "ORA", "ASL", "SLO", "BPL", "ORA", "STP", "SLO", "NOP", "ORA", "ASL", "SLO", "CLC", "ORA", "NOP", "SLO", "NOP", "ORA", "ASL", "SLO", "JSR", "AND", "STP", "RLA", "BIT", "AND", "ROL", "RLA", "PLP", "AND", "ROL", "ANC", "BIT", "AND", "ROL", "RLA", "BMI", "AND", "STP", "RLA", "NOP", "AND", "ROL", "RLA", "SEC", "AND", "NOP", "RLA", "NOP", "AND", "ROL", "RLA", "RTI", "EOR", "STP", "SRE", "NOP", "EOR", "LSR", "SRE", "PHA", "EOR", "LSR", "ALR", "JMP", "EOR", "LSR", "SRE", "BVC", "EOR", "STP", "SRE", "NOP", "EOR", "LSR", "SRE", "CLI", "EOR", "NOP", "SRE", "NOP", "EOR", "LSR", "SRE", "RTS", "ADC", "STP", "RRA", "NOP", "ADC", "ROR", "RRA", "PLA", "ADC", "ROR", "ARR", "JMP", "ADC", "ROR", "RRA", "BVS", "ADC", "STP", "RRA", "NOP", "ADC", "ROR", "RRA", "SEI", "ADC", "NOP", "RRA", "NOP", "ADC", "ROR", "RRA", "NOP", "STA", "NOP", "SAX", "STY", "STA", "STX", "SAX", "DEY", "NOP", "TXA", "XAA", "STY", "STA", "STX", "SAX", "BCC", "STA", "STP", "AHX", "STY", "STA", "STX", "SAX", "TYA", "STA", "TXS", "TAS", "SHY", "STA", "SHX", "AHX", "LDY", "LDA", "LDX", "LAX", "LDY", "LDA", "LDX", "LAX", "TAY", "LDA", "TAX", "LAX", "LDY", "LDA", "LDX", "LAX", "BCS", "LDA", "STP", "LAX", "LDY", "LDA", "LDX", "LAX", "CLV", "LDA", "TSX", "LAS", "LDY", "LDA", "LDX", "LAX", "CPY", "CMP", "NOP", "DCP", "CPY", "CMP", "DEC", "DCP", "INY", "CMP", "DEX", "AXS", "CPY", "CMP", "DEC", "DCP", "BNE", "CMP", "STP", "DCP", "NOP", "CMP", "DEC", "DCP", "CLD", "CMP", "NOP", "DCP", "NOP", "CMP", "DEC", "DCP", "CPX", "SBC", "NOP", "ISC", "CPX", "SBC", "INC", "ISC", "INX", "SBC", "NOP", "SBC", "CPX", "SBC", "INC", "ISC", "BEQ", "SBC", "STP", "ISC", "NOP", "SBC", "INC", "ISC", "SED", "SBC", "NOP", "ISC", "NOP", "SBC", "INC", "ISC"];


use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::libc::CLOCK_REALTIME_ALARM;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;

use std::error::Error;
use std::{io::Read, time, thread, iter::Inspect, io::{stdin, stdout, Write}};

mod mapper;

mod memory;
mod opcodes;
mod ppu;
mod apu;
mod controller;

use apu::APU;
use ppu::PPU;
use controller::Controller;

use crate::memory::Memory;

#[allow(non_snake_case)]
#[derive(Debug)]
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
    pause: u8,
    jump: bool
}

impl CPU {
    pub fn new() -> Self {
        CPU { A: 0, X: 0, Y: 0, PC: 0xFFFF, SP: 0xFD, carry: false, zero: false, interupt_disable: true, decimal: false, break_cmd: false, overflow: false, negative: false, pause: 0, jump: false } 
    }
}

/*
    ---- ----
    NVss DIZC
    |||| ||||
    |||| |||+- Carry
    |||| ||+-- Zero
    |||| |+--- Interrupt Disable
    |||| +---- Decimal
    ||++------ No CPU effect, see: the B flag
    |+-------- Overflow
    +--------- Negative
    */

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut p = "".to_string();
        p.push(if self.negative {'N'} else {'n'});
        p.push(if self.overflow {'V'} else {'v'});
        p.push('s');
        p.push(if self.break_cmd {'B'} else {'b'});
        p.push(if self.decimal {'D'} else {'d'});
        p.push(if self.interupt_disable {'I'} else {'i'});
        p.push(if self.zero {'Z'} else {'z'});
        p.push(if self.carry {'C'} else {'c'});
        write!(f, "A:{:02X} X:{:02X} Y:{:02X} SP:{:02X} P:{}", self.A, self.X, self.Y, self.SP, p)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct iNES {
    // header
    head: usize,
    PGRSIZE: u8,
    CHRSIZE: u8, // 0 = CHR RAM

    // Flags 6
    mirror: bool, // false = horizontal, true = vertical
    bat_PRGRAM: bool,
    has_trainer: bool,
    ignore_mirror: bool,

    mapper: mapper::Mapper,

    // Flags 7
    unisystem: bool,
    playchoice10: bool,
    nes2: bool,

    // Flags 9
    region: bool, // false: NTSC, true: PAL

    // body
    trainer: Vec<u8>,
    PGR: Vec<u8>,
    CHR: Vec<u8>
}

#[derive(Debug)]
pub struct Console {
    CPU: CPU,
    PPU: ppu::PPU,
    APU: APU,
    Memory: memory::Memory,
    Game: iNES,
    Controller1: Controller,
    Controller2: Controller,
    Strobe: bool,
    cycles: usize,
}

impl Console {
    pub fn new(game: iNES) -> Console {
        Console{
        CPU: CPU::new(),
        PPU: PPU::new(game.CHR.clone()),
        APU: APU::new(),
        Memory: memory::Memory::new(),
        Game: game,
        Controller1: Controller::new(),
        Controller2: Controller::new(),
        Strobe: false,
        cycles: 0
        }
    }
}

#[allow(non_snake_case)]
fn read_iNES(path: String) -> Result<iNES, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut rom: Vec<u8> = vec![0;file.metadata().unwrap().len() as usize];
    file.read(&mut rom)?;
    
    let mut temp32 = 0;
    for (pos, val) in rom[0..4].iter().cloned().enumerate() {
        temp32 |= (val as u64) << 24 - (pos * 8);
    }
    println!("{:x}", temp32);
    if temp32 != 0x4e45531a {
        panic!("bad header");
    }

    Ok( iNES {
        head: 0x4e45531a,
        PGRSIZE: rom[4],
        CHRSIZE: rom[5],


        mirror: (rom[6] & 0x01) != 0, //
        bat_PRGRAM: (rom[6] & 0x02) != 0,
        has_trainer: (rom[6] & 0x04) != 0,
        ignore_mirror: (rom[6] & 0x08) != 0,
        
        mapper: mapper::Mapper::new((rom[7] & 0xF0) | ((rom[6] & 0xF0)>>4)),

        unisystem: (rom[7] & 0x01) != 0,
        playchoice10: (rom[7] & 0x02) != 0,
        nes2: (rom[7] & 0x04) != 0,

        region: (rom[9] & 0x01) != 0,


        trainer: vec![0;if rom[6] & 0x4 != 0 {8192} else {0}],
        PGR: rom[(16+(if rom[6] & 0x4 != 0 {512} else {0}))..(16+(if rom[6] & 0x4 != 0 {512} else {0})+(rom[4] as usize*16384))].to_vec(),
        CHR: rom[(16+(if rom[6] & 0x4 != 0 {512} else {0})+(rom[4] as usize*16384))..(16+(if rom[6] & 0x4 != 0 {512} else {0})+(rom[4] as usize*16384)+(rom[5] as usize * 8192))].to_vec()
    })
}

fn InstructionToString(console: &mut Console, opcode: u8) -> String {
    let pc = console.CPU.PC;
    let aaa = (opcode & 0b11100000) >> 5;
    let bbb = (opcode & 0b00011100) >> 2;
    let cc = opcode & 0b00000011;
    let addr: u16 = match cc {
        0b00 => {
            match bbb {
                0b000 => { // immed
                    if aaa > 0b011 {
                        pc+1
                    } else if opcode == 0x20 {
                        memory::read16(console, pc+1)
                    } else {
                        0
                    }
                }
                0b001 => { // zero page
                    memory::read(console, pc+1) as u16
                }
                0b011 => { // absolute
                    if opcode == 0x6C {
                        let temp = memory::read16(console, pc+1);
                        memory::read16(console, temp)
                    } else {
                        memory::read16(console, pc+1)
                    }
                }
                0b100 => {
                    let num = memory::read(console, pc+1);
                    if (num & 0x80) != 0 {
                        (console.CPU.PC as i32 + (((!num) as i32)+1)*-1) as u16 + 2
                    } else {
                        console.CPU.PC + num as u16 + 2
                    }
                }
                0b101 => { // zp indexed x
                    ((memory::read(console, pc+1) + console.CPU.X) as u16) % 0xFF
                }
                0b111 => { // abs indexed x
                    memory::read16(console, pc+1) + (console.CPU.X as u16)
                }
                _ => {
                    0
                }
            }
        }
        0b01 => {
            match bbb {
                0b000 => { // (zp,X)
                    let temp = memory::read(console, pc+1);
                    memory::read16(console, ( temp as u16 + console.CPU.X as u16)% 0xFF) as u16
                }
                0b001 => { // zp
                    memory::read(console, pc+1) as u16
                }
                0b010 => { // immed
                    pc+1
                }
                0b011 => { // abs
                    memory::read16(console, pc+1)
                }
                0b100 => { // (zp), Y
                    let temp = memory::read(console, pc+1) as u16;
                    memory::read16(console, temp) + console.CPU.Y as u16
                }
                0b101 => { // zp,X
                    (memory::read(console, pc+1) as u16 + console.CPU.X as u16) % 0xFF
                }
                0b110 => { // abs, Y
                    let temp = memory::read16(console, pc+1);
                    temp + (console.CPU.Y as u16)
                }
                0b111 => { // abs, X
                    let temp = memory::read16(console, pc+1);
                    temp + (console.CPU.X as u16)
                }
                _ => {
                    panic!("impossible to reach");
                }
            }
        }
        0b10 => {
            match bbb {
                0b000 => {
                    if aaa > 0b011 {
                        pc+1
                    } else {
                        0
                    }
                }
                0b001 => { // zp
                    memory::read(console, pc+1) as u16
                }
                0b011 => { // abs
                    memory::read16(console, pc+1)
                }
                0b101 => {
                    if opcode == 0x96 || opcode == 0xB6 {
                        (memory::read(console, pc+1) as u16 + console.CPU.Y as u16) % 0xFF // zp, Y
                    } else {
                        (memory::read(console, pc+1) as u16 + console.CPU.X as u16) % 0xFF // zp, X
                    }
                }
                0b111 => {
                    if opcode == 0x9E || opcode == 0xBE {
                        memory::read16(console, pc+1) + (console.CPU.Y as u16)
                    } else {
                        memory::read16(console, pc+1) + (console.CPU.X as u16)
                    }
                }
                _ => {
                    0
                }
            }
        }
        _ => {
            panic!("unreachable")
        }
    };
    if opcode & 0x0F == 0x8 {
        return OPNAMES[opcode as usize].to_string()
    }
    if addr != 0x4016 && (addr < 0x2000 || addr >= 0x6000){
        if addr < 0x100 {
            format!("{} ${:02X} (#{:02X})",OPNAMES[opcode as usize], addr, memory::read(console, addr))
        } else {
            format!("{} ${:04X} (#{:02X})",OPNAMES[opcode as usize], addr, memory::read(console, addr))
        }
    } else {
        if addr < 0x100 {
            format!("{} ${:02X}",OPNAMES[opcode as usize], addr)
        } else {
            format!("{} ${:04X}",OPNAMES[opcode as usize], addr)
        }
    }
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {  
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Window", 640, 600)
        .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
        .build()
        .unwrap();
    let ntwindow = video_subsystem.window("Window", 512, 480)
    .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
    .build()
    .unwrap();

    let mut canvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();
    let mut ntcanvas = ntwindow.into_canvas()
    .index(find_sdl_gl_driver().unwrap())
    .build()
    .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();
    // ntcanvas.set_scale(10.0, 10.0).unwrap();
    let creator = canvas.texture_creator();
    // let ntcreator = ntcanvas.texture_creator();
    let mut texture = creator
       .create_texture_target(sdl2::pixels::PixelFormatEnum::RGB24, 256, 240).unwrap();

    // let mut nttexture = ntcreator
    //    .create_texture_target(sdl2::pixels::PixelFormatEnum::RGB24, 256, 240).unwrap();

    let dk = read_iNES("dk.nes".to_string()).expect("Read Error");
    let mut nes = Console::new(dk);

    println!("Start Vector 0x{:x}", memory::read16(&mut nes, 0xFFFC));
    nes.CPU.PC = memory::read16(&mut nes, 0xFFFC);
    
    let mut icount = 0;
    let mut calllevel = 0;
    //println!("{:?}", nes.Game.PGR[0..16].as_ref());
    let mut breakpoints: Vec<u16> = Vec::new();

    struct conditional_breakpoint {
        addr: u16,
        watch: u16,
        val: u8
    }

    let mut conditional_breakpoints: Vec<conditional_breakpoint> = Vec::<conditional_breakpoint>::new();
    'inputloop: loop {
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        input = input.to_lowercase();
        match input.chars().next().unwrap() {
            's' => break 'inputloop,
            'b' => {
                let bp = u16::from_str_radix(&input[2..6], 16)?;
                breakpoints.push(bp);
            },
            'c' => {
                let addr = u16::from_str_radix(&input[2..6], 16)?;
                let watch = u16::from_str_radix(&input[7..11], 16)?;
                let val = u8::from_str_radix(&input[12..14], 16)?;
                conditional_breakpoints.push(conditional_breakpoint{addr, watch, val})
            },
            'q' => return Ok(()),
            _ => {}
        }
    }
    let mut step = false;
    loop {
        let pc = nes.CPU.PC;
        let opcode = memory::read(&mut nes, pc);
        let opcode2 = memory::read(&mut nes, pc+1);
        let opcode3 = memory::read(&mut nes, pc+2);
        
        if nes.cycles%3 == 0 && !nes.PPU.oam_transfer {
            if nes.CPU.pause == 0 {
                if nes.Strobe {
                    nes.Controller1.shiftreg = nes.Controller1.Buttons;
                    println!("{:08b}", nes.Controller1.shiftreg);
                }
                //println!("{:04X}", memory::read16(&mut nes, 0xFFFA));
                //println!("{:?}", nes.CPU);
                let temp = InstructionToString(&mut nes, opcode);
                println!("{} {:}{:}{:04X}: {}", icount, nes.CPU, " ".repeat(calllevel+1), pc, temp);
                let mut do_break = false;
                for x in &conditional_breakpoints {
                    if x.addr == pc && memory::read(&mut nes, x.watch) == x.val {
                        do_break = true;
                    }
                }
                if breakpoints.contains(&nes.CPU.PC) || do_break || step {
                    step = false;
                    'breakpoint: loop {
                        let mut input = String::new();
                        stdin().read_line(&mut input)?;
                        match input.chars().next().unwrap() {
                            'p' => {
                                if input.len() == 7 {
                                    let addr = u16::from_str_radix(&input[2..6], 16)?;
                                    println!("${:04X}: {:02X}", addr, memory::read(&mut nes, addr));
                                } else if input.len() == 12 {
                                    let addr_start = u16::from_str_radix(&input[2..6], 16)?;
                                    let addr_end = u16::from_str_radix(&input[7..11], 16)?;
                                    print!("${:04X}:", addr_start);
                                    for i in addr_start..=addr_end {
                                        if (i-addr_start) % 0x10 == 0 && i != addr_start {
                                            print!("${:04X}:", i);
                                        }
                                        print!(" {:02X}", memory::read(&mut nes, i));
                                        if ((i-addr_start) % 0x10 == 15 && i != addr_start) || i == addr_end {println!()};
                                    }
                                }
                            },
                            's' => {
                                step = true;
                                break 'breakpoint;
                            },
                            'r' => break 'breakpoint,
                            'q' => return Ok(()),
                            _ => {}
                        }
                    }
                }
                //println!("{:02X} {:02X}", memory::read(&mut nes, 0x1FF), memory::read(&mut nes, 0x1FE));
                if OPNAMES[opcode as usize] == "JSR" || OPNAMES[opcode as usize] == "BRK" {
                    calllevel += 1;
                }
                if OPNAMES[opcode as usize] == "RTI" || OPNAMES[opcode as usize] == "RTS" {calllevel -= 1};
                icount += 1;
                //thread::sleep(time::Duration::from_millis(1));
            }
            opcodes::interpret_opcode(&mut nes, opcode);
            if nes.PPU.nmi_enable && nes.PPU.nmi_occured && nes.PPU.scanline == 241 && nes.PPU.cycle == 2 {
                //println!("Next instruction: {:04X}",nes.CPU.PC);
                calllevel += 1;
                let index = (nes.CPU.SP.wrapping_sub(1)) as u16 | 0x100;
                let pc = nes.CPU.PC;
                memory::write(&mut nes, index, ((pc & 0xFF00) >> 8 ) as u8);
                nes.CPU.SP = nes.CPU.SP.wrapping_sub(1);
                //println!("Pushed {:02X} at {:04X}", ((pc & 0xFF00) >> 8 ) as u8, index);
                let index = (nes.CPU.SP.wrapping_sub(1)) as u16 | 0x100;
                memory::write(&mut nes, index, (pc & 0x00FF) as u8);
                nes.CPU.SP = nes.CPU.SP.wrapping_sub(1);
                //println!("Pushed {:02X} at {:04X}", (pc & 0x00FF) as u8, index);



                let mut p: u8 = 0x00;
                p |= nes.CPU.carry as u8;
                p |= (nes.CPU.zero as u8) << 1;
                p |= (nes.CPU.interupt_disable as u8) << 2;
                p |= (nes.CPU.decimal as u8) << 3;
                p |= (nes.CPU.break_cmd as u8) << 4;
                p |= (nes.CPU.overflow as u8) << 6;
                p |= (nes.CPU.negative as u8) << 7;

                let index = (nes.CPU.SP-1) as u16 | 0x100;
                memory::write(&mut nes, index, p);
                nes.CPU.SP -= 1;
                //println!("Pushed {:02X} at {:04X}", p, index);

                nes.CPU.interupt_disable = true;
                
                nes.CPU.PC = memory::read16(&mut nes, 0xFFFA);
            } 
        }

        //if nes.cycles%4 == 0 {
            ppu::stepPPU(&mut nes, &mut canvas, &mut texture, &mut ntcanvas);
            if nes.PPU.cycle == 0 && nes.PPU.scanline == 0 {
            //println!("[{}]", nes.PPU.scanline);
            }
        //}
        for event in event_pump.poll_iter() {
            match event {
              Event::Quit { .. } => std::process::exit(0),
              Event::KeyDown { keycode, .. }  => {
                match keycode.unwrap() {
                    Keycode::Escape => std::process::exit(0),
                    Keycode::Down => {
                        nes.Controller1.Buttons |= 0b00100000;
                        println!("Controller = {:#08b}", nes.Controller1.Buttons);
                    },
                    Keycode::Return => {
                        nes.Controller1.Buttons |= 0b00001000;
                        println!("Controller = {:#08b}", nes.Controller1.Buttons);
                    },
                    _ => {}
                }
                
              },
              Event::KeyUp { keycode, .. } => {
                match keycode.unwrap() {
                    Keycode::Down => {
                        nes.Controller1.Buttons &= 0b11011111;
                        println!("Controller = {:#08b}", nes.Controller1.Buttons);
                    },
                    Keycode::Return => {
                        nes.Controller1.Buttons &= 0b11110111;
                        println!("Controller = {:#08b}", nes.Controller1.Buttons);
                    },
                    _ => {}
                }
              }
              _ => { /* do nothing */ }
            }
         }
        nes.cycles += 1
    }    
}

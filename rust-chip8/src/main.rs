extern crate piston_window;
extern crate image as im;

use piston_window::*;
use std::fs;
use rand::prelude::*;

const ON: im::Rgba<u8> = im::Rgba([255, 255, 255, 255]);
const OFF: im::Rgba<u8> = im::Rgba([0, 0, 0, 255]);

const WIDTH: u8 = 64;
const HEIGHT: u8 = 32;
const ZOOM: u32 = 8;
const FONT_START: usize = 0x050;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("chip8", (ZOOM*WIDTH as u32, ZOOM*HEIGHT as u32))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut texture_context = window.create_texture_context();
    let mut canvas = im::ImageBuffer::new(WIDTH.into(), HEIGHT.into());
    for p in canvas.pixels_mut() {
        *p = OFF;
    }
    let mut ts = TextureSettings::new();
    ts.set_filter(Filter::Nearest);
    let mut texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &canvas,
            &ts
        ).unwrap();

    let fn_ = match std::env::args().nth(1) {
        Some(val) => val,
        None => "src/roms/IBM".to_string(),
    };

    let contents = fs::read(fn_).unwrap();
    let mut chip8 = Chip8::new(contents);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            for _ in 0..12 {
                step(&mut chip8, &mut canvas);
            }
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                texture_context.encoder.flush(device);
                clear([0.0; 4], g);
                image(&texture, c.transform.zoom(ZOOM as f64), g);
            });
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            // This is a bit hacky. means only one key can be pressed at a time.
            if let Some(val) = key_to_input(key) {
                chip8.key = Some(val);
            }
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            // We only check to make sure it's valid key, not to use the value
            if let Some(_) = key_to_input(key) {
                chip8.key = None;
            }
        }
    }
}

fn key_to_input(key: Key) -> Option<u8> {
    match key {
        // row 1
        Key::D1 => Some(0x1),
        Key::D2 => Some(0x2),
        Key::D3 => Some(0x3),
        Key::D4 => Some(0xC),
        // row 2
        Key::Q => Some(0x4),
        Key::W => Some(0x5),
        Key::E => Some(0x6),
        Key::R => Some(0xD),
        // row 3
        Key::A => Some(0x7),
        Key::S => Some(0x8),
        Key::D => Some(0x9),
        Key::F => Some(0xE),
        // row 4
        Key::Z => Some(0xA),
        Key::X => Some(0x0),
        Key::C => Some(0xB),
        Key::V => Some(0xF),
        _ => None,
    }
}

fn step(chip8: &mut Chip8, canvas: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>) {
    if chip8.delay_timer > 0 {
        chip8.delay_timer -= 1;
    }
    if chip8.sound_timer > 0 {
        chip8.sound_timer -= 1;
    }

    let instruction = decode(chip8.ram[chip8.pc], chip8.ram[chip8.pc+1]);
    //println!("{:?} pc {} i {} reg {:?}", instruction, chip8.pc, chip8.i, chip8.var);
    match instruction {
        Instruction::ClearScreen => {
            for p in canvas.pixels_mut() {
                *p = OFF;
            }
        },
        Instruction::SetImm {vx,value} => chip8.var[vx as usize] = value,
        Instruction::AddImm {vx,value} => chip8.var[vx as usize] = chip8.var[vx as usize].wrapping_add(value),
        Instruction::Set {vx, vy} => chip8.var[vx as usize] = chip8.var[vy as usize],
        Instruction::Or {vx, vy} => chip8.var[vx as usize] = chip8.var[vx as usize] | chip8.var[vy as usize],
        Instruction::And {vx, vy} => chip8.var[vx as usize] = chip8.var[vx as usize] & chip8.var[vy as usize],
        Instruction::Xor {vx, vy} => chip8.var[vx as usize] = chip8.var[vx as usize] ^ chip8.var[vy as usize],
        Instruction::Add {vx, vy} => {
            let (res, overflow) = chip8.var[vx as usize].overflowing_add(chip8.var[vy as usize]);
            chip8.var[vx as usize] = res;
            chip8.var[0xF] = overflow as u8;
        }
        Instruction::SubLeft {vx, vy} => {
            let (res, overflow) = chip8.var[vx as usize].overflowing_sub(chip8.var[vy as usize]);
            chip8.var[vx as usize] = res;
            chip8.var[0xF] = !overflow as u8;
        }
        Instruction::SubRight {vx, vy} => {
            let (res, overflow) = chip8.var[vy as usize].overflowing_sub(chip8.var[vx as usize]);
            chip8.var[vx as usize] = res;
            chip8.var[0xF] = !overflow as u8;
        }
        Instruction::ShiftRight {vx, vy: _} => {
            let v = chip8.var[vx as usize];
            let (res, _overflow) = v.overflowing_shr(1);
            chip8.var[vx as usize] = res;
            chip8.var[0xF] = v&0x01 as u8;
        }
        Instruction::ShiftLeft {vx, vy: _} => {
            let v = chip8.var[vx as usize];
            let (res, _overflow) = v.overflowing_shl(1);
            chip8.var[vx as usize] = res;
            chip8.var[0xF] = (v&0x80 > 0) as u8;
        }
        Instruction::GetDelay {vx} => chip8.var[vx as usize] = chip8.delay_timer,
        Instruction::SetDelay {vx} => chip8.delay_timer = chip8.var[vx as usize],
        Instruction::SetSound {vx} => chip8.sound_timer = chip8.var[vx as usize],
        Instruction::Random {vx, value} => {
            let r: u8 = chip8.rng.gen();
            chip8.var[vx as usize] = r & value;
        }
        Instruction::BCDC {vx} => {
            let v = chip8.var[vx as usize];
            chip8.ram[chip8.i as usize] = v/100;
            chip8.ram[(chip8.i+1) as usize] = (v/10) % 10;
            chip8.ram[(chip8.i+2) as usize] = v % 10;
        }
        Instruction::SetIndexReg {addr} => chip8.i = addr,
        Instruction::AddIndexReg {vx} => chip8.i += chip8.var[vx as usize] as u16, // TODO handle overflow???
        Instruction::Font {vx} => {
            chip8.i = FONT_START as u16 + chip8.var[vx as usize] as u16 * 5
        },
        Instruction::StoreReg {vx} => {
            let vx = vx as u16;
            for r in 0..=vx {
                chip8.ram[(chip8.i+r) as usize] = chip8.var[r as usize];
            }
        }
        Instruction::LoadReg {vx} => {
            let vx = vx as u16;
            for r in 0..=vx {
                chip8.var[r as usize] = chip8.ram[(chip8.i+r) as usize];
            }
        }
        Instruction::Draw {vx, vy, n} => {
            chip8.var[0xF] = 0;

            let mut vf = 0;
            let mut y = chip8.var[vy as usize] % HEIGHT;
            for row in 0..n {
                let sprite: u8 = chip8.ram[(chip8.i + row as u16) as usize];
                if y >= HEIGHT {
                    break;
                }
                let mut x = chip8.var[vx as usize] % WIDTH;
                for sprite_idx in 0..8 {
                    if x >= WIDTH {
                        break;
                    }
                    //let haspix = (sprite >> sprite_idx) & 0x1 == 1;
                    let haspix = (sprite << sprite_idx) & 0x80 != 0;
                    let p = canvas.get_pixel_mut(x.into(), y.into());
                    if *p == ON && haspix {
                        *p = OFF;
                        vf = 1;
                    } else if *p == OFF && haspix {
                        *p = ON;
                    }
                    x += 1;
                }
                y += 1;
            }
            chip8.var[0xF] = vf;
        },
        // Modifies the PC
        Instruction::GetKey {vx} => {
            if let Some(key) = chip8.key {
                chip8.var[vx as usize] = key;
            } else {
                return // no change to PC
            }
        }
        Instruction::Call {addr} => {
            chip8.stack.push(chip8.pc);
            chip8.pc = addr as usize;
            return
        }
        Instruction::Return => {
            chip8.pc = chip8.stack.pop().unwrap();
            // HACK We don't return from here since we want PC to be incremented, otherwise
            // we'll infinite loop into the function call.
        }
        Instruction::Jump{addr} => {
            chip8.pc = addr as usize;
            return
        }
        Instruction::SkipEqImm {vx, value} => if chip8.var[vx as usize] == value { chip8.pc += 2; },
        Instruction::SkipNotEqImm {vx, value} => if chip8.var[vx as usize] != value { chip8.pc += 2; },
        Instruction::SkipEq {vx, vy} => if chip8.var[vx as usize] == chip8.var[vy as usize] { chip8.pc += 2; },
        Instruction::SkipNotEq {vx, vy} => if chip8.var[vx as usize] != chip8.var[vy as usize] { chip8.pc += 2; },
        Instruction::SkipKeyEq {vx} => {
            if let Some(key) = chip8.key {
                if chip8.var[vx as usize] == key {
                    chip8.pc += 2;
                }
            }
        },
        Instruction::SkipKeyNotEq {vx} => {
            if let Some(key) = chip8.key {
                if chip8.var[vx as usize] != key {
                    chip8.pc += 2;
                }
            } else {
                chip8.pc += 2;
            }
        },
        _ => panic!("ayy{:?}",instruction),
    }
    // Making sure this is our last statement.
    chip8.pc += 2;
}

#[derive(Debug)]
enum Instruction {
    ClearScreen,
    Return,
    Jump {addr: u16},
    Call {addr: u16},
    SetImm {vx: u8, value: u8},
    AddImm {vx: u8, value: u8},
    SkipEqImm {vx: u8, value: u8},
    SkipNotEqImm {vx: u8, value: u8},
    SkipEq {vx: u8, vy: u8},
    SkipNotEq {vx: u8, vy: u8},
    SkipKeyEq {vx: u8},
    SkipKeyNotEq {vx: u8},
    SetIndexReg {addr: u16},
    AddIndexReg {vx: u8},
    Font {vx: u8},
    Draw {vx: u8, vy: u8, n: u8},
    Set {vx: u8, vy: u8 },
    Or {vx: u8, vy: u8 },
    And {vx: u8, vy: u8 },
    Xor {vx: u8, vy: u8 },
    Add {vx: u8, vy: u8 },
    SubLeft {vx: u8, vy: u8 },
    SubRight {vx: u8, vy: u8 },
    ShiftLeft {vx: u8, vy: u8 },
    ShiftRight {vx: u8, vy: u8 },
    StoreReg {vx: u8},
    LoadReg {vx: u8},
    Random {vx: u8, value: u8},
    BCDC {vx: u8},
    GetKey {vx: u8},
    GetDelay {vx: u8},
    SetDelay {vx: u8},
    SetSound {vx: u8},
    NoOp,
}

fn decode(upper: u8, lower: u8) -> Instruction {
    let n = lower & 0xF;
    let nn = lower;
    let nnn = (upper as u16 & 0xF) << 8 | lower as u16;
    let vx = upper & 0xF;
    let vy = (lower & 0xF0) >> 4;
    let inst = (upper & 0xF0) >> 4;
    match inst {
        0x0 => {
            if upper == 0x00 && lower == 0xE0 {
                Instruction::ClearScreen
            } else if upper == 0x00 && lower == 0xEE {
                Instruction::Return
            } else {
                Instruction::NoOp
            }
        },
        0x1 => Instruction::Jump {addr: nnn},
        0x2 => Instruction::Call {addr: nnn},
        0x3 => Instruction::SkipEqImm {vx: vx, value: nn},
        0x4 => Instruction::SkipNotEqImm {vx: vx, value: nn},
        0x5 => Instruction::SkipEq {vx: vx, vy: vy}, // HACK check last byte?
        0x6 => Instruction::SetImm {vx: vx, value: nn},
        0x7 => Instruction::AddImm {vx: vx, value: nn},
        0x8 => {
            match n {
                0x0 => Instruction::Set {vx: vx, vy: vy },
                0x1 => Instruction::Or {vx: vx, vy: vy },
                0x2 => Instruction::And {vx: vx, vy: vy },
                0x3 => Instruction::Xor {vx: vx, vy: vy },
                0x4 => Instruction::Add {vx: vx, vy: vy },
                0x5 => Instruction::SubLeft {vx: vx, vy: vy },
                0x6 => Instruction::ShiftRight {vx: vx, vy: vy },
                0x7 => Instruction::SubRight {vx: vx, vy: vy },
                0xE => Instruction::ShiftLeft {vx: vx, vy: vy },
                _ => panic!("{:x}{:x}",upper, lower),
            }
        }
        0x9 => Instruction::SkipNotEq {vx: vx, vy: vy}, // HACK check last byte?
        0xA => Instruction::SetIndexReg {addr: nnn},
        0xB => panic!("{:x}{:x}",upper, lower),
        0xC => Instruction::Random {vx: vx, value: nn},
        0xD => Instruction::Draw {vx: vx, vy: vy, n: n},
        0xE => {
            match nn {
                0x9E => Instruction::SkipKeyEq {vx: vx},
                0xA1 => Instruction::SkipKeyNotEq {vx: vx},
                _ => panic!("{:x}{:x}",upper, lower),
            }
        },
        0xF => {
            match nn {
                0x07 => Instruction::GetDelay{vx:vx},
                0x15 => Instruction::SetDelay{vx:vx},
                0x18 => Instruction::SetSound{vx:vx},
                0x0A => Instruction::GetKey{vx: vx},
                0x29 => Instruction::Font{vx:vx},
                0x33 => Instruction::BCDC{vx:vx},
                0x55 => Instruction::StoreReg{vx:vx},
                0x65 => Instruction::LoadReg{vx:vx},
                0x1E => Instruction::AddIndexReg{vx:vx},
                _ => panic!("{:x}{:x}",upper, lower),
            }
        },
        _ => panic!("{:x}{:x}",upper, lower),
    }
}

struct Chip8 {
    ram: [u8; 4096],
    // registers
    pc: usize,
    i: u16,
    var: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<usize>,
    key: Option<u8>,
    rng: rand::rngs::ThreadRng,
}

impl Chip8 {
    fn new(program: Vec<u8>) -> Chip8 {
        let mut c = Chip8 {
            ram: [0; 4096],
            pc: 0x200,
            // are these right??
            i: 0,
            var: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            stack: Vec::new(),
            key: None,
            rng: rand::thread_rng(),
        };
        c.ram[c.pc..(c.pc+program.len())].copy_from_slice(&program);
        c.ram[FONT_START..FONT_START+0x50].copy_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ]);
        c
    }
}

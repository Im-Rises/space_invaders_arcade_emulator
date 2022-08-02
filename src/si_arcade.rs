use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::binary_lib::*;
use crate::si_arcade::inputs_outputs::InputsOutputs;

mod cpu;
mod inputs_outputs;
mod mmu;
mod ppu;

const WINDOW_WIDTH: usize = 600;
const WINDOW_HEIGHT: usize = 600;

const INTERRUPT_VBLANK_COUNTER: usize = cpu::CLOCK_FREQUENCY / ppu::SCREEN_FREQUENCY;
const INTERRUPT_MIDDLE_VBLANK: usize = INTERRUPT_VBLANK_COUNTER / 2;

const SCREEN_REFRESH_TIME: u128 = 16;

pub struct SpaceInvadersArcade {
    cpu: cpu::Cpu,
    ppu: ppu::Ppu,
    mmu: Rc<RefCell<mmu::Mmu>>,
    canvas: WindowCanvas,
    sdl_context: Sdl,
    extraship_btn_last_state: bool,
    inputs_outputs: InputsOutputs,
}

impl SpaceInvadersArcade {
    pub fn new() -> SpaceInvadersArcade {
        let mmu_init = Rc::new(RefCell::new(mmu::Mmu::new()));
        let video_init = SpaceInvadersArcade::init_video().unwrap();
        SpaceInvadersArcade {
            cpu: cpu::Cpu::new(&mmu_init, 0),
            ppu: ppu::Ppu::new(&mmu_init),
            mmu: Rc::clone(&mmu_init),
            canvas: video_init.0,
            sdl_context: video_init.1,
            extraship_btn_last_state: false,
            inputs_outputs: InputsOutputs::new(),
        }
    }
    pub fn start(&mut self) {
        let mut frequency_counter: usize = 0;
        let mut last_frequency_counter: usize = 0;
        let mut time = Instant::now();
        while self.get_window_active().unwrap() {
            if !self.cpu.get_halted() {
                if self.cpu.get_cycles() == 0 {
                    let opcode = self.cpu.fetch_opcode();
                    if opcode == 0xDB {
                        let port = self.cpu.fetch_byte();
                        let a = self.inputs(port, self.cpu.get_a());
                        self.cpu.set_a(a);
                        self.cpu.set_cycles(10);
                    } else if opcode == 0xd3 {
                        let port = self.cpu.fetch_byte();
                        self.outputs(port, self.cpu.get_a());
                        self.cpu.set_cycles(10);
                    } else {
                        let cycles = self.cpu.compute_opcode(opcode);
                        self.cpu.set_cycles(cycles);
                    }
                }
                self.cpu.set_cycles(self.cpu.get_cycles() - 1);
            }
            frequency_counter += 1;
            if self.cpu.get_inte() {
                if frequency_counter > INTERRUPT_MIDDLE_VBLANK && last_frequency_counter <= INTERRUPT_MIDDLE_VBLANK {
                    cpu::interrupts::interrupt(&mut self.cpu, 1);
                }
                if frequency_counter > INTERRUPT_VBLANK_COUNTER {
                    cpu::interrupts::interrupt(&mut self.cpu, 2);
                    frequency_counter = 0;
                    self.ppu.clock();
                    self.update_screen().expect("Error: Cannot update screen");
                    while time.elapsed().as_millis() < SCREEN_REFRESH_TIME {
                        // println!("{}", time.elapsed().as_millis())
                    }
                    time = Instant::now();
                }
            } else {
                frequency_counter = 0;
            }

            last_frequency_counter = frequency_counter;
        }
    }

    fn inputs(&mut self, port: u8, mut data: u8) -> u8 {
        match port {
            0 => {
                data = 0b0000_1110;
            } //INPUTS (Mapped in hardware but never used by the code)
            1 => {
                data = 0b0000_1000;
                data = set_reset_bit(data, 0, self.inputs_outputs.coin);
                data = set_reset_bit(data, 1, self.inputs_outputs.player2.start);
                data = set_reset_bit(data, 2, self.inputs_outputs.player1.start);
                data = set_reset_bit(data, 4, self.inputs_outputs.player1.shot);
                data = set_reset_bit(data, 5, self.inputs_outputs.player1.left);
                data = set_reset_bit(data, 6, self.inputs_outputs.player1.right);
            } //INPUTS
            2 => {
                data = 0b0000_0000;
                data = set_reset_bit(data, 0, self.inputs_outputs.dip3);
                data = set_reset_bit(data, 1, self.inputs_outputs.dip5);
                data = set_reset_bit(data, 3, self.inputs_outputs.dip6);
                data = set_reset_bit(data, 4, self.inputs_outputs.player2.shot);
                data = set_reset_bit(data, 5, self.inputs_outputs.player2.left);
                data = set_reset_bit(data, 6, self.inputs_outputs.player2.right);
                data = set_reset_bit(data, 7, self.inputs_outputs.dip7);
            } //INPUTS
            3 => data = ((self.inputs_outputs.shift_register >> (8 - self.inputs_outputs.shift_offset)) & 0xFF) as u8,
            6 => (), //WATCHDOG
            _ => {
                println!(
                    "Error: Writing to port not implemented at port {} with data {}",
                    port, data
                );
            }
        }

        data
    }

    fn outputs(&mut self, port: u8, data: u8) {
        match port {
            2 => self.inputs_outputs.shift_offset = data & 0b0000_0111,
            3 => (), //Sound bit
            4 => self.inputs_outputs.shift_register = self.inputs_outputs.shift_register >> 8 | (data as u16) << 8,
            5 => (), //Sound bit
            6 => (), //Watch dog
            _ => {
                println!(
                    "Error: Reading from port not implemented at port {} with data {}",
                    port, data
                );
            }
        }
    }

    // fn pause_emulation(&self) {}
    // fn restart_emulation(&self) {}
    // fn save_state(&self) {}
    // fn load_state(&self) {}

    /*-------------------SDL2 Video and Inputs---------------------*/

    fn init_video() -> Result<(WindowCanvas, Sdl), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(
                "Space Invaders Arcade Emulator",
                WINDOW_WIDTH as u32,
                WINDOW_HEIGHT as u32,
            )
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas
            .set_logical_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
            .expect("Error: Cannot create canvas");
        Ok((canvas, sdl_context))
    }

    fn update_screen(&mut self) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGB24,
                ppu::SCREEN_WIDTH as u32,
                ppu::SCREEN_HEIGHT as u32,
            )
            .map_err(|e| e.to_string())?;
        texture
            .update(None, self.ppu.get_screen(), ppu::SCREEN_WIDTH * 3)
            .expect("Error: Cannot create texture");

        self.canvas.copy_ex(&texture, None, None, -90.0, None, false, false)?;
        self.canvas.present();
        Ok(())
    }

    fn get_window_active(&mut self) -> Result<bool, String> {
        let mut event_pump = self.sdl_context.event_pump()?;
        let mut window_active = true;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => window_active = false,

                //Insert Coin KeyDown
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => self.inputs_outputs.coin = true,
                //Insert Coin KeyUp
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => self.inputs_outputs.coin = false,

                //Player 1 KeyDown
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => self.inputs_outputs.player1.left = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => self.inputs_outputs.player1.right = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => self.inputs_outputs.player1.shot = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => self.inputs_outputs.player1.start = true,
                //Player 1 KeyUp
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => self.inputs_outputs.player1.left = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => self.inputs_outputs.player1.right = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => self.inputs_outputs.player1.shot = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => self.inputs_outputs.player1.start = false,

                //Player 2 KeyDown
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => self.inputs_outputs.player2.left = true,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => self.inputs_outputs.player2.right = true,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => self.inputs_outputs.player2.shot = true,
                Event::KeyDown {
                    keycode: Some(Keycode::G),
                    ..
                } => self.inputs_outputs.player2.start = true,
                //Player 2 KeyUp
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => self.inputs_outputs.player2.left = false,
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => self.inputs_outputs.player2.right = false,
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => self.inputs_outputs.player2.shot = false,
                Event::KeyUp {
                    keycode: Some(Keycode::G),
                    ..
                } => self.inputs_outputs.player2.start = false,

                //DIP 3
                Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => self.inputs_outputs.dip3 = true,
                Event::KeyUp {
                    keycode: Some(Keycode::K),
                    ..
                } => self.inputs_outputs.dip3 = false,

                //DIP 5
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => self.inputs_outputs.dip5 = true,
                Event::KeyUp {
                    keycode: Some(Keycode::L),
                    ..
                } => self.inputs_outputs.dip5 = false,

                //DIP 6
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    if !self.extraship_btn_last_state {
                        self.inputs_outputs.dip6 = !self.inputs_outputs.dip6;
                        self.extraship_btn_last_state = true;
                        if self.inputs_outputs.dip6 {
                            println!("Extra ship at 1000 points");
                        } else {
                            println!("Extra ship at 1500 points");
                        }
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    self.extraship_btn_last_state = false;
                }

                // //DIP7 Coin info displayed in demo screen 0=ON
                // Event::KeyDown {
                //     keycode: Some(Keycode::O),
                //     ..
                // } => self.inputs_outputs.borrow_mut().dip7 = true,
                // Event::KeyUp {
                //     keycode: Some(Keycode::O),
                //     ..
                // } => self.inputs_outputs.borrow_mut().dip7 = false,

                // Default
                _ => window_active = true,
            }
        }
        Ok(window_active)
    }
}

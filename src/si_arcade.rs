use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{CanvasBuilder, Texture, WindowCanvas};
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

use crate::binary_lib::get_bit;

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
    inputs_outputs: Rc<RefCell<inputs_outputs::InputsOutputs>>,
    canvas: WindowCanvas,
    sdl_context: Sdl,
    extraship_btn_last_state: bool,
}

impl SpaceInvadersArcade {
    pub fn new() -> SpaceInvadersArcade {
        let mmu_init = Rc::new(RefCell::new(mmu::Mmu::new()));
        let video_init = SpaceInvadersArcade::init_video().unwrap();
        let inputs_outputs_init = Rc::new(RefCell::new(inputs_outputs::InputsOutputs::new()));
        SpaceInvadersArcade {
            cpu: cpu::Cpu::new(&mmu_init, &inputs_outputs_init, 0),
            ppu: ppu::Ppu::new(&mmu_init),
            mmu: Rc::clone(&mmu_init),
            inputs_outputs: Rc::clone(&inputs_outputs_init),
            canvas: video_init.0,
            sdl_context: video_init.1,
            extraship_btn_last_state: false,
        }
    }
    pub fn start(&mut self) {
        let mut frequency_counter: usize = 0;
        let mut last_frequency_counter: usize = 0;
        let mut time = Instant::now();
        while self.get_window_active().unwrap() {
            self.cpu.clock();
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
            // .hidden()
            // .opengl()
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
                } => self.inputs_outputs.borrow_mut().coin = true,
                //Insert Coin KeyUp
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => self.inputs_outputs.borrow_mut().coin = false,

                //Player 1 KeyDown
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => self.inputs_outputs.borrow_mut().player1.left = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => self.inputs_outputs.borrow_mut().player1.right = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => self.inputs_outputs.borrow_mut().player1.shot = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => self.inputs_outputs.borrow_mut().player1.start = true,

                //Player 1 KeyUp
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => self.inputs_outputs.borrow_mut().player1.left = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => self.inputs_outputs.borrow_mut().player1.right = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => self.inputs_outputs.borrow_mut().player1.shot = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => self.inputs_outputs.borrow_mut().player1.start = false,

                //Player 2 KeyDown
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => self.inputs_outputs.borrow_mut().player2.left = true,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => self.inputs_outputs.borrow_mut().player2.right = true,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => self.inputs_outputs.borrow_mut().player2.shot = true,
                Event::KeyDown {
                    keycode: Some(Keycode::G),
                    ..
                } => self.inputs_outputs.borrow_mut().player2.start = true,
                //Player 2 KeyUp
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => self.inputs_outputs.borrow_mut().player2.left = false,
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => self.inputs_outputs.borrow_mut().player2.right = false,
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => self.inputs_outputs.borrow_mut().player2.shot = false,
                Event::KeyUp {
                    keycode: Some(Keycode::G),
                    ..
                } => self.inputs_outputs.borrow_mut().player2.start = false,

                //DIP 3
                Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => self.inputs_outputs.borrow_mut().dip3 = true,
                Event::KeyUp {
                    keycode: Some(Keycode::K),
                    ..
                } => self.inputs_outputs.borrow_mut().dip3 = false,

                //DIP 5
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => self.inputs_outputs.borrow_mut().dip5 = true,
                Event::KeyUp {
                    keycode: Some(Keycode::L),
                    ..
                } => self.inputs_outputs.borrow_mut().dip5 = false,

                //DIP 6
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    if !self.extraship_btn_last_state {
                        self.inputs_outputs.borrow_mut().dip6 = !self.inputs_outputs.borrow_mut().dip6;
                        self.extraship_btn_last_state = true;
                        if self.inputs_outputs.borrow().dip6 {
                            println!("Extra ship at 1000 points");
                        } else {
                            println!("Extra ship at 1000 points");
                        }
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    self.inputs_outputs.borrow_mut().dip6 = false;
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

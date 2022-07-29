use std::cell::RefCell;
use std::rc::Rc;

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

pub struct SpaceInvadersArcade {
    cpu: cpu::Cpu,
    ppu: ppu::Ppu,
    mmu: Rc<RefCell<mmu::Mmu>>,
    inputs_outputs: Rc<RefCell<inputs_outputs::InputsOutputs>>,
    canvas: WindowCanvas,
    sdl_context: Sdl,
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
        }
    }
    pub fn start(&mut self) {
        let mut frequency_counter: usize = 0;
        let mut last_frequency_counter: usize = 0;
        while self.get_window_active().unwrap() {
            self.cpu.clock();
            frequency_counter += 1;
            if self.cpu.get_inte() {
                if frequency_counter >= INTERRUPT_MIDDLE_VBLANK && last_frequency_counter < INTERRUPT_MIDDLE_VBLANK {
                    cpu::interrupts::interrupt(&mut self.cpu, 1);
                }
                if frequency_counter >= INTERRUPT_VBLANK_COUNTER {
                    cpu::interrupts::interrupt(&mut self.cpu, 2);
                    frequency_counter = 0;
                    self.ppu.clock();
                    self.update_screen().expect("TODO: panic message");
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

    fn clock_ppu(&self) {}

    fn handle_inputs(&self) {}

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
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas
            .set_logical_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
            .expect("TODO: panic message");
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
            .expect("TODO: panic message");

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
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => self.inputs_outputs.borrow_mut().set_c(),
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => self.inputs_outputs.borrow_mut().reset_c(),
                _ => window_active = true,
            }
        }
        Ok(window_active)
    }
}

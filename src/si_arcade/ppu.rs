extern crate sdl2;

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
use crate::si_arcade::mmu::Mmu;

pub const SCREEN_FREQUENCY: usize = 60;
const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 224;

const WINDOW_WIDTH: usize = 600;
const WINDOW_HEIGHT: usize = 600;

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
    screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
    canvas: WindowCanvas,
    sdl_context: Sdl,
}

//https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/renderer-texture.rs

impl Ppu {
    pub fn new(mmu: &Rc<RefCell<Mmu>>) -> Ppu {
        let video = Ppu::init_video().unwrap();

        Ppu {
            mmu: Rc::clone(&mmu),
            screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
            canvas: video.0,
            sdl_context: video.1,
        }
    }

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

    pub fn start_video(&mut self) {
        // self.window.show();
    }

    pub fn clock(&mut self) -> Result<(), String> {
        let mut index: usize = 0;
        for data in self.mmu.borrow().get_vram() {
            for bit in 0..8 {
                let color: u8;
                if get_bit(*data, bit) {
                    color = 0xFF;
                } else {
                    color = 0;
                }
                self.screen[index] = color;
                self.screen[index + 1] = color;
                self.screen[index + 2] = color;
                index += 3;
            }
        }

        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, (SCREEN_WIDTH) as u32, SCREEN_HEIGHT as u32)
            .map_err(|e| e.to_string())?;
        texture
            .update(None, &self.screen, SCREEN_WIDTH * 3)
            .expect("TODO: panic message");

        self.canvas.copy_ex(&texture, None, None, -90.0, None, false, false)?;
        self.canvas.present();
        Ok(())
    }

    pub fn get_window_active(&self) -> Result<bool, String> {
        let mut event_pump = self.sdl_context.event_pump()?;
        let mut window_active = true;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => window_active = false,
                _ => window_active = true,
            }
        }
        Ok(window_active)
    }
}

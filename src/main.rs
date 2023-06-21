//! This crate and executable are solely for running and testing the library.
//!
//!
mod window;
use anyhow::Result;
use cgmath::{vec2, Vector2};
use internal::{Internal, InternalData};
use log::info;
use window::*;
use winit::dpi::PhysicalSize;

pub struct KittenGame {
    window: KittenWindow,
    internal_renderer: Internal,
    //internal_graphics_data: InternalData,
    window_size: Vector2<usize>,
}

impl KittenGame {
    pub fn run(title: &str, window_size: Vector2<usize>) -> Result<()> {
        // initialize important stuff.
        let (mut window, window_loop) = window::KittenWindow::new(
            title,
            PhysicalSize::new(window_size.x.try_into()?, window_size.y.try_into()?),
        )?;

        let internal_renderer = Internal::new(&window)?;
        //let interal_graphics_data = InternalData::default();
        // mouse handling / gamepad handling / keyboard handling (input)
        // audio??
        // graphics :3

        let kitten_game = KittenGame {
            window,
            internal_renderer,
            window_size,
        };

        window_loop.run(
            kitten_game,
            |kitten_game: &mut KittenGame| kitten_game.init(),
            move |kitten_game: &mut KittenGame| kitten_game.update(),
            move |kitten_game: &mut KittenGame| kitten_game.render(),
            |kitten_game: &mut KittenGame| kitten_game.quit(),
        );
    }
    fn init(&mut self) -> Result<()> {
        todo!()
    }
    fn quit(&mut self) -> Result<()> {
        info!("quitting!");
        Ok(())
    }
    fn update(&mut self) -> Result<()> {
        Ok(())
    }
    fn render(&mut self) -> Result<()> {
        // 1. select / load in shader.
        // 2. create / set pipeline
        // 3. draw
        self.internal_renderer.render(self.window_size)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    KittenGame::run("Title", vec2(1270, 720))?;
    Ok(())
}

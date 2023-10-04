//! This crate and executable are solely for running and testing the library.
//!
//!
use anyhow::Result;
use glam::UVec2;
use internal::{Internal, InternalData};
use log::info;
use window::*;

pub struct KittenGame {
    window: KittenWindow,
    internal_renderer: Internal,
    internal_graphics_data: InternalData,
    _input_system: (),
    window_size: UVec2,
}

impl Target for KittenGame {}

impl KittenGame {
    pub fn run(title: &str, window_size: UVec2) -> Result<()> {
        // initialize important stuff.
        let (window, window_loop) = window::KittenWindow::new(title, window_size)?;

        let internal_renderer = Internal::new(&window)?;
        let internal_graphics_data = InternalData::default();
        // mouse handling / gamepad handling / keyboard handling (input)
        // audio??
        // graphics :3

        let kitten_game = KittenGame {
            window,
            internal_renderer,
            internal_graphics_data,
            _input_system: (),
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
        self.window.set_resizable(true);
        Ok(())
    }
    fn quit(&mut self) -> Result<()> {
        info!("quitting!");
        Ok(())
    }
    fn update(&mut self) -> Result<()> {
        if self.window_size != self.window.size()? {
            self.window_size = self.window.size()?;
        }

        Ok(())
    }
    fn render(&mut self) -> Result<()> {
        // 1. select / load in shader.
        // 2. create / set pipeline
        // 3. draw
        self.internal_renderer
            .render(&self.internal_graphics_data, self.window_size)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    KittenGame::run("Title", UVec2::from((1280, 720)))?;
    Ok(())
}

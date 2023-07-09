mod state;

use cgmath::vec2;
use internal::WindowTrait;
pub use state::SharedState;
use std::rc::Rc;

use anyhow::Result;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use winit::{
    dpi::{PhysicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::KittenGame;

pub struct KittenWindow {
    state: SharedState,
}

pub struct WindowLoop {
    state: SharedState,
    event_loop: EventLoop<()>,
}

impl WindowLoop {
    pub fn run<I, U, R, Q>(
        self,
        kitten_game: KittenGame,
        mut init_fn: I,
        mut update_fn: U,
        mut render_fn: R,
        mut quit_fn: Q,
    ) -> !
    where
        I: 'static + FnMut(&mut KittenGame) -> Result<()>,
        U: 'static + FnMut(&mut KittenGame) -> Result<()>,
        R: 'static + FnMut(&mut KittenGame) -> Result<()>,
        Q: 'static + FnMut(&mut KittenGame) -> Result<()>,
    {
        let mut kitten_game = kitten_game;
        init_fn(&mut kitten_game).unwrap();
        self.event_loop
            .run(move |event, _window_target, control_flow| {
                control_flow.set_poll();
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        quit_fn(&mut kitten_game).unwrap();
                        control_flow.set_exit();
                    }
                    Event::MainEventsCleared => {
                        self.state.window.request_redraw();
                    }
                    Event::RedrawRequested(window_id) if window_id == self.state.window.id() => {
                        update_fn(&mut kitten_game).unwrap();
                        match render_fn(&mut kitten_game) {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("{e}")
                            }
                        }
                    }
                    event => self
                        .state
                        .events
                        .borrow_mut()
                        .push(event.to_static().unwrap()),
                }
            })
    }
}

impl KittenWindow {
    pub fn new(title: &str, size: PhysicalSize<u32>) -> Result<(Self, WindowLoop)> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .build(&event_loop)?;
        let state = SharedState {
            window: Rc::new(window),
            events: Default::default(),
        };

        Ok((
            Self {
                state: state.clone(),
            },
            WindowLoop { state, event_loop },
        ))
    }
}

impl WindowTrait for KittenWindow {
    fn size(&self) -> std::result::Result<cgmath::mint::Vector2<usize>, anyhow::Error> {
        let inner = self.state.window.inner_size();
        Ok(vec2(inner.width as usize, inner.height as usize).into())
    }

    fn set_size(&mut self, size: (u32, u32)) {
        self.state
            .window
            .set_inner_size(PhysicalSize::new(size.0, size.1))
    }

    fn resizable(&self) -> bool {
        self.state.window.is_resizable()
    }

    fn set_resizable(&mut self, resizable: bool) {
        self.state.window.set_resizable(resizable)
    }
}

unsafe impl HasRawDisplayHandle for KittenWindow {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.state.window.raw_display_handle()
    }
}

unsafe impl HasRawWindowHandle for KittenWindow {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.state.window.raw_window_handle()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, 4);
    }
}

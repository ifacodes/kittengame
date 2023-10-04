use glam::UVec2;
pub use state::SharedState;
use std::rc::Rc;

use anyhow::Result;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use winit::{
    dpi::{PhysicalSize, Pixel, Size},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub struct KittenWindow {
    state: SharedState,
}

pub struct WindowLoop {
    state: SharedState,
    event_loop: EventLoop<()>,
}

pub trait WindowTrait: HasRawDisplayHandle + HasRawWindowHandle {
    fn size(&self) -> Result<UVec2>;
    fn set_size(&mut self, size: (u32, u32));
    fn resizable(&self) -> bool;
    fn set_resizable(&mut self, resizable: bool);
}

pub trait Target {}

impl WindowLoop {
    pub fn run<I, U, R, Q, T: Target + 'static>(
        self,
        kitten_game: T,
        mut init_fn: I,
        mut update_fn: U,
        mut render_fn: R,
        mut quit_fn: Q,
    ) -> !
    where
        I: 'static + FnMut(&mut T) -> Result<()>,
        U: 'static + FnMut(&mut T) -> Result<()>,
        R: 'static + FnMut(&mut T) -> Result<()>,
        Q: 'static + FnMut(&mut T) -> Result<()>,
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
    pub fn new(title: &str, size: UVec2) -> Result<(Self, WindowLoop)> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::<u32>::from(size.to_array()))
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
    fn size(&self) -> std::result::Result<UVec2, anyhow::Error> {
        let inner = self.state.window.inner_size();
        Ok(UVec2::from((inner.width, inner.height)))
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

mod state {
    use std::{cell::RefCell, rc::Rc};

    use winit::{event::Event, window::Window};

    #[derive(Debug, Clone)]
    pub struct SharedState {
        pub window: Rc<Window>,
        pub events: Rc<RefCell<Vec<Event<'static, ()>>>>,
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

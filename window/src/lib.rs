mod state;
mod traits;
mod window;

use state::SharedState;
use std::rc::Rc;
use traits::{HandlesWindowLoop, KittenWindow};

use anyhow::Result;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

struct Window {
    state: SharedState,
}

struct WindowLoop {
    state: SharedState,
    event_loop: EventLoop<()>,
}

impl HandlesWindowLoop for WindowLoop {
    fn run<I, U, R, Q>(self, init_fn: I, update_fn: U, render_fn: R, mut quit_fn: Q) -> !
    where
        I: 'static + FnMut() -> Result<()>,
        U: 'static + FnMut() -> Result<()>,
        R: 'static + FnMut() -> Result<()>,
        Q: 'static + FnMut() -> Result<()>,
    {
        self.event_loop
            .run(move |event, window_target, control_flow| {
                control_flow.set_poll();
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        quit_fn().unwrap();
                        control_flow.set_exit();
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

impl KittenWindow for Window {
    type HandlesWindowLoop = WindowLoop;

    fn new(title: &str) -> Result<Self> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title(title).build(&event_loop)?;
        let state = SharedState {
            window: Rc::new(window),
            events: Default::default(),
        };

        Ok(Self { state })
    }
}

unsafe impl HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.state.window.raw_display_handle()
    }
}

unsafe impl HasRawWindowHandle for Window {
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

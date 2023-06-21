use std::{cell::RefCell, rc::Rc};

use winit::event::Event;

#[derive(Debug, Clone)]
pub struct SharedState {
    pub window: Rc<winit::window::Window>,
    pub events: Rc<RefCell<Vec<Event<'static, ()>>>>,
}

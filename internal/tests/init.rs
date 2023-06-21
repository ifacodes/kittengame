use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new().with_title("init test");
    let window = builder
        .build(&event_loop)
        .expect("unable to create window!");

    internal::Internal::new(&window).expect("unable to initialize Internal");
}

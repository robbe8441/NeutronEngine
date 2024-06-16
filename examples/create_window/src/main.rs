use neutron::window::{Window, events::*};


fn main() {
    let window = Window::new().unwrap();

    window.set_size(100, 100);
    
    while !window.should_close() {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.01));
        let size = window.get_size();
        window.set_size(size.0 + 1, size.1);
        window.set_position(size.0 + 1, size.1);
        window.poll_events();
        dbg!(window.get_position());
    }
}

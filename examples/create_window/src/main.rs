use neutron::window::{Window, events::*};


fn main() {
    let window = Window::new().unwrap();

    window.set_size(1000, 500);
    let mut name = "".to_string();

    while !window.should_close() {
        window.poll_events();
        name.push('A');
        window.set_title(name.as_str());

        let len = name.len();

        let clone = window.clone();
        clone.set_size(len as i32, 500);
    }
}

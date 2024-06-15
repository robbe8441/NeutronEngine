use neutron::window::{Window, events::*};


fn main() {
    let mut window = Window::new().unwrap();
    window.set_swap_interval(0);

    window.run(|event, window| {
        match event {

             WindowEvent::Key(Key::F11, _, Action::Press, _) => {
                 window.set_full_screen(!window.is_full_screen());
             }

            _ => {}
        }

    });
}

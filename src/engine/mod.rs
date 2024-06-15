mod rendering;
mod window;

pub fn setup() {
    let (event_loop, window) = window::new().unwrap();

    let mut renderer = rendering::new(window.clone()).unwrap();

    let mut scene = rendering::update::Scene::new(&renderer);

    event_loop
        .run(|event, target| match event {
            winit::event::Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                winit::event::WindowEvent::RedrawRequested => {
                    scene.draw(&mut renderer).unwrap();
                }
                winit::event::WindowEvent::CloseRequested => target.exit(),
                _ => {}
            },
            winit::event::Event::AboutToWait => window.request_redraw(),
            _ => {}
        })
        .unwrap()
}

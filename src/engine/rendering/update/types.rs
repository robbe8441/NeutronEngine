// use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};
//
// #[repr(C)]
// #[derive(BufferContents, Vertex, Clone)]
// pub struct Vertex2D {
//     #[format(R32G32_SFLOAT)]
//     position: [f32; 2],
// }
//
// pub const SCREEN_VERTECIES: [Vertex2D; 6] = [
//     Vertex2D {
//         position: [-1.0, -1.0],
//     },
//     Vertex2D {
//         position: [1.0, -1.0],
//     },
//     Vertex2D {
//         position: [1.0, 1.0],
//     },
//     Vertex2D {
//         position: [1.0, 1.0],
//     },
//     Vertex2D {
//         position: [-1.0, 1.0],
//     },
//     Vertex2D {
//         position: [-1.0, -1.0],
//     },
// ];

pub mod main_shader {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "./shaders/main.glsl",
    }
}

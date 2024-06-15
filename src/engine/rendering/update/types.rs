
pub const ROOT : u64 = 129526;

pub mod main_shader {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "./shaders/main.glsl",
    }
}

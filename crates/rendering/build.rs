use cmake::Config;



fn main() {

    let dst = Config::new("vulkan_bindings").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=vulkan");
}

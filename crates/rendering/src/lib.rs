
#[link(name = "vulkan", kind="static")]
extern "C" {
    fn bad_add(a:i32, b:i32) -> i32;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = unsafe { bad_add(2, 2) };
        assert_eq!(result, 4);
    }
}

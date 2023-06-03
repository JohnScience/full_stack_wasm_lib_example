
fn main() {
    println!("Hello, world!");
}

#[cfg(all(test, feature = "frontend"))]
pub mod frontend_tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test() {
        assert_eq!(1, 1);
    }
}

#[cfg(all(test, feature = "backend"))]
pub mod backend_tests {
    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
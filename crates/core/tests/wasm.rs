#[cfg(target_family = "wasm32-unknown-unknown")]
#[cfg(test)]
mod tests {
    use fluidity_core::timestamp;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn compiles() {
        let f = |i: usize, j: usize| i + j;
        let a = f(2, 2);
        assert_eq!(a, 4);
    }

    #[wasm_bindgen_test]
    fn test_timestamp() {
        let a = timestamp();
        assert!(true)
    }
}

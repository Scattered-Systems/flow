#[cfg(target_family = "wasm32-unknown-unknown")]
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn lib_compiles() {
        let f = |i: usize, j: usize| i + j;
        let a = f(2, 2);
        assert_eq!(a, 4);
    }

    #[wasm_bindgen_test]
    fn test_timestamp() {
        let a = fluidity::timestamp();
        assert!(true)
    }
}

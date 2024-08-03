//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use argon2_wasm;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_dedicated_worker);

#[wasm_bindgen_test]
fn test_hash() {
    assert!(argon2_wasm::hash_password(&"horse battery stapler").is_ok())
}

#[wasm_bindgen_test]
fn test_verify() {
    let hash = argon2_wasm::hash_password(&"horse battery stapler").unwrap();
    assert!(argon2_wasm::verify_password(
        &"horse battery stapler",
        &hash
    ))
}

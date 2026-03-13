use wasm_bindgen::prelude::*;

// 在浏览器控制台打印
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    log("Hello World from Rust!");
}

// 加法函数
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

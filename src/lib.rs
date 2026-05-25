use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
pub fn get_points() -> JsValue {
    let points = vec![
        Point { x: 50.0, y: 50.0 },
        Point { x: 120.0, y: 80.0 },
        Point { x: 200.0, y: 150.0 },
        Point { x: 300.0, y: 120.0 },
        Point { x: 310.0, y: 130.0 },
     ];

    serde_wasm_bindgen::to_value(&points).unwrap()
}
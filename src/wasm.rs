use std::sync::Once;
use log::warn;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wee_alloc;

use crate::dispatch;
use crate::dag::{chunk, key};
use crate::prolly::chunker::Chunker;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn exercise_dag() {
    init_panic_hook();
    let c = chunk::Chunk::new("h1".to_string(), vec![0, 1], &vec!["r1"]);
    let k: key::Key = "c/h1/d".parse().unwrap();
    warn!("{:?} {:?}", c, k);
}

#[wasm_bindgen]
pub fn buzhash() {
    init_panic_hook();
    let mut c = Chunker::default();
    c.hash_byte(b'f');
}

#[wasm_bindgen]
pub async fn dispatch(db_name: String, rpc: String, args: String) -> Result<String, JsValue> {
    init_panic_hook();
    match dispatch::dispatch(db_name, rpc, args).await {
        Err(v) => Err(JsValue::from_str(&v[..])),
        Ok(v) => Ok(v),
    }
}

static INIT: Once = Once::new();

fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    INIT.call_once(|| {
        if let Err(e) = console_log::init_with_level(log::Level::Info) {
            web_sys::console::error_1(&format!("Error registering console_log: {}", e).into());
        }
    });
}

use async_std::sync::{Mutex, RwLock};
use std::collections::HashMap;
use std::sync::Once;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::dag;
use crate::embed;
use crate::kv;
use crate::kv::idbstore::IdbStore;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(not(default))]
pub async fn new_idbstore(name: String) -> Option<Box<dyn kv::Store>> {
    init_panic_hook();
    match IdbStore::new(&name).await {
        Ok(Some(v)) => Some(Box::new(v)),
        _ => None,
    }
}

#[wasm_bindgen]
pub async fn dispatch(db_name: String, rpc: String, args: String) -> Result<String, JsValue> {
    init_panic_hook();
    match embed::dispatch(db_name, rpc, args).await {
        Err(v) => Err(JsValue::from_str(&v[..])),
        Ok(v) => Ok(v),
    }
}

#[wasm_bindgen]
pub struct ScanItem {
    key: js_sys::Uint8Array,
    val: js_sys::Uint8Array,
}

static INIT: Once = Once::new();

pub fn init_console_log() {
    INIT.call_once(|| {
        if let Err(e) = console_log::init_with_level(log::Level::Info) {
            web_sys::console::error_1(&format!("Error registering console_log: {}", e).into());
        }
    });
}

fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    init_console_log();
}

struct ConnMap {
    map: HashMap<String, RwLock<dag::Store>>,
}

// This is safe because we don't actually use ConnMap across threads
// (there are no threads in wasm). But lazy_static doesn't know that.
unsafe impl Send for ConnMap {}

lazy_static! {
    static ref connections: Mutex<ConnMap> = Mutex::new(ConnMap {
        map: HashMap::new(),
    });
}

#[wasm_bindgen]
pub async fn dispatch2(db: String, rpc: String, args: String) -> Result<JsValue, JsValue> {
    match rpc.as_str() {
        "open" => do_open(db)
            .await
            .map_err(|e| format!("{:?}", e).into())
            .map(|_| JsValue::undefined()),
        _ => unimplemented!(),
    }
}

#[derive(Debug)]
enum OpenError {
    DbNameMustBeNonEmpty,
    IdbStoreNewError(kv::StoreError),
    NoIdbStoreReturned,
}

async fn do_open(db_name: String) -> Result<(), OpenError> {
    use OpenError::*;

    if db_name.is_empty() {
        return Err(DbNameMustBeNonEmpty);
    }

    let mut conns = connections.lock().await;

    if conns.map.contains_key(&db_name) {
        return Ok(());
    }

    let kvs: Box<dyn kv::Store> = match db_name.as_str() {
        #[cfg(not(target_arch = "wasm32"))]
        "mem" => Box::new(crate::kv::memstore::MemStore::new()),
        _ => Box::new(
            IdbStore::new(&db_name)
                .await
                .map_err(IdbStoreNewError)?
                .ok_or(NoIdbStoreReturned)?,
        ),
    };

    let ds = dag::Store::new(kvs);
    conns.map.insert(db_name, RwLock::new(ds));
    Ok(())
}

use crate::db;
use embed::types::ScanRequest;
use nanoserde::{DeJson, DeJsonErr};

#[derive(Debug)]
enum ScanError {
    DbNameMustBeNonEmpty,
    DbNotOpen(String),
    OpenReadTransactionError(dag::Error),
    InvalidJson(DeJsonErr),
}

async fn do_scan(db_name: String, args: String) -> Result<Vec<ScanItem>, ScanError> {
    use ScanError::*;

    if db_name.is_empty() {
        return Err(DbNameMustBeNonEmpty);
    }

    let req: ScanRequest = DeJson::deserialize_json(&args).map_err(InvalidJson)?;

    let conns = connections.lock().await;
    let ds = conns.map.get(&db_name).ok_or_else(|| DbNotOpen(db_name))?;
    let ds = ds.read().await;
    let dr = ds.read().await.map_err(OpenReadTransactionError)?;

    // TODO: Need to either get the existing open tx from dispatch
    // Or else have JS not use transactions for this test

    let read = db::Read::new(dr.read());

    let mut res = Vec::<ScanItem>::new();
    for pe in read.scan((&req.opts).into()) {
        //res.push(ScanItem::try_from(pe).map_err(|e| format!("{:?}", e))?);
    }
    //Ok(ScanResponse { items: res })

    unimplemented!()
}

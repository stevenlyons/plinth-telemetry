#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;

use crate::{
    beacon::BeaconBatch,
    config::Config,
    event::PlayerEvent,
    session::{Session, SessionMeta},
};

#[wasm_bindgen]
pub struct WasmSession {
    inner: Session,
}

#[wasm_bindgen]
impl WasmSession {
    #[wasm_bindgen(constructor)]
    pub fn new(config_json: &str, meta_json: &str, now_ms: f64) -> Result<WasmSession, JsValue> {
        let config: Config = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("config parse error: {e}")))?;
        let meta: SessionMeta = serde_json::from_str(meta_json)
            .map_err(|e| JsValue::from_str(&format!("meta parse error: {e}")))?;
        Ok(WasmSession {
            inner: Session::new(config, meta, now_ms as u64),
        })
    }

    pub fn process_event(&mut self, event_json: &str, now_ms: f64) -> Result<String, JsValue> {
        let event: PlayerEvent = serde_json::from_str(event_json)
            .map_err(|e| JsValue::from_str(&format!("event parse error: {e}")))?;
        let beacons = self.inner.process_event(event, now_ms as u64);
        serde_json::to_string(&BeaconBatch::new(beacons))
            .map_err(|e| JsValue::from_str(&format!("serialize error: {e}")))
    }

    pub fn tick(&mut self, now_ms: f64) -> Result<String, JsValue> {
        let beacons = self.inner.tick(now_ms as u64);
        serde_json::to_string(&BeaconBatch::new(beacons))
            .map_err(|e| JsValue::from_str(&format!("serialize error: {e}")))
    }

    pub fn destroy(&mut self, now_ms: f64) -> Result<String, JsValue> {
        let beacons = self.inner.destroy(now_ms as u64);
        serde_json::to_string(&BeaconBatch::new(beacons))
            .map_err(|e| JsValue::from_str(&format!("serialize error: {e}")))
    }

    pub fn set_playhead(&mut self, playhead_ms: f64) {
        self.inner.set_playhead(playhead_ms as u64);
    }
}

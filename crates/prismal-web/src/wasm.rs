//! JavaScript bindings of the player (built for `wasm32` with `wasm-bindgen --target web`).
//! Every value crosses the boundary as a JSON string; failures are thrown as JSON strings.

use crate::{examples, Player};
use wasm_bindgen::prelude::*;

fn s(v: serde_json::Value) -> String {
    v.to_string()
}

/// The embedded reference programs: `[{key, title, source}]`.
#[wasm_bindgen]
pub fn examples() -> String {
    s(serde_json::Value::Array(examples::all().into_iter().map(|e| serde_json::json!({ "key": e.key, "title": e.title, "source": e.source })).collect()))
}

#[wasm_bindgen]
pub struct WebPlayer(Player);

#[wasm_bindgen]
impl WebPlayer {
    /// Compiles source text; throws the located diagnostics.
    #[wasm_bindgen(constructor)]
    pub fn new(src: &str) -> Result<WebPlayer, JsValue> {
        Player::load(src).map(WebPlayer).map_err(|d| JsValue::from_str(&d.to_string()))
    }
    pub fn formatted(&self) -> String {
        self.0.formatted()
    }
    pub fn catalogue(&self) -> String {
        s(self.0.catalogue())
    }
    pub fn open(&mut self, presentation: &str, video: bool) -> Result<String, JsValue> {
        self.0.open(presentation, video).map(s).map_err(|d| JsValue::from_str(&d.to_string()))
    }
    pub fn layout(&self) -> String {
        s(self.0.layout())
    }
    pub fn frame(&self, p: f64, dt: f64) -> String {
        s(self.0.frame(p, dt))
    }
    pub fn observations(&self) -> String {
        s(self.0.observations())
    }
    pub fn set_control(&mut self, rep: &str, value: f64) -> String {
        s(self.0.set_control(rep, value))
    }
    pub fn key(&mut self, rep: &str, key: &str) -> String {
        s(self.0.key(rep, key))
    }
    pub fn pointer_down(&mut self, rep: &str, part: Option<String>) -> String {
        s(self.0.pointer_down(rep, part.as_deref()))
    }
    pub fn pointer_move(&mut self, x: f64, y: f64) -> String {
        s(self.0.pointer_move(x, y))
    }
    pub fn pointer_up(&mut self) -> String {
        s(self.0.pointer_up())
    }
    pub fn cancel(&mut self) {
        self.0.cancel()
    }
    pub fn undo(&mut self) {
        self.0.undo()
    }
    pub fn redo(&mut self) {
        self.0.redo()
    }
    pub fn lesson_set_control(&mut self, p: f64, rep: &str, value: f64) -> Result<String, JsValue> {
        self.0.lesson_set_control(p, rep, value).map(s).map_err(|d| JsValue::from_str(&d.to_string()))
    }
    pub fn lesson_continue(&mut self, p: f64) -> Result<String, JsValue> {
        self.0.lesson_continue(p).map(s).map_err(|d| JsValue::from_str(&d.to_string()))
    }
    pub fn lesson_restart(&mut self) -> Result<String, JsValue> {
        self.0.lesson_restart().map(s).map_err(|d| JsValue::from_str(&d.to_string()))
    }
    pub fn seek(&mut self, t: f64) -> String {
        s(self.0.seek(t))
    }
    pub fn reset(&mut self) -> String {
        s(self.0.reset())
    }
    pub fn session(&self) -> String {
        s(self.0.session())
    }
    pub fn run_cases(&self) -> String {
        s(self.0.run_cases())
    }
    pub fn locate(&self, element: &str) -> String {
        s(self.0.locate(element))
    }
}

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
  Ok(())
}

#[wasm_bindgen]
pub fn create() -> Result<(), JsValue> {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  let canvas = document.create_element("canvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
  canvas.set_width(500);
  canvas.set_height(500);

  // Manufacture the element we're gonna append
  let p = document.create_element("p")?;
  p.set_text_content(Some("Hello from Rust!"));

  body.append_child(&canvas)?;
  body.append_child(&p)?;
  Ok(())
}

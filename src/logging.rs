use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => ($crate::logging::log(&format_args!($($t)*).to_string()))
}

pub fn logwithdiv(message: &str) {
  let href = web_sys::window()
    .unwrap()
    .location()
    .href()
    .expect("web_sys::window().unwrap().location().href() failed");
  if href.contains("debug") {
    let document = web_sys::window().unwrap().document().unwrap();
    let consolelog_div = document.get_element_by_id("consolelog-div").unwrap();
    let consolelog_div = consolelog_div
      .dyn_into::<web_sys::HtmlDivElement>()
      .expect("failed to get logdiv.");
    consolelog_div.set_inner_text(&format!("{}\n{}", message, consolelog_div.inner_text()));
    consolelog_div
      .style()
      .set_property("display", "block")
      .expect("consolelog_div.style().set_property failed");
    consolelog_div.set_class_name("logdiv");
  }

  console_log!("{}", message);
}

macro_rules! console_log_with_div {
    ($($t:tt)*) => ($crate::logging::logwithdiv(&format_args!($($t)*).to_string()))
}

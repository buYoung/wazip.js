mod utils;
mod zip;

use wasm_bindgen::prelude::*;
use crate::zip::WaZip;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// wasm이 로드되면 error가 발생할 경우 브라우저 콘솔에 개발자가 보기 편하게 에러를 출력하도록 설정
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}

#[wasm_bindgen]
pub fn greet() {
    WaZip::new(Option::from(6));
}

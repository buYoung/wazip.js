use std::io::Write;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use zip::{CompressionMethod};
use zip::write::FileOptions;

#[wasm_bindgen]
pub struct WaZip {
    zip: zip::ZipWriter<std::io::Cursor<Vec<u8>>>,
    options: FileOptions,
}


#[wasm_bindgen(js_class = WaZip)]
impl WaZip {
    #[wasm_bindgen(constructor)]
    pub fn new(compression_level: Option<i32>) -> WaZip {
        let cursor = std::io::Cursor::new(Vec::new());
        let _zip = zip::ZipWriter::new(cursor);

        let _compression_level = match compression_level {
            Some(level) => {
                if level < 0 {
                    0
                } else if level > 9 {
                    9
                } else {
                    level
                }
            },
            None => 6,
        };
        let options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .compression_level(Option::from(_compression_level))
            .unix_permissions(0o755);

        WaZip { zip: _zip, options: options}
    }

    pub async fn add_file(&mut self, name: &str, data: &[u8]) -> Result<(), JsValue> {
        if let Err(e) = self.zip.start_file(name, self.options) {
            return Err(JsValue::from_str(format!("[start_file] {}", &e.to_string()).as_str()));
        }
        if let Err(e) = self.zip.write_all(data) {
            return Err(JsValue::from_str(format!("[write_all] {}", &e.to_string()).as_str()));
        }

        Ok(())
    }

    pub async fn add_directory(&mut self, name: &str) -> Result<(), JsValue> {
        if let Err(e) = self.zip.add_directory(name, self.options) {
            return Err(JsValue::from_str(format!("[add_directory] {}", &e.to_string()).as_str()));
        }

        Ok(())
    }

    pub fn finish(&mut self) -> Result<Vec<u8>, JsValue> {
        let finish = self.zip.finish();
        if let Err(e) = finish {
            return Err(JsValue::from_str(format!("[finish] {}", &e.to_string()).as_str()));
        }

        let cursor = finish.unwrap();

        Ok(cursor.into_inner())
    }
}
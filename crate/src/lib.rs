extern crate chardet;
extern crate encoding;
extern crate wasm_bindgen;

use encoding::label::encoding_from_whatwg_label;
use encoding::DecoderTrap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DetectionResult {
    encoding: String,
    confidence: f32,
    decoded: Option<String>,
}

// wasm-bindgen requires public fields to be Copy, so we implement getters as a
// fallback.
#[wasm_bindgen]
impl DetectionResult {
    #[wasm_bindgen]
    pub fn encoding(&self) -> String {
        self.encoding.clone()
    }

    #[wasm_bindgen]
    pub fn confidence(&self) -> f32 {
        self.confidence
    }

    #[wasm_bindgen]
    pub fn decoded(&self) -> Option<String> {
        match &self.decoded {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }
}

#[wasm_bindgen]
pub fn detect_and_decode(payload: &[u8]) -> DetectionResult {
    let (charset, confidence, _) = chardet::detect(payload);
    let encoding = chardet::charset2encoding(&charset)
        .to_string()
        .to_uppercase();
    let decoded = encoding_from_whatwg_label(&encoding).map_or(None, |encoding| {
        encoding.decode(payload, DecoderTrap::Replace).ok()
    });
    DetectionResult {
        encoding,
        confidence,
        decoded,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! encoding_test {
        ( $test:ident, $encodings:expr, $encoded_file:expr, $decoded_file:expr $(,)* ) => {
            #[test]
            fn $test() {
                let result = detect_and_decode(include_bytes!($encoded_file));
                assert!(
                    $encodings.contains(&result.encoding.as_str()),
                    result.encoding
                );
                assert_eq!(
                    Some(include_str!($decoded_file).to_string()),
                    result.decoded
                );
            }
        };
    }

    encoding_test!{
        test_utf_8,
        &vec!["UTF-8"],
        "tests/UTF-8.txt",
        "tests/UTF-8-decoded.txt",
    }

    encoding_test!{
        test_utf_16be,
        &vec!["UTF-16BE", "UTF-16"],
        "tests/UTF-16BE.txt",
        "tests/UTF-16BE-decoded.txt",
    }

    encoding_test!{
        test_utf_16le,
        &vec!["UTF-16LE", "UTF-16"],
        "tests/UTF-16LE.txt",
        "tests/UTF-16LE-decoded.txt",
    }

    encoding_test!{
        test_gbk,
        &vec!["GBK", "GB2312", "GB18030"],
        "tests/GBK.txt",
        "tests/GBK-decoded.txt",
    }

    encoding_test!{
        test_windows_1252, // aka Latin-1, ISO-8859-1
        &vec!["WINDOWS-1252", "ISO-8859-1"],
        "tests/Windows-1252.txt",
        "tests/Windows-1252-decoded.txt",
    }
}

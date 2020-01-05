#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;

use sudachi::morpheme::Morpheme;
use sudachi::tokenizer::{Mode, Tokenizer};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn analyze(src: &str) -> String {
    let morpheme_list = morphemes(src);
    let dst = morpheme_list
        .into_iter()
        .map(|morpheme| {
            format!(
                "{}\t{}\t{}\t{}\t{}",
                morpheme.surface(),
                morpheme.pos().join(","),
                morpheme.normalized_form(),
                morpheme.dictionary_form(),
                morpheme.reading_form(),
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    dst
}

static BYTES: &'static [u8] = include_bytes!("resources/system.dic");
lazy_static! {
    static ref TOKENIZER: Tokenizer<'static> = Tokenizer::new(BYTES);
}

fn morphemes<'a>(src: &str) -> Vec<Morpheme<'a>> {
    let src = src.to_string();
    TOKENIZER.tokenize(&src, &Mode::C, false)
}

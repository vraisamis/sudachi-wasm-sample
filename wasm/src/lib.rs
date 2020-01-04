extern crate wasm_bindgen;

use wasm_bindgen::prelude::wasm_bindgen;
use sudachi::tokenizer::{Tokenizer, Mode};


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn analyze(src: &str) {
    let bytes = include_bytes!("resources/system.dic");
    let tokenizer = Tokenizer::new(bytes);
    let mode = Mode::C;
    let src = src.to_string();
    let morpheme_list = tokenizer.tokenize(&src, &mode, false);
    let dst = morpheme_list.into_iter().map(|morpheme| {
        format!(
                    "{}\t{}\t{}\t{}\t{}",
                    morpheme.surface(),
                    morpheme.pos().join(","),
                    morpheme.normalized_form(),
                    morpheme.dictionary_form(),
                    morpheme.reading_form(),
               )
    }).collect::<Vec<String>>().join("\n");
    alert(&dst);
}



#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

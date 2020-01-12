#![recursion_limit = "256"]

extern crate wasm_bindgen;

use std::panic::{catch_unwind, AssertUnwindSafe};
use sudachi::tokenizer::{Mode, Tokenizer};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::{html, App, Component, ComponentLink, Html, InputData, ShouldRender};

static BYTES: &'static [u8] = include_bytes!("resources/system.dic");

struct Token {
    surface: String,
    pos: String,
    normalized: String,
    dictionary: String,
    reading: String,
}

impl Token {
    fn to_tr(&self) -> Html {
        html! {
            <tr>
                <td>{ self.surface.clone() }</td>
                <td>{ self.pos.clone() }</td>
                <td>{ self.normalized.clone() }</td>
                <td>{ self.dictionary.clone() }</td>
                <td>{ self.reading.clone() }</td>
            </tr>
        }
    }
}
struct Model {
    tokenizer: Tokenizer<'static>,
    text: String,
    tokens: Vec<Token>,
    err: String,
    link: ComponentLink<Self>,
}
enum Msg {
    Change(String),
}

impl Model {
    fn tokenize(&mut self) {
        match catch_unwind(AssertUnwindSafe(|| {
            self.tokenizer.tokenize(&self.text, &Mode::C, false)
        })) {
            Ok(tokens) => {
                self.tokens = tokens
                    .into_iter()
                    .map(|morpheme| Token {
                        surface: morpheme.surface().to_string(),
                        pos: morpheme.pos().join(","),
                        normalized: morpheme.normalized_form().to_string(),
                        dictionary: morpheme.dictionary_form().to_string(),
                        reading: morpheme.reading_form().to_string(),
                    })
                    .collect::<Vec<Token>>();
                self.err = "".to_string();
            }
            Err(_) => {
                self.tokens = vec![];
                self.err = "tokenize error".into();
            }
        }
    }
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            tokenizer: Tokenizer::new(BYTES),
            text: "".into(),
            tokens: vec![],
            err: "".into(),
            link: link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            Change(s) => {
                self.text = s;
                self.tokenize();
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Sudachi wasm yew サンプル"}</h1>
                <input type="textarea"
                    oninput = self.link.callback(|e: InputData| Msg::Change(e.value))
                    value = {&self.text}
                    />
                <p>{self.tokens.len()}</p>
                <p>{self.text.clone()}</p>
                <table>
                    <thead>
                        <tr>
                            <th>{"surface"}</th>
                            <th>{"pos"}</th>
                            <th>{"normalized"}</th>
                            <th>{"dictionary"}</th>
                            <th>{"reading"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { self.tokens.iter().map(|t| t.to_tr()).collect::<Html>() }
                    </tbody>
                </table>
                <div>{ self.err.clone() }</div>
            </>
        }
    }
}

#[wasm_bindgen]
pub fn run() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tokenize() {
        let tokenizer = Tokenizer::new(BYTES);
        let s = "？？？".to_string();
        let opt = catch_unwind(AssertUnwindSafe(|| tokenizer.tokenize(&s, &Mode::C, true)));
        assert_eq!(opt.is_err(), true);

        let s = "日本語".to_string();
        let opt = catch_unwind(AssertUnwindSafe(|| tokenizer.tokenize(&s, &Mode::C, true)));
        assert_eq!(opt.is_ok(), true);
    }
}

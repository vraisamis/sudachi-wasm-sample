#![recursion_limit = "1024"]

extern crate wasm_bindgen;

use sudachi::tokenizer::{Mode, Tokenizer};
use unic_normal::StrNormalForm;
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

struct Model {
    tokenizer: Tokenizer<'static>,
    text: String,
    tokens: Vec<Token>,
    link: ComponentLink<Self>,
}
enum Msg {
    Change(String),
}

impl Model {
    fn tokenize(&mut self) {
        let text = self.text.nfkc().collect::<String>();
        let tokens = self.tokenizer.tokenize(&text, &Mode::C, false);
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
                        {
                            self.tokens.iter().map(|t| {
                                html! {
                                    <tr>
                                        <td>{ t.surface.clone() }</td>
                                        <td>{ t.pos.clone() }</td>
                                        <td>{ t.normalized.clone() }</td>
                                        <td>{ t.dictionary.clone() }</td>
                                        <td>{ t.reading.clone() }</td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </tbody>
                </table>
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

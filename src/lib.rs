use nlprule::{Rules, Tokenizer};
use wasm_bindgen::prelude::*;

mod utils;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    utils::set_panic_hook();

    let mut tokenizer_bytes: &'static [u8] = include_bytes!("../binaries/en_tokenizer.bin");
    let mut rules_bytes: &'static [u8] = include_bytes!("../binaries/en_rules.bin");

    let tokenizer =
        Tokenizer::from_reader(&mut tokenizer_bytes).expect("tokenizer binary is valid");
    let rules = Rules::from_reader(&mut rules_bytes).expect("rules binary is valid");

    assert_eq!(
        rules.correct("She was not been here since Monday.", &tokenizer),
        String::from("She was not here since Monday.")
    );

    let corrected = rules.correct("She was not been here since Monday.", &tokenizer);

    alert(&corrected);
}

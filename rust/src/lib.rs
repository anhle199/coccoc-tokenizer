use cxx::{CxxString, CxxVector};

#[cxx::bridge]
pub mod tokenizer {
    extern "Rust" {
        fn convert_to_string_vector(vec: &CxxVector<CxxString>) -> Vec<String>;
    }

    unsafe extern "C++" {
        include!("coccoc-tokenizer/tokenizer/tokenizer.hpp");

        fn initialize(dict_path: &CxxString, load_nontone_data: bool) -> i32;
        fn word_tokenize(text: &CxxString, tokenize_option: i32) -> Vec<String>;
    }
}

fn convert_to_string_vector(vec: &CxxVector<CxxString>) -> Vec<String> {
    vec.iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect()
}

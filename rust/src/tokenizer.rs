use std::time::Instant;

use cxx::{CxxString, CxxVector, let_cxx_string};

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn convert_to_string_vector(vec: &CxxVector<CxxString>) -> Vec<String>;
    }

    unsafe extern "C++" {
        include!("coccoc-tokenizer/tokenizer/tokenizer.hpp");

        fn initialize(dict_path: &CxxString, load_nontone_data: bool) -> i32;
        fn segment_original(text: &CxxString) -> Vec<String>;
    }
}

fn convert_to_string_vector(vec: &CxxVector<CxxString>) -> Vec<String> {
    vec.iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect()
}

pub struct CocCocTokenizer;

impl CocCocTokenizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn initialize(dict_path: &str, load_nontone_data: bool) -> i32 {
        let start_time = Instant::now();
        println!("start initializing Vietnamese dictionary...");
        let_cxx_string!(cxx_dict_path = dict_path);
        let init_res = ffi::initialize(&cxx_dict_path, load_nontone_data);
        let duration = start_time.elapsed();
        println!("initialized Vietnamese dictionary in {:?}", duration);
        return init_res;
    }

    pub fn segment_original(&self, text: &str) -> Vec<String> {
        let_cxx_string!(cxx_text = text);
        return ffi::segment_original(&cxx_text);
    }

    pub fn tokenize(&self, text: &str) -> String {
        return self.segment_original(text).join("\t");
    }
}

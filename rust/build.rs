use std::path::PathBuf;

fn main() {
    cxx_build::bridge("src/tokenizer.rs")
        .file("tokenizer/tokenizer.cpp")
        .cpp(true)
        .std("c++11")
        .include(PathBuf::from("/Users/lehoanganh/.local/include"))
        .compile("coccoc-tokenizer");

    println!("cargo:rerun-if-changed=tokenizer/tokenizer.cpp");
    println!("cargo:rerun-if-changed=tokenizer/tokenizer.hpp");
}

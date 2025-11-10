#include "tokenizer.hpp"
#include "coccoc-tokenizer/src/tokenizer.rs.h"
#include "rust/cxx.h"
#include <tokenizer/tokenizer.hpp>
#include <tokenizer/token.hpp>

int32_t initialize(const std::string &dict_path, bool load_nontone_data = true) {
    return Tokenizer::instance().initialize(dict_path, load_nontone_data);
}

rust::Vec<rust::String> segment_original(const std::string &text) {
    std::vector<FullToken> tokens = Tokenizer::instance().segment_keep_puncts(text);

    rust::Vec<rust::String> rust_tokens;
    rust_tokens.reserve(tokens.size());
    for (auto it : tokens) {
        rust_tokens.push_back(it.text);
    }
    return rust_tokens;
}

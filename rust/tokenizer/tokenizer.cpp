#include "tokenizer.hpp"
#include "coccoc-tokenizer/src/lib.rs.h"
#include "rust/cxx.h"
#include <tokenizer/tokenizer.hpp>

int32_t initialize(const std::string &dict_path, const bool &load_nontone_data = true) {
	return Tokenizer::instance().initialize(dict_path, load_nontone_data);
}

rust::Vec<rust::String> word_tokenize(const std::string &text, const int32_t &tokenize_option) {
	std::vector<std::string> tokens = Tokenizer::instance()
	    .segment_to_string_list(text, false, tokenize_option);
	return convert_to_string_vector(tokens);
}

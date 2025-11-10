#include "rust/cxx.h"

int32_t initialize(const std::string &dict_path, bool load_nontone_data);
rust::Vec<rust::String> word_tokenize(const std::string &text, int32_t tokenize_option);

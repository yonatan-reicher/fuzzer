#include <filesystem>
#include <iostream>
#include <fstream>
#include <set>
#include <vector>

namespace fs = std::filesystem;

// Assume the current working directory is the root of our project (Cargo.toml
// directory)
constexpr const char* const words_path = "./resources/test_output_distribution/words.txt";

using Word = std::vector<char>;
using Words = std::multiset<Word>;

bool are_we_in_root() {
    constexpr const char* root_marker = "Cargo.toml";
    return fs::exists(root_marker); 
}

Words read_old_words() {
    // Words is a series of (len, binary string) pairs
    Words words;
    uint64_t len;

    std::ifstream words_file(words_path, std::ios::binary);
    while (words_file.read(reinterpret_cast<char*>(&len), sizeof(len))) {
        Word word(len);
        words_file.read(word.data(), len);
        words.insert(std::move(word));
    }
    return words;
}

void write_words(const Words& words) {
    std::ofstream words_file(words_path, std::ios::binary);
    for (const Word& word : words) {
        uint64_t len = word.size();
        words_file.write(reinterpret_cast<const char*>(&len), sizeof(len));
        words_file.write(word.data(), len);
    }
}

Word read_new_word() {
    constexpr size_t bufferSize = 1024;
    char buffer[bufferSize];
    while (true) {
        std::cin.read(buffer, bufferSize);
        if (std::cin.eof()) {
            break;
        }
    }
    return Word(buffer, buffer + std::cin.gcount());
}

int main() {
    if (!are_we_in_root()) {
        std::cerr << "Please run this program from the root of the project" << std::endl;
        // Return 0, so our fuzzer fails this benchmark
        return 0;
    }

    if (!fs::exists(words_path)) {
        std::ofstream words_file(words_path);
    }

    Words words = read_old_words();
    Word word = read_new_word();
    words.insert(word);
    write_words(words);

    return 0;
}

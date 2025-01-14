#include <iostream>
#include <string>
#include <cstdlib>

void vulnerable_function(const std::string &input) {
    int value = std::atoi(input.c_str());
    if (value == 0) {
        throw std::runtime_error("Divide by zero error");
    }
    std::cout << "Result: " << (100 / value) << std::endl;
}

int main() {
    std::string input;
    std::cout << "Enter a number: ";
    std::cin >> input;
    try {
        vulnerable_function(input);
    } catch (const std::exception &e) {
        std::cerr << e.what() << std::endl;
        return -1;
    }
    return 0;
}


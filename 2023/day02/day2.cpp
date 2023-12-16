#include <fstream>
#include <iostream>

#include "Game.hpp"

int main() {
    std::ifstream infile("input");
    int validGameIdSum{0};
    int powerSum{0};
    std::string line;

    while (std::getline(infile, line)) {
        Game game{line};

        if (game.isValid(12, 13, 14)) {
            validGameIdSum += game.id;
            std::cout << "VALID: ";
        } else {
            std::cout << "INVALID: ";
        }

        std::cout << line << '\n';
        powerSum += game.power();
    }

    std::cout << validGameIdSum << '\n';
    std::cout << powerSum << '\n';

    return 0;
}
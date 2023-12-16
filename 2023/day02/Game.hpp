#ifndef GAME_HPP
#define GAME_HPP

#include <iostream>
#include <string>
#include <vector>

#include "Showing.hpp"

struct Game {
    int id{};
    std::vector<Showing> showings{};

    int minRedNeeded{};
    int minGreenNeeded{};
    int minBlueNeeded{};

    Game(const std::string& gameStr);

    inline bool isValid(int red, int green, int blue) const {
        for (const auto& showing : showings) {
            if (!showing.isValid(red, green, blue)) return false;
        }

        return true;
    }

    inline int power() { return minRedNeeded * minGreenNeeded * minBlueNeeded; }
};

#endif
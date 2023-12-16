#include "Game.hpp"

#include <iostream>
#include <sstream>
#include <string>

#include "StringSplit.hpp"

Game::Game(const std::string& gameStr) {
    auto splits{StringSplit::split(gameStr, ':')};

    auto& gameDesc{splits[0]};
    auto gameId{std::stoi(StringSplit::split(gameDesc, ' ')[1])};

    id = gameId;

    auto showingsStrings{StringSplit::split(splits[1], ';')};

    for (auto showing : showingsStrings) {
        Showing currShowing{showing};

        if (currShowing.red > minRedNeeded) {
            minRedNeeded = currShowing.red;
        }

        if (currShowing.green > minGreenNeeded) {
            minGreenNeeded = currShowing.green;
        }

        if (currShowing.blue > minBlueNeeded) {
            minBlueNeeded = currShowing.blue;
        }

        showings.push_back(currShowing);
    }
}
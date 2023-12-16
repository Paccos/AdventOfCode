#include "Showing.hpp"

#include <algorithm>
#include <cstdlib>
#include <iostream>
#include <string_view>
#include <vector>

#include "StringSplit.hpp"

std::tuple<int, std::string> splitColorAndNum(
    const std::string& colorAndNumStr) {
    auto numAndColorSplit{StringSplit::split(colorAndNumStr, ' ')};

    int num{};
    std::string colorStr{numAndColorSplit[1]};

    try {
        num = std::stoi(numAndColorSplit[0]);
        colorStr = numAndColorSplit[1];
    } catch (std::invalid_argument) {
        num = std::stoi(numAndColorSplit[1]);
        colorStr = numAndColorSplit[0];
    }

    std::transform(colorStr.begin(), colorStr.end(), colorStr.begin(),
                   [](unsigned char c) { return std::tolower(c); });

    return {num, colorStr};
}

Showing::Showing(const std::string& showingStr) {
    auto cubeDescs{StringSplit::split(showingStr, ',')};

    for (const auto& cubeDesc : cubeDescs) {
        auto numAndColor{splitColorAndNum(cubeDesc)};

        auto num{std::get<0>(numAndColor)};
        auto color{std::get<1>(numAndColor)};

        if (color == "red") {
            red = num;
        } else if (color == "green") {
            green = num;
        } else if (color == "blue") {
            blue = num;
        }
    }

    std::cout << red << ',' << green << ',' << blue << '\n';
}
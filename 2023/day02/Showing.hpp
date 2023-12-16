#ifndef SHOWING_HPP
#define SHOWING_HPP

#include <string_view>

struct Showing {
    int red{};
    int green{};
    int blue{};

    Showing(const std::string& showingStr);

    inline bool isValid(int redBag, int greenBag, int blueBag) const {
        return red <= redBag && green <= greenBag && blue <= blueBag;
    }
};

#endif
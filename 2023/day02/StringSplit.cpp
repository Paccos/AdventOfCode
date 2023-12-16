#include "StringSplit.hpp"

#include <sstream>

std::vector<std::string> StringSplit::split(const std::string& str,
                                            char delim) {
    std::istringstream iss{str};
    std::string token;
    std::vector<std::string> result{};

    while (std::getline(iss, token, delim)) {
        result.push_back(token);
        iss >> std::ws;
    }

    return result;
}
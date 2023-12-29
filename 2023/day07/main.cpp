#include <fstream>
#include <iostream>

#include "Hand.hpp"
#include "StringSplit.hpp"

int main(int argc, char** args) {
    bool jokerRule{false};

    if (argc == 2) {
        if (strcmp(args[1], "-j") && strcmp(args[1], "--joker")) {
            std::cout << "Usage: " << args[0] << " [-j | --joker]" << '\n';

            return -1;
        }

        jokerRule = true;
    }

    std::ifstream infile("input");
    std::string line;

    std::vector<Hand> hands{};

    while (std::getline(infile, line)) {
        auto handAndBid{StringSplit::split(line, ' ')};

        hands.push_back(Hand(handAndBid[0], std::stoi(handAndBid[1])));
    }

    std::sort(hands.begin(), hands.end(),
              [jokerRule](const Hand& a, const Hand& b) {
                  return a.lessThan(b, jokerRule);
              });

    long rankedBiddingsSum{};

    for (auto i{0}; i < hands.size(); ++i) {
        rankedBiddingsSum += (i + 1) * hands[i].getBid();
        std::cout << hands[i] << '\n';
    }

    std::cout << "Result: " << rankedBiddingsSum << '\n';

    return 0;
}
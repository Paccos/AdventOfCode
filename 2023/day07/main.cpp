#include <fstream>
#include <iostream>

#include "Hand.hpp"
#include "StringSplit.hpp"

int main() {
    std::ifstream infile("input");
    std::string line;

    std::vector<Hand> hands{};

    while (std::getline(infile, line)) {
        auto handAndBid{StringSplit::split(line, ' ')};

        hands.push_back(Hand(handAndBid[0], std::stoi(handAndBid[1])));
    }

    std::sort(hands.begin(), hands.end(),
              [](const Hand& a, const Hand& b) { return a < b; });

    long rankedBiddingsSum{};

    for (auto i{0}; i < hands.size(); ++i) {
        rankedBiddingsSum += (i + 1) * hands[i].getBid();
        std::cout << hands[i] << '\n';
    }

    std::cout << "Result: " << rankedBiddingsSum << '\n';

    return 0;
}
#include "Hand.hpp"

#include <algorithm>
#include <map>

Rank twoOrOnePair(const std::map<char, int>& cardAmounts) {
    int pairs{};

    for (const auto& kv : cardAmounts) {
        if (kv.second == 2) ++pairs;
    }

    if (pairs == 2) return Rank::twoPair;

    return Rank::onePair;
}

Rank Hand::getRank() const {
    std::map<char, int> cardAmounts{};
    for (const auto& c : m_hand) {
        if (cardAmounts.contains(c)) {
            cardAmounts[c] += 1;
        } else {
            cardAmounts.insert({c, 1});
        }
    }

    int maxAmount{
        std::max_element(
            std::begin(cardAmounts), std::end(cardAmounts),
            [](const auto& a, const auto& b) { return a.second < b.second; })
            ->second};

    switch (maxAmount) {
        case 5:
            return Rank::fiveOfAKind;
        case 4:
            return Rank::fourOfAKind;
        case 3:
            if (cardAmounts.size() == 2)
                return Rank::fullHouse;
            else
                return Rank::threeOfAKind;
        case 2:
            return twoOrOnePair(cardAmounts);
        default:
            return Rank::highCard;
    }
}

// comparison works like "is a < b"
bool compareCards(char a, char b) {
    constexpr char rankIndex[]{'A', 'K', 'Q', 'J', 'T', '9', '8',
                               '7', '6', '5', '4', '3', '2'};

    std::size_t aIndex{};
    std::size_t bIndex{};

    for (std::size_t i{0}; i < std::size(rankIndex); ++i) {
        if (rankIndex[i] == a) {
            aIndex = i;
        }

        if (rankIndex[i] == b) {
            bIndex = i;
        }
    }

    return aIndex > bIndex;
}

bool Hand::operator<(const Hand& other) const {
    Rank thisRank{this->getRank()};
    Rank otherRank{other.getRank()};

    if (thisRank != otherRank) return thisRank > otherRank;

    // Compare cards for tiebreak. Order of cards in hands matters, i.e. if
    // second card of this hand is higher than second card of other hand, this
    // hand is ranked higher

    for (std::size_t i{0}; i < 5; ++i) {
        char thisCard{this->getDesc().at(i)};
        char otherCard(other.getDesc().at(i));

        if (thisCard != otherCard) {
            return compareCards(thisCard, otherCard);
        }
    }

    return false;
}

Hand& Hand::operator=(const Hand& other) {
    this->m_hand = other.m_hand;
    this->m_bid = other.m_bid;

    return *this;
}

std::ostream& operator<<(std::ostream& os, const Rank rank) {
    switch (rank) {
        case Rank::fiveOfAKind:
            os << "Five of a kind";
            break;

        case Rank::fourOfAKind:
            os << "Four of a kind";
            break;

        case Rank::fullHouse:
            os << "Full House";
            break;

        case Rank::highCard:
            os << "High Card";
            break;

        case Rank::onePair:
            os << "One Pair";
            break;

        case Rank::threeOfAKind:
            os << "Three of a kind";
            break;

        case Rank::twoPair:
            os << "Two Pair";
            break;

        default:
            break;
    }

    return os;
}
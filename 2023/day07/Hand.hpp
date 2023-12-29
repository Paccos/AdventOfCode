#ifndef __HAND_HPP
#define __HAND_HPP

#include <ostream>
#include <string>

enum class Rank {
    fiveOfAKind,
    fourOfAKind,
    fullHouse,
    threeOfAKind,
    twoPair,
    onePair,
    highCard
};

std::ostream& operator<<(std::ostream& os, const Rank rank);

class Hand {
   private:
    std::string m_hand{};
    int m_bid{};

   public:
    Hand(const std::string handStr, const int bid)
        : m_hand(handStr), m_bid(bid){};

    Hand(const Hand& other) : m_hand(other.m_hand), m_bid(other.m_bid){};

    Rank getRank() const;
    inline int getBid() const { return m_bid; };
    inline const std::string_view getDesc() const { return m_hand; }

    bool operator<(const Hand& other) const;
    inline bool operator>(const Hand& other) const { return other < *this; }

    Hand& operator=(const Hand& other);

    inline friend std::ostream& operator<<(std::ostream& os, const Hand& hand) {
        os << "Hand: " << hand.m_hand << " - Bid: " << hand.m_bid
           << " - Rank: " << hand.getRank();

        return os;
    }
};

#endif
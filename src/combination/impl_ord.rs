use std::cmp::Ordering;

use crate::card::Rank;
use crate::combination::Combination;

use ::itertools::Itertools;

impl PartialOrd for Combination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::HighCard { rank: rank_a } => match other {
                Self::HighCard { rank: rank_b } => rank_a.partial_cmp(rank_b),
                Self::Pair { .. } => Some(Ordering::Less),
                Self::TwoPairs { .. } => Some(Ordering::Less),
                Self::ThreeOfAKind { .. } => Some(Ordering::Less),
                Self::Straight { .. } => Some(Ordering::Less),
                Self::Flush { .. } => Some(Ordering::Less),
                Self::FullHouse { .. } => Some(Ordering::Less),
                Self::FourOfAKind { .. } => Some(Ordering::Less),
                Self::StraightFlush { .. } => Some(Ordering::Less),
            },
            Self::Pair {
                rank: pair_rank_a,
                extra: extra_a,
            } => match other {
                Self::HighCard { .. } => Some(Ordering::Greater),
                Self::Pair {
                    rank: pair_rank_b,
                    extra: extra_b,
                } => match pair_rank_a.partial_cmp(pair_rank_b) {
                    Some(Ordering::Equal) => Some(compare_extra(extra_a, extra_b)),
                    ord => ord,
                },
                Self::TwoPairs { .. } => Some(Ordering::Less),
                Self::ThreeOfAKind { .. } => Some(Ordering::Less),
                Self::Straight { .. } => Some(Ordering::Less),
                Self::Flush { .. } => Some(Ordering::Less),
                Self::FullHouse { .. } => Some(Ordering::Less),
                Self::FourOfAKind { .. } => Some(Ordering::Less),
                Self::StraightFlush { .. } => Some(Ordering::Less),
            },
            Self::TwoPairs {
                low: low_pair_rank_a,
                high: high_pair_rank_a,
                extra: extra_a,
            } => match other {
                Self::HighCard { .. } => Some(Ordering::Greater),
                Self::Pair { .. } => Some(Ordering::Greater),
                Self::TwoPairs {
                    low: low_pair_rank_b,
                    high: high_pair_rank_b,
                    extra: extra_b,
                } => match high_pair_rank_a.partial_cmp(high_pair_rank_b) {
                    Some(Ordering::Equal) => match low_pair_rank_a.partial_cmp(low_pair_rank_b) {
                        Some(Ordering::Equal) => Some(compare_extra(extra_a, extra_b)),
                        ord => ord,
                    },
                    ord => ord,
                },
                Self::ThreeOfAKind { .. } => Some(Ordering::Less),
                Self::Straight { .. } => Some(Ordering::Less),
                Self::Flush { .. } => Some(Ordering::Less),
                Self::FullHouse { .. } => Some(Ordering::Less),
                Self::FourOfAKind { .. } => Some(Ordering::Less),
                Self::StraightFlush { .. } => Some(Ordering::Less),
            },
            Self::ThreeOfAKind {
                rank: rank_a,
                extra: extra_a,
            } => match other {
                Self::HighCard { .. } => Some(Ordering::Greater),
                Self::Pair { .. } => Some(Ordering::Greater),
                Self::TwoPairs { .. } => Some(Ordering::Greater),
                Self::ThreeOfAKind {
                    rank: rank_b,
                    extra: extra_b,
                } => match rank_a.partial_cmp(rank_b) {
                    Some(Ordering::Equal) => Some(compare_extra(extra_a, extra_b)),
                    ord => ord,
                },
                Self::Straight { .. } => Some(Ordering::Less),
                Self::Flush { .. } => Some(Ordering::Less),
                Self::FullHouse { .. } => Some(Ordering::Less),
                Self::FourOfAKind { .. } => Some(Ordering::Less),
                Self::StraightFlush { .. } => Some(Ordering::Less),
            },
            Self::Straight { rank: rank_a } => match other {
                Self::HighCard { .. } => Some(Ordering::Greater),
                Self::Pair { .. } => Some(Ordering::Greater),
                Self::TwoPairs { .. } => Some(Ordering::Greater),
                Self::ThreeOfAKind { .. } => Some(Ordering::Greater),
                Self::Straight { rank: rank_b } => match rank_a {
                    Rank::Ace => match rank_b {
                        Rank::Ace => Some(Ordering::Equal),
                        _ => Some(Ordering::Less),
                    },
                    rank_a => match rank_b {
                        Rank::Ace => Some(Ordering::Equal),
                        rank_b => rank_a.partial_cmp(rank_b),
                    },
                },
                Self::Flush { .. } => Some(Ordering::Less),
                Self::FullHouse { .. } => Some(Ordering::Less),
                Self::FourOfAKind { .. } => Some(Ordering::Less),
                Self::StraightFlush { .. } => Some(Ordering::Less),
            },
            Self::Flush { rank: rank_a } => match other {
                Self::HighCard { .. } => Some(Ordering::Greater),
                Self::Pair { .. } => Some(Ordering::Greater),
                Self::TwoPairs { .. } => Some(Ordering::Greater),
                Self::ThreeOfAKind { .. } => Some(Ordering::Greater),
                Self::Straight { .. } => Some(Ordering::Greater),
                Self::Flush { rank: rank_b } => rank_a.partial_cmp(rank_b),
                Self::FullHouse { .. } => Some(Ordering::Less),
                Self::FourOfAKind { .. } => Some(Ordering::Less),
                Self::StraightFlush { .. } => Some(Ordering::Less),
            },
            Self::FullHouse {
                two: rank_two_a,
                three: rank_three_a,
            } => match other {
                Self::HighCard { .. } => Some(Ordering::Greater),
                Self::Pair { .. } => Some(Ordering::Greater),
                Self::TwoPairs { .. } => Some(Ordering::Greater),
                Self::ThreeOfAKind { .. } => Some(Ordering::Greater),
                Self::Straight { .. } => Some(Ordering::Greater),
                Self::Flush { .. } => Some(Ordering::Greater),
                Self::FullHouse {
                    two: rank_two_b,
                    three: rank_three_b,
                } => match rank_three_a.partial_cmp(rank_three_b) {
                    Some(Ordering::Equal) => rank_two_a.partial_cmp(rank_two_b),
                    ord => ord,
                },
                Self::FourOfAKind { .. } => Some(Ordering::Less),
                Self::StraightFlush { .. } => Some(Ordering::Less),
            },
            Self::FourOfAKind {
                rank: rank_a,
                extra: extra_a,
            } => match other {
                Self::HighCard { .. } => Some(Ordering::Greater),
                Self::Pair { .. } => Some(Ordering::Greater),
                Self::TwoPairs { .. } => Some(Ordering::Greater),
                Self::ThreeOfAKind { .. } => Some(Ordering::Greater),
                Self::Straight { .. } => Some(Ordering::Greater),
                Self::Flush { .. } => Some(Ordering::Greater),
                Self::FullHouse { .. } => Some(Ordering::Greater),
                Self::FourOfAKind {
                    rank: rank_b,
                    extra: extra_b,
                } => match rank_a.partial_cmp(rank_b) {
                    Some(Ordering::Equal) => Some(compare_extra(extra_a, extra_b)),
                    ord => ord,
                },
                Self::StraightFlush { .. } => Some(Ordering::Less),
            },
            Self::StraightFlush { rank: rank_a } => match other {
                Self::HighCard { .. } => Some(Ordering::Greater),
                Self::Pair { .. } => Some(Ordering::Greater),
                Self::TwoPairs { .. } => Some(Ordering::Greater),
                Self::ThreeOfAKind { .. } => Some(Ordering::Greater),
                Self::Straight { .. } => Some(Ordering::Greater),
                Self::Flush { .. } => Some(Ordering::Greater),
                Self::FullHouse { .. } => Some(Ordering::Greater),
                Self::FourOfAKind { .. } => Some(Ordering::Greater),
                Self::StraightFlush { rank: rank_b } => match rank_a {
                    Rank::Ace => match rank_b {
                        Rank::Ace => Some(Ordering::Equal),
                        _ => Some(Ordering::Less),
                    },
                    rank_a => match rank_b {
                        Rank::Ace => Some(Ordering::Equal),
                        rank_b => rank_a.partial_cmp(rank_b),
                    },
                },
            },
        }
    }
}

impl Ord for Combination {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn compare_extra(xs: &[Rank], ys: &[Rank]) -> Ordering {
    assert_eq!(xs.len(), ys.len());

    let xs = xs.iter().sorted().rev();
    let ys = ys.iter().sorted().rev();

    xs.zip(ys)
        .map(|(x, y)| x.cmp(y))
        .find(|ord| *ord != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
}

#[cfg(test)]
mod tests {
    use ::claim::*;

    use std::cmp::Ordering;

    use super::Combination;
    use crate::card::Rank;

    #[test]
    fn test_compare_extra() {
        use super::compare_extra;

        let xs = &[Rank::Two, Rank::Nine, Rank::King];
        let ys = &[Rank::Two, Rank::Eight, Rank::King];

        assert_eq!(compare_extra(xs, ys), Ordering::Greater);

        let xs = &[Rank::Two, Rank::Eight, Rank::King];
        let ys = &[Rank::Two, Rank::Nine, Rank::King];

        assert_eq!(compare_extra(xs, ys), Ordering::Less);

        let xs = &[Rank::Two, Rank::Nine, Rank::King];
        let ys = &[Rank::Two, Rank::Nine, Rank::King];

        assert_eq!(compare_extra(xs, ys), Ordering::Equal);
    }

    #[test]
    fn test_ordering_high_card() {
        let lhs = Combination::HighCard { rank: Rank::Two };

        assert_eq!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_lt!(lhs, Combination::HighCard { rank: Rank::Three });
        assert_lt!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Eight, Rank::Six, Rank::Three]
            }
        );
        assert_lt!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four],
            }
        );
        assert_lt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            }
        );
        assert_lt!(lhs, Combination::Straight { rank: Rank::Ace });
        assert_lt!(lhs, Combination::Flush { rank: Rank::Ace });
        assert_lt!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert_lt!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            }
        );
        assert_lt!(lhs, Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_ordering_pair() {
        let lhs = Combination::Pair {
            rank: Rank::Two,
            extra: [Rank::Eight, Rank::Six, Rank::Three],
        };

        assert_gt!(lhs, Combination::HighCard { rank: Rank::Two });
        // assert_eq!(
        //     lhs,
        //     Combination::Pair {
        //         rank: Rank::Two,
        //         extra: [Rank::Eight, Rank::Six, Rank::Three]
        //     }
        // );
        // assert_eq!(
        //     lhs,
        //     Combination::Pair {
        //         rank: Rank::Two,
        //         extra: [Rank::Six, Rank::Eight, Rank::Three]
        //     }
        // );
        // assert_lt!(
        //     lhs,
        //     Combination::Pair {
        //         rank: Rank::Two,
        //         extra: [Rank::Eight, Rank::Six, Rank::Three]
        //     }
        // );
        // assert_lt!(
        //     lhs,
        //     Combination::Pair {
        //         rank: Rank::Four,
        //         extra: [Rank::Eight, Rank::Six, Rank::Three]
        //     }
        // );
        assert_lt!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four],
            }
        );
        assert_lt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            }
        );
        assert_lt!(lhs, Combination::Straight { rank: Rank::Ace });
        assert_lt!(lhs, Combination::Flush { rank: Rank::Ace });
        assert_lt!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert_lt!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            }
        );
        assert_lt!(lhs, Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_ordering_two_pairs() {
        let lhs = Combination::TwoPairs {
            low: Rank::Two,
            high: Rank::Three,
            extra: [Rank::Four],
        };

        assert_gt!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_gt!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Eight, Rank::Six, Rank::Three]
            }
        );
        assert_eq!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_lt!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Five,
                extra: [Rank::Four]
            }
        );
        assert_lt!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Five,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_lt!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Five]
            }
        );
        assert_lt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            }
        );
        assert_lt!(lhs, Combination::Straight { rank: Rank::Ace });
        assert_lt!(lhs, Combination::Flush { rank: Rank::Ace });
        assert_lt!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert_lt!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            }
        );
        assert_lt!(lhs, Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_ordering_three_of_a_king() {
        let lhs = Combination::ThreeOfAKind {
            rank: Rank::Two,
            extra: [Rank::Three, Rank::Four],
        };

        assert_gt!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_gt!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Eight, Rank::Six, Rank::Three]
            }
        );
        assert_gt!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );

        assert_eq!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            }
        );
        assert_lt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Four,
                extra: [Rank::Three, Rank::Four],
            }
        );
        assert_lt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Five, Rank::Four],
            }
        );
        assert_lt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Six],
            }
        );
        assert_lt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Five, Rank::Six],
            }
        );

        assert_lt!(lhs, Combination::Straight { rank: Rank::Ace });
        assert_lt!(lhs, Combination::Flush { rank: Rank::Ace });
        assert_lt!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert_lt!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            }
        );
        assert_lt!(lhs, Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_ordering_straight() {
        let lhs = Combination::Straight { rank: Rank::Ace };

        assert_gt!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_gt!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Eight, Rank::Six, Rank::Three]
            }
        );
        assert_gt!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_gt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            }
        );
        assert_eq!(lhs, Combination::Straight { rank: Rank::Ace });
        assert_lt!(lhs, Combination::Straight { rank: Rank::Two });
        assert_lt!(lhs, Combination::Flush { rank: Rank::Ace });
        assert_lt!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert_lt!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            }
        );
        assert_lt!(lhs, Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_ordering_flush() {
        let lhs = Combination::Flush { rank: Rank::Ace };

        assert_gt!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_gt!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Eight, Rank::Six, Rank::Three]
            }
        );
        assert_gt!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_gt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            }
        );
        assert_gt!(lhs, Combination::Straight { rank: Rank::Ace });
        assert_eq!(lhs, Combination::Flush { rank: Rank::Ace });
        assert_gt!(lhs, Combination::Flush { rank: Rank::Two });
        assert_lt!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert_lt!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            }
        );
        assert_lt!(lhs, Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_ordering_full_house() {
        let lhs = Combination::FullHouse {
            two: Rank::Two,
            three: Rank::Three,
        };

        assert_gt!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_gt!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Eight, Rank::Six, Rank::Three]
            }
        );
        assert_gt!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_gt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            }
        );
        assert_gt!(lhs, Combination::Straight { rank: Rank::Ace });
        assert_gt!(lhs, Combination::Flush { rank: Rank::Ace });
        assert_eq!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert_lt!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Four,
            }
        );
        assert_lt!(
            lhs,
            Combination::FullHouse {
                two: Rank::Four,
                three: Rank::Three,
            }
        );
        assert_lt!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            }
        );
        assert_lt!(lhs, Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_ordering_four_of_a_kind() {
        let lhs = Combination::FourOfAKind {
            rank: Rank::Two,
            extra: [Rank::Three],
        };

        assert_gt!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_gt!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Eight, Rank::Six, Rank::Three]
            }
        );
        assert_gt!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_gt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            }
        );
        assert_gt!(lhs, Combination::Straight { rank: Rank::Ace });
        assert_gt!(lhs, Combination::Flush { rank: Rank::Ace });
        assert_gt!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert_eq!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            }
        );
        assert_lt!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Four,
                extra: [Rank::Three],
            }
        );
        assert_lt!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Four],
            }
        );
        assert_lt!(lhs, Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_ordering_straight_flush() {
        let lhs = Combination::StraightFlush { rank: Rank::Ace };

        assert_gt!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_gt!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Eight, Rank::Six, Rank::Three]
            }
        );
        assert_gt!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_gt!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            }
        );
        assert_gt!(lhs, Combination::Straight { rank: Rank::Ace });
        assert_gt!(lhs, Combination::Flush { rank: Rank::Ace });
        assert_gt!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert_gt!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            }
        );
        assert_eq!(lhs, Combination::StraightFlush { rank: Rank::Ace });
        assert_lt!(lhs, Combination::StraightFlush { rank: Rank::Two });
    }
}

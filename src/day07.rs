use std::collections::HashMap;

pub fn solution(input: String) {
    let mut hands: Vec<([u8; 5], u64)> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (
                hand.trim()
                    .chars()
                    .map(|c| match c {
                        c if c.is_ascii_digit() => c.to_digit(10).unwrap() as u8,
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                bid.trim().parse().unwrap(),
            )
        })
        .collect();
    hands.sort_by_cached_key(|&(hand, _)| {
        let cards = hand
            .into_iter()
            .fold(HashMap::<u8, u64>::new(), |mut counts, card| {
                *counts.entry(card).or_default() += 1;
                counts
            });
        let hand_type = if cards.values().any(|&v| v == 5) {
            7
        } else if cards.values().any(|&v| v == 4) {
            6
        } else if cards.values().any(|&v| v == 3) && cards.values().any(|&v| v == 2) {
            5
        } else if cards.values().any(|&v| v == 3) {
            4
        } else if cards.values().filter(|&v| *v == 2).count() == 2 {
            3
        } else if cards.values().any(|&v| v == 2) {
            2
        } else {
            1
        };
        (hand_type, hand)
    });
    let part1: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum();
    println!("Part 1: {}", part1);
    for (hand, _) in &mut hands {
        for card in hand {
            if *card == 11 {
                *card = 1;
            }
        }
    }
    hands.sort_by_cached_key(|&(hand, _)| {
        let mut cards = hand
            .into_iter()
            .fold(HashMap::<u8, u64>::new(), |mut counts, card| {
                *counts.entry(card).or_default() += 1;
                counts
            });
        let jokers = cards.remove(&1).unwrap_or_default();
        let hand_type = if cards.values().any(|&v| v + jokers == 5) || jokers == 5 {
            7
        } else if cards.values().any(|&v| v + jokers == 4) {
            6
        } else if (cards.iter().any(|(&first, &v)| {
            v + jokers == 3 && cards.iter().any(|(&second, &v)| first != second && v == 2)
        })) || (cards.values().any(|&v| v == 3)
            && cards.values().any(|&v| v + jokers == 2))
        {
            5
        } else if cards.values().any(|&v| v + jokers == 3) {
            4
        } else if cards.values().filter(|&v| *v == 2).count() == 2
            || (jokers >= 1 && cards.values().any(|&v| v == 2))
        {
            3
        } else if cards.values().any(|&v| v + jokers == 2) {
            2
        } else {
            1
        };
        (hand_type, hand)
    });
    let part2: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum();
    println!("Part 2: {}", part2);
}

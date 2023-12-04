use std::collections::{HashMap, HashSet};

use color_eyre::Result;

pub fn solution(input: String) -> Result<()> {
    let cards: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (card, numbers) = line.split_once(':').unwrap();
            let card_no = card
                .strip_prefix("Card ")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let (winning_numbers, our_numbers) = numbers.split_once('|').unwrap();
            let winning_numbers = winning_numbers
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let our_numbers = our_numbers
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            (card_no, winning_numbers.intersection(&our_numbers).count())
        })
        .collect();

    let part1: u32 = cards
        .values()
        .map(|&shared_numbers| {
            if shared_numbers == 0 {
                0
            } else {
                1 << (shared_numbers - 1)
            }
        })
        .sum();
    println!("Part 1: {}", part1);

    let mut copies: HashMap<_, _> = cards.iter().map(|(&k, _)| (k, 1)).collect();
    for i in 1..=cards.len() {
        let card = *cards.get(&i).unwrap();
        for j in 1..=card {
            *copies.get_mut(&(i + j)).unwrap() += copies[&i];
        }
    }
    println!("Part 2: {}", copies.values().sum::<u32>());

    Ok(())
}

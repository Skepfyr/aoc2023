pub fn solution(input: String) {
    let (part1, part2) = input
        .lines()
        .map(|line| {
            let readings: Vec<_> = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let mut differences = vec![readings];
            while !differences.last().unwrap().iter().all(|&d| d == 0) {
                differences.push(
                    differences
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect(),
                );
            }
            let part1 = differences
                .iter()
                .rev()
                .fold(0, |acc, diff| diff.last().unwrap() + acc);
            let part2 = differences.iter().rev().fold(0, |acc, diff| diff[0] - acc);
            (part1, part2)
        })
        .fold((0, 0), |(acc1, acc2), (part1, part2)| {
            (acc1 + part1, acc2 + part2)
        });
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

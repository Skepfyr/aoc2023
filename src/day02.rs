use color_eyre::Result;

pub fn solution(input: String) -> Result<()> {
    let games = input
        .lines()
        .map(|line| {
            let (game, results) = line.split_once(':').unwrap();
            let game = game.strip_prefix("Game ").unwrap().parse::<u32>().unwrap();
            let results = results
                .split(';')
                .map(|results| {
                    results.split(',').fold((0, 0, 0), |(r, g, b), result| {
                        let (amount, colour) = result.trim().split_once(' ').unwrap();
                        let amount = amount.parse::<u32>().unwrap();
                        match colour {
                            "red" if r == 0 => (amount, g, b),
                            "green" if g == 0 => (r, amount, b),
                            "blue" if b == 0 => (r, g, amount),
                            _ => unreachable!(),
                        }
                    })
                })
                .collect::<Vec<_>>();
            (game, results)
        })
        .collect::<Vec<_>>();
    let part1: u32 = games
        .iter()
        .filter(|(_, results)| {
            results
                .iter()
                .all(|&(r, g, b)| r <= 12 && g <= 13 && b <= 14)
        })
        .map(|(game, _)| game)
        .sum();
    println!("Part 1: {}", part1);
    let part2: u32 = games
        .iter()
        .map(|(_, results)| {
            results
                .iter()
                .fold((0, 0, 0), |(max_r, max_g, max_b), &(r, g, b)| {
                    (u32::max(max_r, r), u32::max(max_g, g), u32::max(max_b, b))
                })
        })
        .map(|(r, g, b)| r * g * b)
        .sum();
    println!("Part 2: {}", part2);
    Ok(())
}

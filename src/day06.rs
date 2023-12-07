pub fn solution(input: String) {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap());
    let records = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap());
    let races = times.zip(records).collect::<Vec<_>>();
    let part1: u32 = races
        .iter()
        .map(|&(t, r)| (1..t).map(|b| b * (t - b)).filter(|&d| d > r).count() as u32)
        .product();
    println!("Part 1: {}", part1);

    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .replace(' ', "")
        .split_once(':')
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    let record = lines
        .next()
        .unwrap()
        .replace(' ', "")
        .split_once(':')
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    let part2 = (1..time)
        .map(|b| b * (time - b))
        .filter(|&d| d > record)
        .count();
    println!("Part 2: {}", part2);
}

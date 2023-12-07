const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn solution(input: String) {
    let (part1, part2): (u32, u32) = input
        .lines()
        .map(|line| {
            let first_num_pos = line.find(|c: char| c.is_ascii_digit()).unwrap();
            let first_num = line[first_num_pos..=first_num_pos].parse::<u32>().unwrap();
            let (_, first) = NUMBERS
                .iter()
                .enumerate()
                .filter_map(|(digit, string)| line.find(string).map(|pos| (pos, digit as u32)))
                .chain(std::iter::once((first_num_pos, first_num)))
                .min_by_key(|&(pos, _)| pos)
                .unwrap();
            let last_num_pos = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
            let last_num = line[last_num_pos..=last_num_pos].parse::<u32>().unwrap();
            let (_, last) = NUMBERS
                .iter()
                .enumerate()
                .filter_map(|(digit, string)| line.rfind(string).map(|pos| (pos, digit as u32)))
                .chain(std::iter::once((last_num_pos, last_num)))
                .max_by_key(|&(pos, _)| pos)
                .unwrap();
            (10 * first_num + last_num, 10 * first + last)
        })
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

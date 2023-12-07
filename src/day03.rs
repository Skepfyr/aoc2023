use std::collections::HashMap;

pub fn solution(input: String) {
    let map = input
        .lines()
        .map(|line| (line, line.bytes().collect::<Vec<_>>()))
        .collect::<Vec<_>>();
    let mut part1 = 0;
    let mut gears = HashMap::<_, Vec<_>>::new();
    for (i, (mut line, chars)) in map.iter().enumerate() {
        let mut offset = 0;
        let mut chars = chars.as_slice();
        while let Some(start) = chars.iter().position(|c| c.is_ascii_digit()) {
            let end = chars[start..]
                .iter()
                .position(|c| !c.is_ascii_digit())
                .map(|offset| start + offset)
                .unwrap_or(chars.len());
            let num = line[start..end].parse::<u32>().unwrap();

            let mut neighbouring_symbol = false;
            for (j, chars) in
                (i.saturating_sub(1)..=i + 1).flat_map(|j| map.get(j).map(|(_, chars)| (j, chars)))
            {
                #[allow(clippy::needless_range_loop)]
                for k in
                    (offset + start).saturating_sub(1)..usize::min(offset + end + 1, chars.len())
                {
                    if is_symbol(chars[k]) {
                        neighbouring_symbol = true;
                    }
                    if chars[k] == b'*' {
                        gears.entry((j, k)).or_default().push(num);
                    }
                }
            }

            if neighbouring_symbol {
                part1 += num;
            }

            line = &line[end..];
            chars = &chars[end..];
            offset += end;
        }
    }
    println!("Part 1: {}", part1);
    let part2 = gears
        .values()
        .filter(|neighbours| neighbours.len() == 2)
        .map(|neighbours| neighbours[0] * neighbours[1])
        .sum::<u32>();
    println!("Part 2: {}", part2);
}

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}

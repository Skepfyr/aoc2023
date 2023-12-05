use std::collections::BTreeMap;

use color_eyre::Result;

pub fn solution(input: String) -> Result<()> {
    let mut lines = input.lines();
    let mut part1_locations = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut part2_locations = part1_locations
        .chunks_exact(2)
        .map(|pieces| {
            let start = pieces[0];
            let length = pieces[1];
            (start, length)
        })
        .collect::<Vec<_>>();
    assert!(lines.next().unwrap().is_empty());
    loop {
        if lines.next().is_none() {
            break;
        }
        let mut map = BTreeMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let [dest, source, len] = line
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            map.insert(source, (len, dest));
        }
        part1_locations = part1_locations
            .into_iter()
            .map(|l| {
                map.range(..=l)
                    .last()
                    .filter(|(&start, &(len, _))| l < start + len)
                    .map(|(&start, &(_, dest))| dest + (l - start))
                    .unwrap_or(l)
            })
            .collect();
        part2_locations = part2_locations
            .into_iter()
            .flat_map(|(mut start, mut len)| {
                let first = map
                    .range(..=start)
                    .last()
                    .filter(|(&source, &(len, _))| start < source + len)
                    .map(|(&source, _)| source)
                    .unwrap_or(start);
                let start = &mut start;
                let len = &mut len;
                let mut mapped = map
                    .range(first..=*start + *len)
                    .flat_map(|(&source, &(map_len, dest))| {
                        let gap_map = (*start, source.saturating_sub(*start));
                        *start += gap_map.1;
                        *len -= gap_map.1;
                        let actual_map = (
                            dest + (*start - source),
                            u64::min(*len, map_len - (*start - source)),
                        );
                        *start += actual_map.1;
                        *len -= actual_map.1;
                        [gap_map, actual_map]
                    })
                    .collect::<Vec<_>>();
                mapped.push((*start, *len));
                mapped
            })
            .filter(|&(_, len)| len > 0)
            .collect();
    }
    println!("Part 1: {}", part1_locations.iter().min().unwrap());
    println!(
        "Part 2: {}",
        part2_locations
            .into_iter()
            .filter(|&(_, len)| len > 0)
            .map(|(start, _)| start)
            .min()
            .unwrap()
    );
    Ok(())
}

use std::collections::HashMap;

pub fn solution(input: String) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().trim();
    assert!(lines.next().unwrap().is_empty());
    let nodes: HashMap<_, _> = lines
        .map(|line| {
            let (node, children) = line.split_once('=').unwrap();
            let (left, right) = children
                .trim()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(',')
                .unwrap();
            (node.trim(), (left.trim(), right.trim()))
        })
        .collect();
    let mut steps = 0;
    let mut current = "AAA";
    for dir in instructions.chars().cycle() {
        steps += 1;
        let (left, right) = nodes[current];
        let next = if dir == 'L' { left } else { right };
        current = next;
        if current == "ZZZ" {
            break;
        }
    }
    println!("Part 1: {}", steps);

    let part2 = nodes
        .keys()
        .copied()
        .filter(|node| node.ends_with('A'))
        .map(|node| {
            let mut instrs = instructions.chars().enumerate().cycle();
            let mut steps = 0u64;
            let mut current = node;
            let mut visited = HashMap::new();
            visited.insert((instructions.len() - 1, current), steps);
            let (loop_len, j) = loop {
                let (i, dir) = instrs.next().unwrap();
                steps += 1;
                let (left, right) = nodes[current];
                let next = if dir == 'L' { left } else { right };
                current = next;
                if let Some(prev_loop) = visited.insert((i, current), steps) {
                    break (steps - prev_loop, i);
                }
            };
            let loop_start = current;
            let mut offsets = Vec::new();
            loop {
                let (i, dir) = instrs.next().unwrap();
                let (left, right) = nodes[current];
                let next = if dir == 'L' { left } else { right };
                current = next;
                if current.ends_with('Z') {
                    // Hope there's only one per loop
                    offsets.push(visited.get(&(i, current)).copied().unwrap_or(steps));
                }
                if (i, current) == (j, loop_start) {
                    break;
                }
            }
            // This makes the problem easier!
            assert!(offsets.len() == 1 && offsets[0] == loop_len);
            loop_len
        })
        .fold(1, num::integer::lcm);

    println!("Part 2: {}", part2);
}

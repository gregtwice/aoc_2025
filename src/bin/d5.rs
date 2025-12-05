use std::collections::HashSet;

fn main() {
    let input = aoc::aoc_str!();

    let (ranges, values) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|l| {
            let (low, high) = l.trim().split_once("-").unwrap();

            low.parse::<u128>().unwrap()..=high.parse().unwrap()
        })
        .collect::<Vec<_>>();

    let fresh = values.lines().fold(0, |fresh, l| {
        let i_int = l.trim().parse::<u128>().unwrap();

        for range in &ranges {
            if range.contains(&i_int) {
                return fresh + 1;
            }
        }
        fresh
    });

    let mut p2_r = ranges.clone();

    let mut rs = Vec::new();
    let mut touched_ranges = HashSet::new();

    p2_r.sort_unstable_by_key(|a| *a.start());
    for (i, range) in p2_r.iter().enumerate() {
        let mut wr = range.clone();
        // dbg!(&wr);
        for other in p2_r.iter().skip(i+1 ) {
            // try to coalesce the ranges
            if !touched_ranges.contains(other) && *(other.start() )<= (*wr.end()+1) {
                wr = *wr.start()..=*other.end();
                touched_ranges.insert(other);
            }
        }
        if !touched_ranges.contains(range) {
            rs.push(wr);
        }
    }

    let c = rs.into_iter().fold(0, |acc, r| acc + r.count());
    dbg!(c);

    dbg!(fresh);
}

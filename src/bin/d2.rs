fn main() {
    let input = aoc::aoc_str!();
    let start = std::time::Instant::now();

    let ranges = input
        .lines()
        .flat_map(|l| l.trim().split(','))
        .map(|l| {
            l.split('-')
                .map(|m| m.parse::<u128>().unwrap())
                .collect::<Vec<u128>>()
        })
        .collect::<Vec<_>>();
    let mut p1: u128 = 0;
    let mut p2: u128 = 0;

    for range in &ranges {
        for num in range[0]..=range[1] {
            let candidate = num.to_string();
            let (left, right) = candidate.split_at(candidate.len() / 2);
            if left == right {
                p1 += num;
            }

            for divisor in (1..(candidate.len() / 2) + 1).filter(|d| candidate.len() % d == 0) {
                let same = candidate
                    .as_bytes()
                    .chunks(divisor)
                    .all(|c| c == &candidate.as_bytes()[0..divisor]);

                if same {
                    p2 += num;
                    break;
                }
            }
        }
    }

    let time = std::time::Instant::now() - start;
    println!("{p1}, {p2} in {:#?}", time);
}

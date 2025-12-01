fn main() {
    let input = include_str!("1.in");
    let sol_1 = input
        .trim()
        .lines()
        .fold((0, 50), |(mut zero, mut position), order| {
            let amount = order[1..].parse::<i32>().unwrap();
            match order.as_bytes()[0] {
                b'L' => {
                    position -= amount;
                    if position < 0 {
                        position += 100;
                    }
                }
                b'R' => {
                    position += amount;
                }
                _ => panic!(),
            }
            position %= 100;
            if position == 0 {
                zero += 1;
            }
            (zero, position)
        });
    dbg!(sol_1);

    let sol_2 = input
        .trim()
        .lines()
        .fold((0, 50), |(mut zero, mut position), order| {
            let mut amount = order[1..].parse::<i32>().unwrap();
            match order.as_bytes()[0] {
                b'L' => {
                    while amount != 0 {
                        position -= 1;
                        amount -= 1;
                        if position % 100 == 0 {
                            zero += 1;
                        }
                    }
                }
                b'R' => {
                    while amount != 0 {
                        position += 1;
                        amount -= 1;
                        if position % 100 == 0 {
                            zero += 1;
                        }
                    }
                }
                _ => panic!(),
            }

            (zero, position)
        });
    dbg!(sol_2);
}

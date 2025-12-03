use std::fmt::format;

fn main() {
    let input = aoc::aoc_str!();
    let order = ('1'..='9').rev().collect::<Vec<_>>();
    let result = input.lines().fold(0, |acc, line| {
        for best in order.iter() {

            let parts = line.split(*best).skip(1);

            if parts.clone().count() > 1 {
                // check bigger number at eol
                if line.chars().last().unwrap() > *best{

                // dbg!(format!("{best}{}", line.chars().last().unwrap()));
                return acc + format!("{best}{}",line.chars().last().unwrap()).parse::<i32>().unwrap();
                }
                // dbg!(format!("{best}{best}"));
                return acc + format!("{best}{best}").parse::<i32>().unwrap();
            }
            // dbg!(&parts.clone().collect::<Vec<_>>());
            if parts.clone().count() != 0 && parts.clone().any(|p| p.len() > 0)   {
                // find the biggest second number
                let max = parts
                    .filter(|p| p.len() > 0)
                    .map(|p| {
                        p.chars().max().unwrap()
                    })
                    .max()
                    .unwrap();
                // dbg!(format!("{best}{max}"));
                return acc + format!("{best}{max}").parse::<i32>().unwrap();
            }
        }
        panic!()
    });
    dbg!(result);


    let result = input.lines().fold(0, |acc, line| {
        for best in order.iter() {

            let parts = line.rsplit(*best).skip(1);

            if parts.clone().count() != 0 && parts.clone().any(|p| p.len() > 0)   {
                // find the biggest second number
                let max = parts
                    .filter(|p| p.len() > 0)
                    .map(|p| {
                        p.chars().max().unwrap()
                    })
                    .max()
                    .unwrap();
                // dbg!(format!("{best}{max}"));
                return acc + format!("{best}{max}").parse::<i32>().unwrap();
            }
        }
        panic!()
    });
}


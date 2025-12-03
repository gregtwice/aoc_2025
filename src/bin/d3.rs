use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::format;
#[derive(Debug, Eq, PartialEq,Hash)]
struct State<'s>{
    substr:&'s[u8],
    depth:usize,
}
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




    let result = input.lines().fold((0,0), |(mut p1,mut p2), line| {
        let mut state_p1 = HashMap::<State, u64>::new();
        let mut state_p2 = HashMap::<State, u64>::new();
        p1 += find_best(line.as_bytes(), 2, &mut state_p1);
        p2 += find_best(line.as_bytes(), 12, &mut state_p2);

        (p1,p2)
    });
    dbg!(result);
}


fn find_best<'a ,'s>(value: &'a[u8], depth:usize, state:&mut HashMap<State<'s>, u64>) -> u64
where 'a:'s{

    if let Entry::Occupied(e) = state.entry(State{substr:value, depth}){
        return *e.get();
    }
    if depth == 0 { return 0 }

    if value.len() == depth {
        unsafe { return str::from_utf8_unchecked(value).parse().unwrap() }
    }

    let this = (value[0] - b'0' ) as u64 * 10u64.pow(depth as u32-1) + find_best(&value[1..], depth-1,state);
    let next = find_best(&value[1..], depth, state);

    state.insert(State{substr:value, depth},u64::max(this, next));
    u64::max(this, next)
}
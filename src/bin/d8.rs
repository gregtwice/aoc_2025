use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::iter::Map;

fn main() {
    let input = aoc::aoc_str!();
    let boxes = input
        .lines()
        .map(|l| {
            let mut i = l.split(',').map(str::parse);

            (
                i.next().unwrap().unwrap(),
                i.next().unwrap().unwrap(),
                i.next().unwrap().unwrap(),
            )
        })
        .collect::<HashSet<(i64, i64, i64)>>();

    let mut distances = BTreeMap::<i64, (&(i64, i64, i64), &(i64, i64, i64))>::new();

    for a in &boxes {
        for b in &boxes {
            if a != b {
                let square = (a.0 - b.0) * (a.0 - b.0)
                    + (a.1 - b.1) * (a.1 - b.1)
                    + (a.2 - b.2) * (a.2 - b.2);

                distances.insert(square, (a, b));
            }
        }
    }

    let mut circuits: Vec<HashSet<&(i64, i64, i64)>> = Vec::new();
    let mut joined = HashSet::new();
    distances.iter().take(1000).for_each(|(_d, &(a, b))| {
        let mut ca = None;
        let mut cb = None;
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(a) {
                if ca.is_some() {
                    dbg!("L");
                }
                ca = Some(i);
            }
            if circuit.contains(b) {
                if cb.is_some() {
                    dbg!("L");
                }
                cb = Some(i);
            }
        }

        match (ca, cb) {
            (Some(ia), Some(ib)) => {
                if ia != ib {
                    // merge circuits

                    let [ca, cb] = circuits.get_disjoint_mut([ia, ib]).unwrap();
                    ca.extend(cb.iter());

                    circuits.remove(ib);
                }
            }
            (Some(ia), None) => {
                circuits[ia].insert(b);
            }
            (None, Some(ib)) => {
                circuits[ib].insert(a);
            }

            _ => {
                circuits.push(HashSet::from([a, b]));
            }
        }

        joined.insert(*a);
        joined.insert(*b);
    });

    let mut cc = circuits.iter().map(|c| c.len()).collect::<Vec<_>>();
    cc.sort_unstable();

    dbg!(&cc.iter().rev().take(5).collect::<Vec<_>>());

    let count = cc.iter().rev().take(3).product::<usize>();
    println!("{}", count);

    let mut circuits: Vec<HashSet<&(i64, i64, i64)>> = Vec::new();

    let limit = 1000;

    while let Some((_, (a, b))) = distances.pop_first() {
        let mut ca = None;
        let mut cb = None;
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(a) {
                ca = Some(i);
            }
            if circuit.contains(b) {
                cb = Some(i);
            }
        }

        match (ca, cb) {
            (Some(ia), Some(ib)) => {
                if ia != ib {
                    // merge circuits

                    let [ca, cb] = circuits.get_disjoint_mut([ia, ib]).unwrap();
                    ca.extend(cb.iter());

                    if ca.len() >= limit {
                        dbg!(&a, &b);
                        dbg!(a.0*b.0);
                        return;
                    }
                    circuits.remove(ib);
                }
            }
            (Some(ia), None) => {
                circuits[ia].insert(b);
                if circuits[ia].len() >= limit {
                    dbg!(&a, &b);
                    dbg!(a.0 * b.0);
                    return;
                }
            }
            (None, Some(ib)) => {
                circuits[ib].insert(a);
                if circuits[ib].len() >= limit {
                    dbg!(&a, &b);
                    dbg!(a.0 * b.0);

                    return;
                }
            }

            _ => {
                circuits.push(HashSet::from([a, b]));
            }
        }
    }

    let mut cc = circuits.iter().map(|c| c.len()).collect::<Vec<_>>();
    cc.sort_unstable();

    dbg!(&cc.iter().rev().take(5).collect::<Vec<_>>());

    let count = cc.iter().rev().take(3).product::<usize>();
    println!("{}", count);
}

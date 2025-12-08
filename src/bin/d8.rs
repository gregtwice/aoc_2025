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
        .collect::<HashSet<(i32, i32, i32)>>();

    let mut distances =
        BTreeMap::<ordered_float::OrderedFloat<f64>, (&(i32, i32, i32), &(i32, i32, i32))>::new();

    for a in &boxes {
        for b in &boxes {
            if a != b {
                let square = (a.0 - b.0) * (a.0 - b.0)
                    + (a.1 - b.1) * (a.1 - b.1)
                    + (a.2 - b.2) * (a.2 - b.2);

                let distance = f64::sqrt(f64::from(square));

                distances.insert(distance.into(), (a, b));
            }
        }
    }


    let mut circuits: Vec<HashSet<&(i32, i32, i32)>> = Vec::new();
    let mut joined = HashSet::new();
    distances.iter().take(1000).for_each(|(_d, &(a, b))| {
        let mut ca = None;
        let mut cb = None;
        // println!("{} {:?} {:?}", _d, a, b);

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
        //
        // println!("{} {:?} {:?}", _d, a, b);
        // println!("{:?}", circuits);
    });


    let mut cc = circuits.iter().map( | c|  c.len()).collect::<Vec<_>>();
    cc.sort_unstable();

    dbg!(&cc.iter().rev().take(5).collect::<Vec<_>>());

        let count = cc.iter().rev().take(3).product::<usize>();
    println!("{}", count );

}

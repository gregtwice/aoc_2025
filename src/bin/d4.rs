fn main() {
    let input = aoc::aoc_str!();

    let mut grid = aoc::CompleteGrid::from_grid_string(input, |c| match c {
        '.' => false,
        '@' => true,
        c @ _ => panic!("wtf '{c}'"),
    });

    let mut count = 0;
        for y in 0..grid.height as usize {
            for x in 0..grid.width as usize {
            if *grid.at(x, y) {
                if x == 0 {
                }
                let ok = grid.neighbours(x, y).iter().filter(|x| **x).count() < 4;
                if ok {
                    count += 1;
                }
            }
        }
    }

    let mut cnt_rm = 0;
    loop {
        let mut removable = Vec::new();
        for y in 0..grid.height as usize {
            for x in 0..grid.width as usize {
                if *grid.at(x, y) {

                    let ok = grid.neighbours(x, y).iter().filter(|x| **x).count() < 4;
                    if ok {
                        cnt_rm +=1;
                        removable.push((x,y));
                    }
                }
            }
        }
        if removable.is_empty() {
            break;
        }
        for rm in removable{
            *grid.at_mut(rm.0, rm.1)= false;
        }

    }
    dbg!(cnt_rm);
}

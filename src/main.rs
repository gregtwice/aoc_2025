
fn main() {

    let sol_1 = include_str!("1.in").trim().lines().fold((0, 50), |(mut zero , mut position), order|{
        let amount = order[1..].parse::<i32>().unwrap();
        match order.as_bytes()[0] {
            b'L' => {
                position -= amount;
                if position < 0 {
                    position += 100;
                }
            }
            b'R' =>{
                position += amount;
            }
            _=> panic!()
        }
        position %=100;
        if position == 0 {
            zero +=1;
        }
        (zero ,position)

    });
    dbg!(sol_1);

    let sol_2 = include_str!("1.in").trim().lines().fold((0, 50), |(mut zero , mut position), order|{
        let amount = order[1..].parse::<i32>().unwrap();
        match order.as_bytes()[0] {
            b'L' => {
                position -= amount;

                if position < 0 {
                    zero += (amount/100).max(1);
                }

            }
            b'R' =>{
                position += amount;
                if position > 99 {
                    zero += (amount/100).max(1);
                }

            }
            _=> panic!()
        }
        position %=100;
        if position == 0 {
            zero +=1;
        }
        (zero ,position)

    });
    dbg!(sol_2);
}

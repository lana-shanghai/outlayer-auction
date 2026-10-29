fn auction(
    bid0: u32,
    bid1: u32,
    bid2: u32,
    bid3: u32
) -> (u32, u32) {
    let mut highest = bid0;
    let mut second = 0;
    let mut winner = 0;

    if bid1 > highest {
        second = highest;
        highest = bid1;
        winner = 1;
    } else {
        second = bid1;
    }

    if bid2 > highest {
        second = highest;
        highest = bid2;
        winner = 2;
    } else if bid2 > second {
        second = bid2;
    }

    if bid3 > highest {
        second = highest;
        highest = bid3;
        winner = 3;
    } else if bid3 > second {
        second = bid3;
    }

    (winner, second)
}

#[executable]
fn main() -> (u32, u32) {
    auction(70, 55, 91, 80)
}

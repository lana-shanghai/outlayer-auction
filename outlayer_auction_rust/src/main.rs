use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Deserialize)]
struct Input {
    bids: Vec<u32>,
}

#[derive(Serialize)]
struct Output {
    winner: usize,
    price: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input: Input = serde_json::from_str(&input)?;

    if input.bids.len() < 2 {
        return Err("need at least two bids".into());
    }

    let mut highest = input.bids[0];
    let mut second = 0u32;
    let mut winner = 0usize;

    for (i, &bid) in input.bids.iter().enumerate().skip(1) {
        if bid > highest {
            second = highest;
            highest = bid;
            winner = i;
        } else if bid > second {
            second = bid;
        }
    }

    let result = Output {
        winner,
        price: second,
    };

    let json = serde_json::to_string(&result)?;

    let mut stdout = io::stdout();
    writeln!(stdout, "{json}")?;
    stdout.flush()?;

    Ok(())
}

extern crate rand;

use std::io;

fn main() {
    const GRID_SIZE: u8 = 5;
    let mut grid = Vec::new();

    loop {
        for _n in 0..GRID_SIZE {
            let mut row: Vec<bool> = Vec::new();

            for _m in 0..GRID_SIZE {
                row.push(rand::random());
            }

            println!("{:?}", row);
            grid.push(row);
        }

        println!("\n'q' to quit. Any other key to continue.");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Something went wrong");

        if input.trim() == "q" {
            println!("\nAll done!");
            break;
        } else {
            continue
        }
    }
}

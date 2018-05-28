extern crate rand;

use std::io;

mod grid;

fn main() {
    let mut step = 0;
    let mut grid = grid::Grid::new();

    loop {
        step += 1;
        println!("Step {}\n", step);

        grid.evolve();
        grid.print();
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

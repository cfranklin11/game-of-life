extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    const GRID_SIZE: u8 = 5;
    let mut rng = rand::thread_rng();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut step = 0;

    for _n in 0..GRID_SIZE {
        let mut row: Vec<u8> = Vec::new();

        for _m in 0..GRID_SIZE {
            row.push(rng.gen_range(0, 2));
        }

        grid.push(row);
    }

    loop {
        step += 1;
        println!("Step {}\n", step);

        let mut new_grid: Vec<Vec<u8>> = Vec::new();

        for (row_idx, row) in grid.iter().enumerate() {
            let mut new_row: Vec<u8> = Vec::new();

            for (item_idx, item) in row.iter().enumerate() {
                let mut neighbors = Vec::new();

                for neighbor_row_idx in 0..3 {
                    match (row_idx + neighbor_row_idx).cmp(&0) {
                        Ordering::Greater => (),
                        _ => continue
                    }
                    let neighbor_row: &Vec<u8> = match grid.get(row_idx + neighbor_row_idx - 1) {
                        None => continue,
                        Some(ref mut n_row) => n_row
                    };

                    for neighbor_idx in 0..3 {
                        if neighbor_row_idx == 1 && neighbor_idx == 1 {
                            continue
                        }
                        match (item_idx + neighbor_idx).cmp(&0) {
                            Ordering::Greater => (),
                            _ => continue
                        }
                        let neighbor: &u8 = match neighbor_row.get(item_idx + neighbor_idx - 1) {
                            None => continue,
                            Some(nbr) => nbr
                        };
                        if *neighbor == 1 {
                            neighbors.push(neighbor);
                        }
                    }
                }

                match neighbors.len() {
                    2 => {
                        if *item == 1 {
                            new_row.push(1)
                        } else {
                            new_row.push(0)
                        }
                    },
                    3 => new_row.push(1),
                    _ => new_row.push(0)
                }
            }

            println!("{:?}", new_row);
            new_grid.push(new_row)
        }

        grid = new_grid;

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

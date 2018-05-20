extern crate rand;

use std::io;
use std::cmp::Ordering;

fn main() {
    const GRID_SIZE: u8 = 5;
    let mut grid: Vec<Vec<bool>> = Vec::new();

    for _n in 0..GRID_SIZE {
        let mut row: Vec<bool> = Vec::new();

        for _m in 0..GRID_SIZE {
            row.push(rand::random());
        }

        grid.push(row);
    }

    loop {
        let mut new_grid: Vec<Vec<bool>> = Vec::new();

        for (row_idx, row) in grid.iter().enumerate() {
            let mut new_row: Vec<bool> = Vec::new();

            for (item_idx, item) in row.iter().enumerate() {
                let mut neighbors = Vec::new();

                for neighbor_row_idx in 0..3 {
                    match (row_idx + neighbor_row_idx).cmp(&0) {
                        Ordering::Greater => (),
                        _ => continue
                    }
                    let neighbor_row: &Vec<bool> = match grid.get(row_idx + neighbor_row_idx - 1) {
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
                        let neighbor: &bool = match neighbor_row.get(item_idx + neighbor_idx - 1) {
                            None => continue,
                            Some(nbr) => nbr
                        };
                        if *neighbor {
                            neighbors.push(neighbor);
                        }
                    }
                }

                match neighbors.len() {
                    2 => {
                        if *item {
                            new_row.push(true)
                        } else {
                            new_row.push(false)
                        }
                    },
                    3 => new_row.push(true),
                    _ => new_row.push(false)
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

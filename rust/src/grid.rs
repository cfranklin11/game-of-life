use std::cmp::Ordering;
use rand;
use rand::Rng;

pub struct Grid {
    pub rows: Vec<Vec<u8>>
}

impl Grid {
    pub fn new() -> Grid {
        const GRID_SIZE: u8 = 5;
        let mut rng = rand::thread_rng();
        let mut grid_rows: Vec<Vec<u8>> = Vec::new();

        for _n in 0..GRID_SIZE {
            let mut row: Vec<u8> = Vec::new();

            for _m in 0..GRID_SIZE {
                row.push(rng.gen_range(0, 2));
            }

            grid_rows.push(row);
        }

        Grid { rows: grid_rows }
    }


    pub fn evolve(&mut self) {
        self.rows = self.create_new_grid();
    }

    pub fn print(&self) {
        for row in self.rows.iter() {
            println!("{:?}", row)
        }
    }

    fn get_live_neighbors(&self, row_idx: usize, item_idx: usize) -> Vec<&u8> {
        let mut neighbors = Vec::new();

        for neighbor_row_idx in 0..3 {
            match (row_idx + neighbor_row_idx).cmp(&0) {
                Ordering::Greater => (),
                _ => continue
            }
            let neighbor_row: &Vec<u8> = match self.rows.get(row_idx + neighbor_row_idx - 1) {
                None => continue,
                Some(mut n_row) => n_row
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

        neighbors
    }

    fn create_new_grid(&self) -> Vec<Vec<u8>> {
        let mut new_grid: Vec<Vec<u8>> = Vec::new();

        for (row_idx, row) in self.rows.iter().enumerate() {
            let mut new_row: Vec<u8> = Vec::new();

            for (item_idx, item) in row.iter().enumerate() {
                let neighbors = self.get_live_neighbors(row_idx, item_idx);

                match neighbors.len() {
                    2 => {
                        if *item == 1 {
                            new_row.push(1);
                        } else {
                            new_row.push(0);
                        }
                    },
                    3 => new_row.push(1),
                    _ => new_row.push(0)
                }
            }

            new_grid.push(new_row);
        }

        new_grid
    }
}

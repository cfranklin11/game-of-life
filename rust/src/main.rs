extern crate rand;

fn main() {
    const GRID_SIZE: u8 = 5;
    let mut grid = Vec::new();

    for _n in 0..GRID_SIZE {
        let mut row: Vec<bool> = Vec::new();

        for _m in 0..GRID_SIZE {
            row.push(rand::random());
        }

        println!("{:?}", row);
        grid.push(row);
    }
}

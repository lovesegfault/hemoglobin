// Here because linters go insane since most code is still unused.
#![allow(dead_code)]

#[macro_use]
extern crate ndarray;
extern crate num_traits;
extern crate rand;

use std::fmt;
use num_traits::Zero;
use rand::Rng;

// Cell is really just a boolean with better naming.
#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Alive,
    Dead,
}


impl std::ops::Add<Cell> for Cell {
    type Output = Cell;
    fn add(self, rhs: Cell) -> Cell {
        if self.is_zero() && rhs.is_zero() {
            Cell::Dead
        } else {
            Cell::Alive
        }
    }
}

impl num_traits::Zero for Cell {
    fn zero() -> Self {
        Cell::Dead
    }
    fn is_zero(&self) -> bool {
        self == &Cell::Dead
    }
}


impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Alive => write!(f, "{}", "█"),
            Cell::Dead => write!(f, "{}", "░"),
        }
    }
}

impl Cell {
    fn new() -> Cell {
        Cell::Dead
    }
    // Sets a cell to Alive
    fn birth(&mut self) {
        *self = Cell::Alive;
    }
    // Sets a cell to Dead
    fn kill(&mut self) {
        *self = Cell::Dead;
    }
    fn is_alive(&self) -> bool {
        self == &Cell::Alive
    }
    fn is_dead(&self) -> bool {
        self == &Cell::Dead
    }

}

struct World {
    height: usize,
    width: usize,
    grid: ndarray::Array2<Cell>,
}

impl World {
    fn new(height: usize, width: usize) -> World {
        World {
            height: height,
            width: width,
            grid: ndarray::Array2::zeros((width, height)),
        }
    }
    fn gen(&mut self) {
        for col in 0..self.width {
            for row in 0..self.height {
                if rand::thread_rng().gen_weighted_bool(10) {
                    self.grid[[col, row]] = Cell::Alive;
                }
            }
        }
    }
    fn clear(&mut self) {
        self.grid = ndarray::Array2::zeros((self.width, self.height))
    }

    fn neighbours(&self, cell: (usize, usize), state: Cell) -> usize {
        unimplemented!();
    }

    fn step(&mut self) {
        let mut new_grid: ndarray::Array2<Cell> = ndarray::Array2::zeros((self.width, self.height));
        for col in 0..self.width {
            for row in 0..self.height {
            	let neigh_living = self.neighbours((col, row), Cell::Alive);
                // Rule 1. Any live cell with fewer than two live neighbours
                // dies, as if caused by underpopulation.
                if self.grid[[col, row]].is_alive() && neigh_living < 2 {
                	new_grid[[col, row]].kill();
                }
                // Rule 2. Any live cell with two or three live neighbours
                // lives on to the next generation.
                else if self.grid[[col, row]].is_alive() && (neigh_living == 2 || neigh_living == 3) {
                	new_grid[[col, row]] = self.grid[[col, row]];
                }
                // Rule 3. Any live cell with more than three live neighbours
                // dies, as if by overpopulation.
                else if self.grid[[col, row]].is_alive() && neigh_living > 3 {
                    new_grid[[col, row]].kill();
                }
                // Rule 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                else if self.grid[[col, row]].is_dead() && neigh_living == 3 {
                    new_grid[[col, row]].birth();
                }
            }
        }
        self.grid = new_grid;
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let top_frame = "╔".to_owned() + &"═".repeat(self.width) + "╗";
        let mut middle_frame = "\n".to_owned();
        for row in 0..self.height {
            middle_frame += "║";
            for col in 0..self.width {
                middle_frame += &self.grid[[col, row]].to_string();
            }
            middle_frame += "║\n";
        }
        let bottom_frame = "╚".to_owned() + &"═".repeat(self.width) + "╝";

        write!(f, "{}{}{}", top_frame, middle_frame, bottom_frame)
    }
}

fn main() {
    let mut w = World::new(30, 60);
    w.gen();
    println!("{}", w);

    let slice = w.grid.slice(s![0..4, 0..4]);
    println!("w.grid.slice(s![0..4, 0..4])\n{}", slice);
}

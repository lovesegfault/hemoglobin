// Leaving this here otherwise linters go insane since most of these are unused.
#![allow(dead_code)]

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
    // Returns the Cell of a cell
    fn probe(self) -> Cell {
        self
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
    	for col in 1..self.width {
    		for row in 1..self.height {
    			if rand::thread_rng().gen_weighted_bool(10) {
    				self.grid[[col, row]] = Cell::Alive;
    			}
    		}
    	}
    }
    fn clear(&mut self) {
        self.grid = ndarray::Array2::from_elem((self.width, self.height), Cell::Dead)
    }
}

impl fmt::Display for World {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let top_frame = "╔".to_owned() + &"═".repeat(self.width-1) + "╗";
		let mut middle_frame = "\n".to_owned();
		for row in 1..self.height {
			middle_frame += "║";
			for col in 1..self.width{
				middle_frame += &self.grid[[col, row]].to_string();
			}
			middle_frame += "║\n";
		}
		let bottom_frame = "╚".to_owned() + &"═".repeat(self.width-1) + "╝";

		write!(f, "{}{}{}", top_frame, middle_frame, bottom_frame)
	}
}

fn main() {
    let mut w = World::new(30, 50);
    w.gen();
    println!("{}", w);

}

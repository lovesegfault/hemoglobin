extern crate bit_vec;
extern crate num;
extern crate rand;
extern crate rustty;

use std::collections::HashSet;

use bit_vec::BitVec;
use num::bigint::BigInt;
use rand::Rng;
use rustty::ui::Widget;
use rustty::CellAccessor;

type Cell = (usize, usize);
type CellSet = HashSet<Cell>;

pub struct World {
    height: usize,
    width: usize,
    rule: BitVec,
    grid: CellSet,
}

impl World {
    pub fn new((width, height): Cell, rule: BigInt) -> World {
        World {
            height: height,
            width: width,
            rule: BitVec::from_bytes(&rule.to_bytes_be().1),
            grid: HashSet::with_capacity(height * width),
        }
    }

    pub fn gen(&mut self) {
        self.grid.clear();
        for x in 1..self.width {
            for y in 1..self.height {
                if rand::thread_rng().gen_weighted_bool(30) {
                    self.grid.insert((x, y));
                }
            }
        }
    }

    fn get_state(&self, cell: &Cell) -> usize {

        let (x, y) = (cell.0, cell.1);
        let t = (y + self.height - 1) % self.height;
        let b = (y + 1) % self.height;
        let l = (x + self.width - 1) % self.width;
        let r = (x + 1) % self.width;

        let mut val = 0;

        val += (self.grid.contains(&(l, t)) as usize) << 0;
        val += (self.grid.contains(&(x, t)) as usize) << 1;
        val += (self.grid.contains(&(r, t)) as usize) << 2;

        val += (self.grid.contains(&(l, y)) as usize) << 3;
        val += (self.grid.contains(&(x, y)) as usize) << 4;
        val += (self.grid.contains(&(r, y)) as usize) << 5;

        val += (self.grid.contains(&(l, b)) as usize) << 6;
        val += (self.grid.contains(&(x, b)) as usize) << 7;
        val += (self.grid.contains(&(r, b)) as usize) << 8;

        val
    }

    fn decide_next_state(&self, cell: &Cell) -> bool {
        let state = self.get_state(cell);
        if state > self.rule.len() -1 {
            return false
        }
        return self.rule[state];
    }

    // This is an obviously dumb way to do this
    // TODO: Find a better way
    fn neighbors(&self, cell: &Cell) -> CellSet {
        let mut neighbors: CellSet = HashSet::with_capacity(8);
        let (x, y) = (cell.0, cell.1);

        let t = (y + self.height - 1) % self.height;
        let b = (y + 1) % self.height;
        let l = (x + self.width - 1) % self.width;
        let r = (x + 1) % self.width;

        neighbors.insert((l, t));
        neighbors.insert((x, t));
        neighbors.insert((r, t));

        neighbors.insert((l, y));
        neighbors.insert((r, y));

        neighbors.insert((l, b));
        neighbors.insert((x, b));
        neighbors.insert((r, b));

        neighbors
    }

    fn neighbor_count(&self, cell: &Cell) -> (CellSet, CellSet) {
        let mut neighbors: (CellSet, CellSet) = (HashSet::with_capacity(8), HashSet::with_capacity(8));
        for neighbor in self.neighbors(cell) {
            if self.grid.contains(&neighbor) {
                neighbors.0.insert(neighbor);
            } else {
                neighbors.1.insert(neighbor);
            }
        }
        neighbors
    }

    pub fn step(&mut self) {
        let mut new_state: CellSet = HashSet::with_capacity(self.width * self.height);

        for cell in &self.grid {
            let (living, dead) = self.neighbor_count(cell);
            for itercell in living.union(&dead) {
                if self.decide_next_state(itercell) {
                    new_state.insert(*itercell);
                }
            }
            if self.decide_next_state(cell) {
                new_state.insert(*cell);
            }
        }
        self.grid = new_state;
    }

    pub fn render(&self, canvas: &mut Widget) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut cell = canvas.get_mut(x, y).unwrap();
                if self.grid.contains(&(x, y)) {
                    cell.set_ch('\u{25AA}');
                } else {
                    cell.set_ch(' ');
                }
            }
        }
    }
}

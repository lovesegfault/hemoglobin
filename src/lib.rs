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
        for x in 0..self.width {
            for y in 0..self.height {
                if rand::thread_rng().gen_weighted_bool(30) {
                    self.grid.insert((x, y));
                }
            }
        }
    }

    fn get_state(&self, cell: &Cell) -> usize {

        let (x, y) = (cell.0, cell.1);
        let top = y.checked_sub(1) != None;
        let bot = y.checked_add(1) <= Some(self.height);
        let right = x.checked_add(1) <= Some(self.width);
        let left = x.checked_sub(1) != None;

        let mut val = 0;

        if left && top {
            val += (self.grid.contains(&(x-1, y-1)) as usize) << 0;
        }
        if top {
            val += (self.grid.contains(&(x, y-1)) as usize)  << 1;
        }
        if top && right {
            val += (self.grid.contains(&(x+1, y-1)) as usize) << 2;
        }
        if left {
            val += (self.grid.contains(&(x-1, y)) as usize) << 3;
        }
        val += (self.grid.contains(&(x, y)) as usize) << 4;
        if right {
            val += (self.grid.contains(&(x+1, y)) as usize) << 5;
        }
        if left && bot {
            val += (self.grid.contains(&(x-1, y+1)) as usize) << 6;
        }
        if bot {
            val += (self.grid.contains(&(x, y+1)) as usize) << 7;
        }
        if bot && right {
            val += (self.grid.contains(&(x+1, y+1)) as usize) << 8;
        }
        val
    }

    fn decide_next_state(&self, cell: &Cell) -> bool {
        let state = self.get_state(cell);
        if state > self.rule.len() - 1 {
            return false
        }
        return self.rule[state];
    }

    // This is an obviously dumb way to do this
    // TODO: Find a better way
    fn neighbors(&self, cell: &Cell) -> CellSet {
        let mut neighbors: CellSet = HashSet::with_capacity(8);
        let (x, y) = (cell.0, cell.1);

        let top = y.checked_sub(1) != None;
        let bot = y.checked_add(1) <= Some(self.height);
        let right = x.checked_add(1) <= Some(self.width);
        let left = x.checked_sub(1) != None;

        if right {
            neighbors.insert((x + 1, y));
        }
        if right && bot {
            neighbors.insert((x + 1, y + 1));
        }
        if right && top {
            neighbors.insert((x + 1, y - 1));
        }
        if bot {
            neighbors.insert((x, y + 1));
        }
        if top {
            neighbors.insert((x, y - 1));
        }
        if left {
            neighbors.insert((x - 1, y));
        }
        if left && bot {
            neighbors.insert((x - 1, y + 1));
        }
        if left && top {
            neighbors.insert((x - 1, y - 1));
        }
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
                    cell.set_ch('\u{2588}');
                } else {
                    cell.set_ch(' ');
                }
            }
        }
    }
}

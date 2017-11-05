extern crate bit_vec;
extern crate num;
extern crate rand;
extern crate rustty;

use std::collections::{HashSet};
use std::time::Duration;

use bit_vec::BitVec;

use num::bigint::BigInt;

use rand::Rng;

use rustty::{Terminal, Event, HasSize, CellAccessor};
use rustty::ui::{Widget, Alignable, HorizontalAlign, VerticalAlign};

type Cell = (usize, usize);
type CellSet = HashSet<Cell>;

struct World {
    height: usize,
    width: usize,
    rule: BitVec,
    grid: CellSet,
}



impl World {
    fn new((width, height): Cell, rule: BigInt) -> World {
        let (_, bytes) = rule.to_bytes_be();
        World {
            height: height,
            width: width,
            rule: BitVec::from_bytes(&bytes),
            grid: HashSet::with_capacity(height * width),
        }
    }
    fn gen(&mut self) {
        self.grid.clear();
        for x in 1..self.width {
            for y in 1..self.height {
                if rand::thread_rng().gen_weighted_bool(30) {
                    self.grid.insert((x, y));
                }
            }
        }
    }

    fn get_present_state(&self, cell: &Cell) -> usize {

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

    fn make_dead_or_alive(&self, cell: &Cell) -> bool {
        let state = self.get_present_state(cell);
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

    fn step(&mut self) {
        let mut new_state: CellSet = HashSet::with_capacity(self.width * self.height);

        for cell in &self.grid {
            let (living, dead) = self.neighbor_count(cell);
            for itercell in living.union(&dead) {
                if self.make_dead_or_alive(itercell) {
                    new_state.insert(*itercell);
                }
            }
            if self.make_dead_or_alive(cell) {
                new_state.insert(*cell);
            }
        }
        self.grid = new_state;
    }

    fn render(&self, canvas: &mut Widget) {
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


fn main() {
    let rule  = "98492".parse::<BigInt>().unwrap();

    //Create terminal and canvas
    let mut term = Terminal::new().unwrap();
    let mut canvas = Widget::new(term.size().0, term.size().1);
    canvas.align(&term, HorizontalAlign::Left, VerticalAlign::Top, 0);

    let (width, height) = canvas.size();
    let mut w = World::new((width, height), rule);
    w.gen();

    let mut auto = false;
    let mut delay;

    'rendering: loop {
        if auto {
            delay = 0;
        } else {
            delay = 10;
        }
        while let Some(Event::Key(c)) =
            term.get_event(Some(Duration::from_millis(delay)).unwrap())
                .unwrap()
        {
            match c {
                'q' => break 'rendering,
                'g' => w.gen(),
                'n' => w.step(),
                'a' => auto = true,
                's' => auto = false,
                _ => {}
            }
        }
        if auto {
            w.step();
        }
        w.render(&mut canvas);
        canvas.draw_into(&mut term);
        term.swap_buffers().unwrap();
    }
}

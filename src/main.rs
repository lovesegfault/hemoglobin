extern crate rand;
extern crate rustty;

use std::collections::HashSet;
use std::time::Duration;

use rand::Rng;

use rustty::{Terminal, Event, HasSize, CellAccessor};
use rustty::ui::{Widget, Alignable, HorizontalAlign, VerticalAlign};

type Board = HashSet<(usize, usize)>;

#[derive(Clone)]
struct World {
    height: usize,
    width: usize,
    grid: Board,
}

impl World {
    fn new((width, height): (usize, usize)) -> World {
        World {
            height: height,
            width: width,
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

    // This is an obviously dumb way to do this
    // TODO: Find a better way
    fn neighbors(&self, cell: &(usize, usize)) -> Board {
        let mut neighbors: Board = HashSet::with_capacity(8);
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

    // TODO: Fix dumbness
    fn neighbor_count(&self, cell: &(usize, usize)) -> (Board, Board) {
        let mut neighbors: (Board, Board) = (HashSet::with_capacity(8), HashSet::with_capacity(8));
        for neighbor in self.neighbors(cell) {
            if self.grid.contains(&neighbor) {
                neighbors.0.insert(neighbor);
            } else {
                neighbors.1.insert(neighbor);
            }
        }
        neighbors
    }
    // TODO: undumb
    fn step(&mut self) {
        let mut new_state: Board = HashSet::with_capacity(self.width * self.height);

        for cell in &self.grid {
            let (living, dead) = self.neighbor_count(cell);
            if living.len() < 2 || living.len() > 3 {
            } else if living.len() == 2 || living.len() == 3 {
                new_state.insert(*cell);
            }

            for neighbor in dead {
                if self.neighbor_count(cell).0.len() == 3 {
                    new_state.insert(neighbor);
                }
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
    //Create terminal and canvas
    let mut term = Terminal::new().unwrap();
    let mut canvas = Widget::new(term.size().0, term.size().1);
    canvas.align(&term, HorizontalAlign::Left, VerticalAlign::Top, 0);

    let (width, height) = canvas.size();
    let mut w = World::new((width, height));
    w.gen();

    let mut auto = false;

    'rendering: loop {
        while let Some(Event::Key(c)) =
            term.get_event(Some(Duration::from_millis(0)).unwrap())
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

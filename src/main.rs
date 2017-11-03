// Here because linters go insane since most code is still unused.
#![allow(dead_code)]

extern crate rand;
extern crate rustty;

use std::collections::HashSet;
use std::time::Duration;

use rand::Rng;

use rustty::{Terminal, Event, HasSize, CellAccessor};
use rustty::ui::{Widget, Alignable, HorizontalAlign, VerticalAlign};

#[derive(Clone)]
struct World {
    height: usize,
    width: usize,
    grid: HashSet<(usize, usize)>,
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
                if rand::thread_rng().gen_weighted_bool(10) {
                    self.grid.insert((x, y));
                }
            }
        }
    }

    fn step(&mut self) {
    	let mut new_state: Vec<(usize, usize)> =
    		Vec::with_capacity(self.width * self.height);
    	for living in &self.grid {
    		unimplemented!()
    	}
    }

    fn render(&self, canvas: &mut Widget) {
    	for x in 0..self.width {
    		for y in 0..self.height {
    			let mut cell = canvas.get_mut(x, y).unwrap();
    			if self.grid.contains(&(x, y)) {
                    cell.set_ch('█');
                } else {
                    cell.set_ch('░');
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

    'rendering: loop {
        while let Some(Event::Key(c)) =
            term.get_event(Duration::from_millis(50)).unwrap()
        {
            match c {
                'q' => break 'rendering,
                'g' => w.gen(),
                _ => {}
            }
        }
        w.render(&mut canvas);
        canvas.draw_into(&mut term);
        term.swap_buffers().unwrap();
    }
}

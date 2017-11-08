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

#[cfg(test)]
mod tests {
    use super::*;
    use num::PrimInt;

    #[test]
    fn test_bitvec_bit_order() {
        // Consider a two-byte number where the firt byte's value is 10 and
        // the second byte's value is 7. Converting to a little endian byte
        // array should make the 0th byte 10 and the 1th byte 7.
        let ten_seven = BigInt::from(10 + 7*(2.pow(8))); 
        let bytes = ten_seven.to_bytes_le().1;
        assert_eq!(bytes[0], 10);
        assert_eq!(bytes[1], 7);
        // Now check that we can convert this to a BitVec. The bytes are in
        // little endian order, but the bits within each byte are big endian:
        // [00001010][00000111]
        let bits = BitVec::from_bytes(&bytes);
        for i in 0..16 {
            println!("{}", bits[i]);
        }
        let expected = vec![
            // 0th byte representing 10
            false,
            false,
            false,
            false,
            true,
            false,
            true,
            false,
            // 1st byte representing 7
            false,
            false,
            false,
            false,
            false,
            true,
            true,
            true];
        for i in 0..16 {
            assert_eq!(bits[i], expected[i]);
        }
    }

    #[test]
    fn test_grid_from_gidney_string() {
        let grid = grid_from_gidney_string(vec!["   ", "   "]);
        assert_eq!(grid, CellSet::new());
        let grid = grid_from_gidney_string(vec!["#  ", "   "]);
        let mut expected = CellSet::new();
        expected.insert((0, 0));
        assert_eq!(grid, expected);
    }
}

pub fn conway_kode() -> BigInt {
    let mut kode = BigInt::from(0);
    for state in 0..512 {
        let mut bit_count = 0usize;
        let current_state = (state >> 4) % 2;
        for bit_offset in [0, 1, 2, 3, 5, 6, 7, 8].iter() {
            bit_count += (state >> bit_offset) & 1usize;
        }
        let result = BigInt::from(match bit_count {
            2 => current_state,
            3 => 1,
            _ => 0
        });
    kode = kode + (result << state);
    }
    kode
}

pub fn grid_from_gidney_string(s: Vec<&str>) -> CellSet {
    let mut result = CellSet::new();
    for (y, row) in s.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                result.insert((x, y));
            }
        }
    }
    result
}

fn rule_int_to_bitvec(x: BigInt) -> BitVec {
    let padded_x = x + (BigInt::from(1) << 512);
    BitVec::from_bytes(&padded_x.to_bytes_le().1)
}

pub struct World {
    height: usize,
    width: usize,
    rule: BitVec,
    pub grid: CellSet,
}

impl World {
    pub fn new((width, height): Cell, rule: BigInt) -> World {
        World {
            height: height,
            width: width,
            rule: rule_int_to_bitvec(rule),
            grid: HashSet::with_capacity(height * width),
        }
    }

    pub fn set_grid(&mut self, grid: CellSet) {
        self.grid = grid;
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

    fn get_cell(&self, x: Option<usize>, y: Option<usize>) -> bool {
        match x {
            None => false,
            Some(xx) => match y {
                None => false,
                Some(yy) => self.grid.contains(&(xx, yy))
            }
        }
    }

    fn get_state(&self, cell: &Cell) -> usize {

        let (x, y) = (cell.0, cell.1);
        let mut val = 0;
        for dx in 0..3 {
            for dy in 0..3 {
                if self.get_cell((x+dx).checked_sub(1), (y+dy).checked_sub(1)) {
                    val += 1 << (dx + (3*dy));
                }
            }
        }
        val
    }

    fn decide_next_state(&self, cell: &Cell) -> bool {
        let state = self.get_state(cell);
        return self.rule[state];
    }

    pub fn step(&mut self) {
        let mut new_state: CellSet = HashSet::with_capacity(self.width * self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = (x, y);
                if self.decide_next_state(&cell) {
                    new_state.insert(cell);
                }
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

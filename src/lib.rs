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

struct Grid {
    grid: CellSet,
    bounds: Option<(usize, usize)>,
}

impl Grid {
    fn new(b: Option<(usize, usize)>) -> Self {
        match b {
            None => Grid {
                grid: CellSet::new(),
                bounds: b,
            },
            _ => Grid {
                grid: CellSet::with_capacity(b.unwrap().0 * b.unwrap().1),
                bounds: b,
            },
        }

    }
    fn insert(&mut self, cell: Cell) {
        if self.bounds != None {
            if cell.0 <= self.bounds.unwrap().0 && cell.1 <= self.bounds.unwrap().1 {
                self.grid.insert(cell);
            }
        } else {
            self.grid.insert(cell);
        }
    }
    fn contains(&self, cell: &Cell) -> bool {
        self.grid.contains(cell)
    }

    fn x_bound(&self) -> usize {
        self.bounds.unwrap().0
    }

    fn y_bound(&self) -> usize {
        self.bounds.unwrap().1
    }

    pub fn gen(&mut self) {
        if self.bounds == None {
            return;
        }
        self.grid.clear();
        for x in 0..self.bounds.unwrap().0 {
            for y in 0..self.bounds.unwrap().1 {
                if rand::thread_rng().gen_weighted_bool(10) {
                    self.insert((x, y));
                }
            }
        }
    }
}

impl From<Vec<String>> for Grid {
    fn from(s: Vec<String>) -> Self {
        /// Returns a Grid interpreted from a string representation
        ///
        /// # Arguments
        ///
        /// * `s` - Representation of the grid. Each element of the vector
        /// represents a row in the grid. Hash marks # indicate live cells.
        /// For example vec!['#  ', '   ', ' # '] represents a grid with live
        /// cells at (0, 0) and (2, 1).
        ///
        /// # Example
        ///
        /// ```
        /// use hemoglobin::grid_from_string;
        /// let grid = test_grid_from_string(vec!['#  ', '   ', ' # ']);
        /// ```
        let mut result = Grid::new(None);
        for (y, row) in s.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    result.insert((x, y));
                }
            }
        }
        result
    }
}

pub struct Rule {
    dec: BigInt,
    bin: BitVec,
}

impl From<BigInt> for Rule {
    fn from(x: BigInt) -> Self {
        Rule {
            dec: x.clone(),
            bin: {
                let result_reversed = BitVec::from_bytes(&x.to_bytes_be().1);
                let mut result = BitVec::from_elem(512, false);
                for i in 0..result_reversed.len() {
                    result.set(i, result_reversed[result_reversed.len() - i - 1]);
                }
                result
            },
        }
    }
}

impl From<String> for Rule {
    fn from(s: String) -> Self {
        Rule::from(s.parse::<BigInt>().unwrap())
    }
}

pub struct World {
    rule: Rule,
    grid: Grid,
}

impl World {
    pub fn new(width: usize, height: usize, rule: Rule) -> World {
        World {
            rule: rule,
            grid: Grid::new(Some((width, height))),
        }
    }

    fn decide_next_state(&self, cell: &Cell) -> bool {
        let state = get_state(&self, cell);
        return self.rule.bin[state];
    }

    pub fn step(&mut self) {
        let mut new_state = Grid::new(self.grid.bounds);

        for x in 0..self.grid.x_bound() {
            for y in 0..self.grid.y_bound() {
                let cell = (x, y);
                if self.decide_next_state(&cell) {
                    new_state.insert(cell);
                }
            }
        }
        self.grid = new_state;
    }

    pub fn gen(&mut self) {
        self.grid.gen()
    }

    pub fn render(&self, canvas: &mut Widget) {
        for x in 0..self.grid.x_bound() {
            for y in 0..self.grid.y_bound() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use num::PrimInt;

    const EXPECTED_1082_BITS: [bool; 16] = [
        false,
        true,
        false,
        true,
        false,
        false,
        false,
        false,
        true,
        true,
        true,
        false,
        false,
        false,
        false,
        false,
    ];
    // 1802 = 10 + 7*(2^8), so writing in little endian byte order but writing
    // the bits within each byte with MSB on the left, we have
    // [00001010][00000111].
    // As a BitVec, we want 01010000 11100000.

    #[test]
    fn test_bitvec_bit_order() {
        // Consider a two-byte number where the firt byte's value is 10 and
        // the second byte's value is 7. Converting to a little endian byte
        // array should make the 0th byte 10 and the 1th byte 7.
        let ten_seven = BigInt::from(10 + 7 * (2.pow(8)));
        let bytes = ten_seven.to_bytes_le().1;
        assert_eq!(bytes[0], 10);
        assert_eq!(bytes[1], 7);
        // Now check what happens when we convert this to a BitVec. The bytes
        // are in little endian order, but the bits within each byte are big
        // endian:
        // [00001010][00000111]
        let bits = BitVec::from_bytes(&bytes);
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
            true,
        ];
        for i in 0..16 {
            assert_eq!(bits[i], expected[i]);
        }
    }

    #[test]
    fn test_conway_code() {
        let expected = "476348294852520375132009738840824718882889556\
                        423255282629108876378472743729817205343700177\
                        683429960362194923168607044012736510546282236\
                        08960"
            .parse::<BigInt>()
            .unwrap();
        assert_eq!(expected, conway_code());
    }

    #[test]
    fn test_grid_from_string() {

        let grid = grid_from_string(vec!["   ", "   "]);
        let mut expected = CellSet::new();
        assert_eq!(grid, expected);

        let grid = grid_from_string(vec!["#  ", "   "]);
        expected.insert((0, 0));
        assert_eq!(grid, expected);

        let grid = grid_from_string(vec!["#  ", " # "]);
        expected.insert((1, 1));
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_decimal_encoded_string_to_bitvec() {
        let s = "1802";
        let bitvec = decimal_encoded_string_to_bitvec(s);
        for i in 0..16 {
            assert_eq!(bitvec[i], EXPECTED_1082_BITS[i]);
        }
    }

    #[test]
    fn test_bigint_to_bitvec() {
        let v = bigint_to_bitvec(BigInt::from(1802));
        for i in 0..16 {
            assert_eq!(v[i], EXPECTED_1082_BITS[i]);
        }
    }

    #[test]
    fn test_get_state() {
        let mut grid = CellSet::new();
        //  0
        // 0#< look here
        //  ^
        grid.insert((0, 0));
        assert_eq!(get_state(&grid, &(0, 0)), 16); // 2^4
        //  01
        // 0#-< look here
        // 1-#
        //  ^
        grid.insert((1, 1));
        assert_eq!(get_state(&grid, &(0, 0)), 272); // 2^4 + 2^8
    }
}

pub fn conway_code() -> BigInt {
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
            _ => 0,
        });
        kode = kode + (result << state);
    }
    kode
}


fn get_state(world: &World, cell: &Cell) -> usize {
    let (x, y) = (cell.0, cell.1);
    let mut val = 0;
    // We now build up an integer representation of the state centered at cell.
    // We iterate over neighboring cells: dx and dy go over [0, 1, 2] where
    //   0 means "minus one", so "left" for x or "up" for y
    //   1 means same row (for x) or column (for y).
    //   2 means "plus one", so "right" for x or "down" for y.
    // Therefore, for a given dx and dy, the coordinates of the neighbor are
    // (x+dx-1, y+dy-1). However, if we're at an edge, these coordinates might
    // take us off the grid. This shows up as a failure to do the subtraction
    // because getting a negative number means we're off the grid. We check
    // for this failure with checked_sub and return false if the call returns
    // None.
    //
    // TODO: replace "integer representation" with canonical name once we pick
    // one.
    for dx in 0..3 {
        for dy in 0..3 {
            if match (x + dx).checked_sub(1) {
                None => false,
                Some(xx) => {
                    match (y + dy).checked_sub(1) {
                        None => false,
                        Some(yy) => world.grid.contains(&(xx, yy)),
                    }
                }
            }
            {
                val += 1 << (dx + (3 * dy));
            }
        }
    }
    val
}

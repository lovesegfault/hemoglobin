extern crate bit_vec;
extern crate num;
extern crate rand;
extern crate termion;


// use termion::event::{Key, Event};
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;
// use std::io::{Write, stdout, stdin};

mod grid;
mod rule;
mod world;

#[cfg(test)]
mod tests {
    use num::PrimInt;
    use num::bigint::BigUint;
    use bit_vec::BitVec;

    // 1802 = 10 + 7*(2^8), so writing in little endian byte order but writing
    // the bits within each byte with MSB on the left, we have
    // [00001010][00000111].
    // As a BitVec, we want 01010000 11100000.
    #[test]
    fn test_bitvec_order() {
        // Consider a two-byte number where the firt byte's value is 10 and
        // the second byte's value is 7. Converting to a little endian byte
        // array should make the 0th byte 10 and the 1th byte 7.
        let ten_seven = BigUint::from(10 + 7 * (2.pow(8)) as u32);
        let bytes = ten_seven.to_bytes_le();
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

    fn gen_conway_dec() -> BigUint {
        let mut kode = BigUint::from(0u32);
        for state in 0..512 {
            let mut bit_count = 0usize;
            let current_state = (state >> 4) % 2;
            for bit_offset in [0, 1, 2, 3, 5, 6, 7, 8].iter() {
                bit_count += (state >> bit_offset) & 1usize;
            }
            let result = BigUint::from(match bit_count {
                2 => current_state,
                3 => 1,
                _ => 0,
            });
            kode = kode + (result << state);
        }
        kode
    }

    #[test]
    fn test_gen_conway_dec() {
        let expected = "476348294852520375132009738840824718882889556\
                        423255282629108876378472743729817205343700177\
                        683429960362194923168607044012736510546282236\
                        08960"
            .parse::<BigUint>()
            .unwrap();
        assert_eq!(expected, gen_conway_dec());
    }
}


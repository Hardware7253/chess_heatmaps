// Convert a char of a number to an integer
// E.g. '1' -> 1
// Offset offsets the ascii value
pub fn char_to_num(c: char, offset: i8) -> Result<i8, ()> {
    let num = c as i8 - {48 + offset};
    if num < 0 || num > 9 {
        return Err(())
    }
    Ok(num)
}

// Chess coordinate notation to bitboard bit
pub fn ccn_to_bit(ccn: &str) -> Result<usize, ()> {
    let ccn_vec: Vec<char> = ccn.chars().collect();
    if ccn_vec.len() < 2 {
        return Err(());
    }

    let x = char_to_num(ccn_vec[0], 48);
    let y = char_to_num(ccn_vec[1], 0);

    let x = match x {
        Ok(num) => num,
        Err(()) => return Err(())
    };

    let y = match y {
        Ok(num) => num,
        Err(()) => return Err(())
    };

    if x > 8 || y > 8 {
        return Err(())
    }

    let i = {{y - 8}.abs() * 8} + x - 1;

    Ok(i.try_into().unwrap())
}

pub fn piece_index_from_char(character: char) -> Result<usize, ()> {
    // Chars correspond to piece index on bitboard array
    let char_ids = ['P', 'R', 'N', 'B', 'Q', 'K', 'p', 'r', 'n', 'b', 'q', 'k'];
    for i in 0..char_ids.len() {
        if character == char_ids[i] {
            return Ok(i);
        }
    }
    Err(())
}

// Return true if given char is uppercase
pub fn uppercase(character: char) -> bool {
    let ascii = character as u8;
    if ascii > 64 && ascii < 91 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_to_num_test() {
        assert_eq!(char_to_num('4', 0), Ok(4));
    }

    #[test]
    fn ccn_to_bit_test() {
        assert_eq!(ccn_to_bit("a1"), Ok(56));
        assert_eq!(ccn_to_bit("a9"), Err(()));
    }

    #[test]
    fn piece_index_from_char_test() {
        assert_eq!(piece_index_from_char('R'), Ok(1));
    }

    #[test]
    fn uppercase_test() {
        assert_eq!(uppercase('A'), true);
        assert_eq!(uppercase('a'), false);
        assert_eq!(uppercase('Z'), true);
        assert_eq!(uppercase('}'), false);
    }
}
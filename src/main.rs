use std::io;
use std::fs;

fn main() {
    // Get opening type
    println!("Chess opening heatmap maker");
    println!("");

    println!("Pick Opening Type: ");
    println!("a) Flank Openings");
    println!("b) Semi-Open Games (other than the french defence)");
    println!("c) Open Games (and the french defence)");
    println!("d) Closed and Semi-Closed Games");
    println!("e) Indian Defences");
    println!("other) All Openings");

    // Get user input as a string
    let mut opening_type = String::new();
    io::stdin()
        .read_line(&mut opening_type)
        .expect("Failed to read input");

    println!("");

    // Take first char from the input
    let opening_type: Vec<char> = opening_type.chars().collect();
    let opening_type: char = opening_type[0];

    // Get database path
    let mut openings_path = String::from("eco/");
    openings_path.push_str(&opening_type.to_string());
    openings_path.push_str(".tsv");

    // Read openings_path into a string
    let openings = fs::read_to_string(openings_path);

    let openings = match openings {
        Ok(string) => string,
        Err(_) => { // If reading openings_path failed assume the user wants all of the openings compiled into the heatmap
            let all_openings = ["eco/a.tsv", "eco/b.tsv", "eco/c.tsv", "eco/d.tsv", "eco/e.tsv"];

            let mut openings = String::new();
            for i in 0..all_openings.len() {
                openings.push_str(&fs::read_to_string(all_openings[i]).unwrap());
                openings.push_str("\n");
            }
            openings
        }
    };

    // Get the pgn of every line, and put it into a vec
    let lines = seperate_lines(&openings);
    let mut pgn_vec = Vec::new();

    for i in 0..lines.len() {
        let line = &lines[i];
        pgn_vec.push(find_pgn(line));
    }

    // Contains an array for each type of piece
    // Numbers in these arrays represent how many times the piece type moved to a square
    // Indexes for arrays
    // White pawn = 0
    // White rook = 1
    // White knight = 2
    // White bishop = 3
    // White queen = 4
    // White king = 5

    // Black pawn = 6
    // Black rook = 7
    // Black knight = 8
    // Black bishop = 9
    // Black queen = 10
    // Black king = 11
    let mut moves_boards = [[0u16; 64]; 12];

    // Go through each pgn string, and increment the squares where pieces move
    for i in 0..pgn_vec.len() {
        let pgn = &pgn_vec[i];

        update_squares_from_pgn(&pgn, &mut moves_boards);
    }

    println!("{:?}", moves_boards);    
}

fn update_squares_from_pgn(pgn: &str, moves_boards: &mut [[u16; 64]; 12]) {

    let pgn_chars: Vec<char> = pgn.chars().collect(); // Turn string into a vector of chars
    let mut team_white = true;

    let mut move_vec = Vec::new();
    for i in 0..pgn_chars.len() {

        // Ignore 'x' in pgn, because it gets in the way when looking for sqaures that are moved to
        if pgn_chars[i] == 'x' {
            continue;
        }

        if pgn_chars[i] == ' ' || pgn_chars[i] == '.' || i == pgn_chars.len() - 1 {

            // When there is a space check if the move_vec has a move
            if move_vec.len() > 0 { // If the move vec isn't empty, and doesn't begin with a number then it is a move
                match chess_heatmaps::char_to_num(move_vec[0], 0) {
                    Ok(_) => (),
                    Err(_) => { // Code runs when move_vec contains a move

                        // Add the current char to the move_vec if this is the last character in the pgn (because there wont be a next iteration)
                        if i == pgn_chars.len() - 1 {
                            move_vec.push(pgn_chars[i]);
                        }

                        // Decode move, and increment squares
                        
                        
                        if move_vec == ['O', '-', 'O', '-', 'O'] || move_vec == ['O', '-', 'O'] { // Castle moves
                            
                            let king_index;
                            let king_bit;
                            
                            let rook_index;
                            let rook_bit;

                            // Get piece indexes for the current team
                            if team_white {
                                king_index = 5;
                                rook_index = 1;
                            } else {
                                king_index = 11;
                                rook_index = 7;
                            }

                            if move_vec == ['O', '-', 'O', '-', 'O'] { // Queen side
                                if team_white {
                                    king_bit = 58;
                                    rook_bit = 59;
                                } else {
                                    king_bit = 2;
                                    rook_bit = 3;
                                }
                            } else { // King side
                                if team_white {
                                    king_bit = 62;
                                    rook_bit = 61;
                                } else {
                                    king_bit = 6;
                                    rook_bit = 5;
                                }
                            }

                            // Increment squares move
                            increment_square(king_index, king_bit, moves_boards);
                            increment_square(rook_index, rook_bit, moves_boards);
                            

                        } else { // Regular moves

                            // Char id is move_vec[0] unless the piece is a pawn
                            // Because in pgn the piece type is usually defined at the start of the move E.g. "Nf6"
                            let mut char_id = move_vec[0];
                            if !chess_heatmaps::uppercase(char_id) {
                                char_id = 'P';
                            }

                            // Get piece board_index from the char_id and convert it to the correct team
                            let mut piece_index = chess_heatmaps::piece_index_from_char(char_id).unwrap();
                            piece_index = convert_white_to_black_id(piece_index, team_white);

                            // When a pawn moves (when not capturiung) it's move_vec will be shorter than other pieces moving
                            // Because pawns don't have thier piece id before the square they move to
                            // So an offset must be applied to get the right chars to append to the ccn_string
                            let mut offset: usize = 1;
                            if move_vec.len() < 3 && char_id == 'P' {
                                offset = 0;
                            }

                            // If the bit expected to contain a number doesn't contain a number shift the offset
                            if offset == 1 && move_vec.len() == 4 {
                                match chess_heatmaps::char_to_num(move_vec[2], 0) {
                                    Ok(_) => (),
                                    Err(_) => offset = 2,
                                };
                            }

                            // Get ccn coordinates that the piece moves to
                            let mut ccn_string = String::new();
                            ccn_string.push_str(&move_vec[0 + offset].to_string());
                            ccn_string.push_str(&move_vec[1 + offset].to_string());

                            // Get board bit the piece moves to from the ccn
                            let board_bit = chess_heatmaps::ccn_to_bit(&ccn_string).unwrap();
                            
                            // Increment squares move
                            increment_square(piece_index, board_bit, moves_boards);
                        }                
    
                        // Team is inverted after this move
                        team_white = !team_white;
                    },
                };
            }

            // Reset move_vec
            move_vec = Vec::new();
            continue;
        }

        // Add current pgn_char to move_vec
        move_vec.push(pgn_chars[i]);
    }
}

// Converts white team piece index to black team piece index of the same type
// Only changes if team_white = false
fn convert_white_to_black_id(id: usize, team_white: bool) -> usize {
    if !team_white {
        return id + 6;
    }
    id
}

fn increment_square(piece_index: usize, board_bit: usize, moves_boards: &mut [[u16; 64]; 12]) {
    moves_boards[piece_index][board_bit] += 1;
}

// Returns the pgn in a given string
// Will not work if there is a 1 in the string before the pgn, or if there is any text after the pgn
fn find_pgn(text: &str) -> String {
    let text_chars: Vec<char> = text.chars().collect(); // Turn string into a vector of chars

    let mut pgn = String::new();
    let mut found_pgn = false;
    for i in 0..text_chars.len() {

        if i != text_chars.len() - 1 {
            if text_chars[i] == '1' && text_chars[i + 1] == '.' { // Once the characters "1." are found in the string start adding character to the output string
                found_pgn = true;
            }
        }

        if found_pgn {
            pgn.push_str(&text_chars[i].to_string());
        }
    }

    pgn
}

// Seperate lines into a vec of string
// Where every line is it's own element in the vec
fn seperate_lines(text: &str) -> Vec<String>{
    let text_chars: Vec<char> = text.chars().collect(); // Turn string into a vector of chars
    let mut string_vec: Vec<String> = Vec::new(); // Output vector, where each string is a different element in the vec

    let mut line = String::new();

    for i in 0..text_chars.len() {
        let ascii = text_chars[i] as u8;
        
        // True if there is a new line
        if ascii == 10 {
            string_vec.push(line); // Push line to vec
            line = String::from(""); // Reset line
            continue;
        }

        line.push_str(&text_chars[i].to_string()); // Push current char to line

        // If this is the last charater push the line
        if i == text_chars.len() - 1 {
            string_vec.push(line.clone()); // Push line to vec
        }
    }
    string_vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pgn_test() {
        let line = String::from("A99     Dutch Defense: Classical Variation, Ilyin-Zhenevsky Variation, Modern Main Line 1. d4 f5 2. c4 Nf6 3. g3 e6 4. Bg2 Be7 5. Nf3 O-O 6. O-O d6 7. Nc3 Qe8 8. b3");
        assert_eq!(find_pgn(&line), "1. d4 f5 2. c4 Nf6 3. g3 e6 4. Bg2 Be7 5. Nf3 O-O 6. O-O d6 7. Nc3 Qe8 8. b3");
    }

    #[test]
    fn seperate_lines_test() {        
        let result = seperate_lines("Line 1\nLine 2");
        let expected = vec![String::from("Line 1"), String::from("Line 2")];

        assert_eq!(result, expected);
        
    }

    #[test]
    fn convert_white_to_black_id_test() {
        assert_eq!(convert_white_to_black_id(3, false), 9);
    }

    #[test]
    fn increment_square_test() {
        let mut moves_boards = [[0u16; 64]; 12];

        let mut expected = moves_boards;
        expected[11][63] = 1;

        increment_square(11, 63, &mut moves_boards);

        assert_eq!(moves_boards, expected);
    }

    #[test]
    fn update_squares_from_pgn_test() {
        let mut moves_boards = [[0u16; 64]; 12];

        let mut expected = moves_boards;
        expected[0][46] += 1; // g3     white
        expected[6][28] += 1; // e5     black
        expected[2][47] += 1; // Nh3    white
        expected[6][27] += 1; // d5     black
        expected[0][37] += 1; // f4     white
        expected[9][47] += 1; // Bxh3   black
        expected[3][47] += 1; // Bxh3   white
        expected[6][37] += 1; // exf4   black

        expected[5][62] += 1; // O-O    white
        expected[1][61] += 1; // O-O    white

        update_squares_from_pgn("1. g3 e5 2. Nh3 d5 3. f4 Bxh3 4. Bxh3 exf4 5. O-O", &mut moves_boards);

        assert_eq!(moves_boards, expected);
    }
    
}


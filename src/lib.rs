use std::fmt;

// Stores piece types types to check moves/representation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

impl Piece {
    // Returns a Vec of all legal moves excluding en passant and castling as coordinate tuples
    // Does not control for check/checkmate
    fn get_basic_moves(&self, x : usize, y : usize, game : &Game) -> Vec<(usize, usize)> {
        let mut moves = vec![(-1, -1)]; // Initialize moves to something that will be trimmed

        // x and y could go negative during move calculation, temporarily convert to signed integers
        let x = i32::try_from(x).unwrap();
        let y = i32::try_from(y).unwrap();
        
        match self {
            Self::King => {
                moves = vec![
                (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                (x - 1, y), (x + 1, y),
                (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)
                ]
            },
            Self::Queen => {
                moves.append(&mut self.get_moves_in_line(x, y, x, y + 7, game)); // South
                moves.append(&mut self.get_moves_in_line(x, y, x, y - 7, game)); // North
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y, game)); // East
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y, game)); // West
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y + 7, game)); // Southeast
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y - 7, game)); // Northeast
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y + 7, game)); // Southwest
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y - 7, game)); // Northwest
            },
            Self::Rook => {
                moves.append(&mut self.get_moves_in_line(x, y, x, y + 7, game)); // South
                moves.append(&mut self.get_moves_in_line(x, y, x, y - 7, game)); // North
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y, game)); // East
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y, game)); // West
            },
            Self::Bishop => {
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y + 7, game)); // Southeast
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y - 7, game)); // Northeast
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y + 7, game)); // Southwest
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y - 7, game)); // Northwest
            },
            Self::Knight => {
                moves = vec![
                    (x - 1, y - 2), (x + 1, y - 2),
                    (x - 2, y - 1), (x + 2, y - 1),
                    (x - 2, y + 1), (x + 2, y + 1),
                    (x - 1, y + 2), (x + 1, y + 2)
                ]
            },
            Self::Pawn => {
                match game.get_color_at(usize::try_from(x).unwrap(),usize::try_from(y).unwrap()).unwrap() {
                    Color::Black => {
                        if game.get_color_at(usize::try_from(x).unwrap(), usize::try_from(y + 1).unwrap()) == None {
                            moves.push((x, y + 1));
                        }
                        if game.get_color_at(usize::try_from(x + 1).unwrap(), usize::try_from(y + 1).unwrap()) == Some(Color::White) {
                            moves.push((x + 1, y + 1));
                        }
                        if x > 0 {
                            if game.get_color_at(usize::try_from(x - 1).unwrap(), usize::try_from(y + 1).unwrap()) == Some(Color::White) {
                                moves.push((x - 1, y + 1));
                            }
                        }
                        if y == 1 && game.get_color_at(usize::try_from(x).unwrap(), usize::try_from(y + 2).unwrap()) == None{
                            moves.push((x, y + 2));
                        }
                    },
                    Color::White => {
                        if game.get_color_at(usize::try_from(x).unwrap(), usize::try_from(y - 1).unwrap()) == None {
                            moves.push((x, y - 1));
                        }
                        if y > 0 {
                            if game.get_color_at(usize::try_from(x + 1).unwrap(), usize::try_from(y - 1).unwrap()) == Some(Color::Black) {
                                moves.push((x + 1, y - 1));
                            }
                            if x > 0 {
                                if game.get_color_at(usize::try_from(x - 1).unwrap(), usize::try_from(y - 1).unwrap()) == Some(Color::Black) {
                                    moves.push((x - 1, y - 1));
                                }
                            }
                        }
                        if y == 6 && game.get_color_at(usize::try_from(x).unwrap(), usize::try_from(y - 2).unwrap()) == None{
                            moves.push((x, y - 2));
                        }
                    }
                }
            }
        }
        let moves = moves.into_iter()
                        // Make sure all moves are in bounds
                         .filter(|(a, b)| ((-1 < *a && *a < 8) && (-1 < *b && *b < 8)))
                         // Convert into usize
                         .map(|(a, b)| (usize::try_from(a).unwrap(), usize::try_from(b).unwrap())) 
                         //Remove all moves colliding with own color
                         .filter(|(a, b)| game.get_color_at(*a, *b) != game.get_color_at(usize::try_from(x).unwrap(),
                                                                                                              usize::try_from(y).unwrap()))
                         .collect();
        return moves;
    }

    // Returns a list of all legal moves in a straight line from (x, y) to (target_x, target_y), stopping if interrupted by another piece
    // Checks for coordinates out of bounds, meaning target_x and target_y can safely be outside of the board
    // Diagonals _must_ have slope +/- 1
    fn get_moves_in_line (&self, mut x : i32, mut y : i32, target_x : i32, target_y : i32, game : &Game) -> Vec<(i32, i32)> {
        let step_x = (target_x - x).signum();
        let step_y = (target_y - y).signum();
        let own_color = game.get_color_at(usize::try_from(x).unwrap(), usize::try_from(y).unwrap()).unwrap();
        let mut possible_moves = vec![(-1, -1)]; // Initialize possible moves to something that will get culled

        for i in 0..8 {
            x += step_x;
            y += step_y;

            // End loop if new x or y out of bounds
            if (x > 7) || (y > 7) || (x < 0) || (y < 0) {
                break;
            }

            match game.get_color_at(usize::try_from(x).unwrap(),usize::try_from(y).unwrap()) {
                Some(color) => {
                    if color != own_color {
                        possible_moves.push((x, y));
                    }
                    break;
                },
                None => {
                    possible_moves.push((x, y));
                },
            }
        }
        return possible_moves;
    }
}

// Game states
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    InProgress, // State during normal gameplay, allow any move
    Check, // King is in check, move out of check should be forced
    GameOver, // Game has ended due to checkmate/resignation, further play disallowed
    Checkmate // Checkmate, award a win and end the game
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Color {
    Black,
    White
}

// Stores game state including:
// Game board with pieces and colors
// Player to move
pub struct Game {
    board: [[Option<Piece>; 8]; 8],
    black: u64, // Positions of black pieces as a bitmap
    white: u64, // And the same for white pieces
    state: GameState,
    player: Color, // The player to move
    promotion_piece : Piece // The piece type that pawns will promote to
}

impl Game {
    // Constructs a Game instance
    pub fn new() -> Game {
        Game {
            // Places the pieces in the starting position
            board : [
                [Some(Piece::Rook), Some(Piece::Knight), Some(Piece::Bishop), Some(Piece::Queen), Some(Piece::King), Some(Piece::Bishop), Some(Piece::Knight), Some(Piece::Rook)],
                [Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn)],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn)],
                [Some(Piece::Rook), Some(Piece::Knight), Some(Piece::Bishop), Some(Piece::Queen), Some(Piece::King), Some(Piece::Bishop), Some(Piece::Knight), Some(Piece::Rook)]
            ],
            black: 0xFF_FF_00_00_00_00_00_00, // Sets top two rows to black
            white: 0x00_00_00_00_00_00_FF_FF, // And bottom two rows to white
            state: GameState::InProgress,
            player: Color::White, // White to move
            promotion_piece: Piece::Queen
        }
    }

    // Takes standard chess coordinates as inputs
    // If the move is legal and the state allows:
    // Makes the move, sets and returns the resulting game state, and advances the turn to the other player (if not checkmated)
    // Any illegal move returns None
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        let numerical_from = string_to_coordinates(_from);
        let numerical_to = string_to_coordinates(_to);
        // First check that the move is allowed, if not exit early and return None
        // Exit if game state prohibits moving
        if !(self.state == GameState::InProgress || self.state == GameState::Check) {
            return None;
        }
        // Exit if attempting to move out of turn
        if self.get_color_at(numerical_from.0, numerical_from.1) != Some(self.player) {
            return None;
        }
        let moves = self.get_possible_moves(_from);
        // Exit if start position is invalid
        if moves == None {
            return None;
        }
        let moves = moves.unwrap();
        // Exit if end position is invalid
        if !moves.contains(&String::from(_to)) {
            return None;
        }
        // We now know that the move is legal, proceed from there
        self.move_piece(numerical_from.0, numerical_from.0, numerical_to.0, numerical_to.1);
        let opponent = get_opposite_color(self.player);
        if self.is_in_check(opponent) {
            if self.has_no_moves(opponent) {
                self.state = GameState::Checkmate;
            }
            else {
                self.state = GameState::Check
            }
        }
        else {
            self.state = GameState::InProgress;
        }
        self.player = opponent; // Turn is over, swap player
        return Some(self.state);
    }

    // Moves piece without checks, will panic at an invalid move
    // Replaces whatever is at the end position, meaning captures happen automatically
    // Promotes pawns if they reach the end of the board
    fn move_piece(&mut self, start_x : usize, start_y : usize, end_x : usize, end_y : usize) -> () {
        let piece = self.board[start_y][start_x].clone().unwrap();
        let color = self.get_color_at(start_x, start_y).unwrap();
        self.board[start_y][start_x] = None;
        self.set_color_at(start_x, start_y, None);
        self.board[end_y][end_x] = Some(piece);
        self.set_color_at(end_x, end_y, Some(color));
        // Start check for promotion
        if piece != Piece::Pawn {
            return
        }
        match color {
            Color::Black => {
                if end_y == 7 {
                    self.board[end_y][end_x] = Some(self.promotion_piece);
                }
            },
            Color::White => {
                if end_y == 0 {
                    self.board[end_y][end_x] = Some(self.promotion_piece);
                }
            }
        }
    }

    // If any moves for the given color are possible, return false
    // Else return true
    fn has_no_moves(&mut self, color : Color) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                if self.get_color_at(x, y) == Some(color) {
                    let moves = self.get_possible_moves(&coordinates_to_string(x, y)).unwrap();
                    if moves.len() > 0 {return false };
                }
            }
        }
        return true
    }

    // Simulates a move and restores the board, returns true if the move would put color in check, otherwise false 
    fn in_check_after_move(&mut self, start_x: usize, start_y : usize, end_x : usize, end_y : usize, color : Color) -> bool {
        let original_board = self.board.clone();
        let original_black = self.black;
        let original_white = self.white;
        self.move_piece(start_x, start_y, end_x, end_y);
        let in_check = self.is_in_check(color);
        self.board = original_board;
        self.black = original_black;
        self.white = original_white;
        return in_check
    }

    // Returns whether the king of specified color at specified coordinates is in check
    // Takes coordinates and color separately to be able to predict check for future position
    fn is_in_check(&self, color : Color) -> bool {
        let opposite_color = get_opposite_color(color);
        let mut all_opponent_moves : Vec<(usize, usize)> = vec![(0, 0)]; // Initialize to value that will be trimmed
        for y in 0..8 {
            for x in 0..8 {
                match self.board[y][x] {
                    Some(piece) => {
                        if self.get_color_at(x, y).unwrap() == opposite_color {
                            all_opponent_moves.append(&mut piece.get_basic_moves(x, y, self));
                        }
                    },
                    None => {},
                }
            }
        }
        all_opponent_moves.swap_remove(0); // Trim initialization value
        return all_opponent_moves.contains(&self.find_king(&color))
    }

    // Returns the coordinates of the king of the specified color
    fn find_king(&self, color: &Color) -> (usize, usize) {
        for y in 0..8 {
            for x in 0..8 {
                if self.board[y][x] == Some(Piece::King) && self.get_color_at(x, y) == Some(*color) {
                    return (x, y)
                }
            }
        }
        panic!("King not found!");
    }
    // Returns the color, if there is one, at the specified x, y coordinates
    // Empty squares or invalid coordinates return None
    fn get_color_at(&self, x: usize, y: usize) -> Option<Color> {
        let index = 8 * y + x;
        let position : u64 = 0x80_00_00_00_00_00_00_00u64 >> index;

        match position & self.black {
            0 => {},
            _ => {return Some(Color::Black)}
        }

        match position & self.white {
            0 => {},
            _ => {return Some(Color::White);}
        }
        None
    }

    fn set_color_at(&mut self, x: usize, y: usize, value : Option<Color>) -> () {
        let index = 8 * y + x;
        let position : u64 = 0x80_00_00_00_00_00_00_00u64 >> index;
        match value {
            Some(color) => {
                match color {
                    Color::Black => {
                        self.black = self.black | position; // set the bit at position to 1
                        self.white = self.white & !position; // set the bit at position to 0
                    },
                    Color::White => {
                        self.black = self.black & !position;
                        self.white = self.white | position;
                    }
                }
            },
            None => {
                self.black = self.black & !position;
                self.white = self.white & !position;
            },
        }
    }

    // Sets the piece type for promotion. Piece type is supplied as a string
    // Piece names must be supplied in english
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        let mut _piece = _piece.to_ascii_lowercase();
        self.promotion_piece = string_to_piece(&_piece);
    }

    pub fn get_game_state(&self) -> GameState{
        self.state
    }

    pub fn get_possible_moves(&mut self, _position: &str) -> Option<Vec<String>> {
        let _position = string_to_coordinates(_position);
        let x = _position.0;
        let y = _position.1;
        match self.board[y][x] {
            Some(piece) => {
                let color = self.get_color_at(x, y).unwrap();
                let moves = piece.get_basic_moves(x, y, self);
                let moves = moves
                                                .into_iter()
                                                .filter(|(to_x, to_y)| !self.in_check_after_move(x, y, *to_x, *to_y, color))
                                                .map(|(to_x, to_y)| coordinates_to_string(to_x, to_y))
                                                .collect::<Vec<String>>();
                return Some(moves);
            },
            None => {return None}
        }
    }
}

fn get_opposite_color(color : Color) -> Color {
    match color {
        Color::Black => {Color::White},
        Color::White => {Color::Black}
    }
}

fn string_to_coordinates(position: &str) -> (usize, usize) {
    let mut x = position.chars().nth(0).unwrap();
    x.make_ascii_uppercase();
    let x = usize::from((x as u8) - 65); // Turn x into an integer by casting char to u8 and removing the ASCII offset

    let mut y = position.chars().nth(1).unwrap();
    let y = usize::from(8 - (y as u8 - 48)); // Same as x, but subtract from 8 to uninvert y coordinate

    return (x, y)
}

// Returns a string representation of numeric board coordinates
fn coordinates_to_string(x: usize, y: usize) -> String {
    let x = u8::try_from(x).unwrap();
    let x = (x + 65) as char;

    let y = u8::try_from(y).unwrap();
    let y = (8 - y + 48) as char;
    return String::from(x) + &String::from(y)
}

fn string_to_piece(string: &str) -> Piece {
    match string {
        "king" => {Piece::King},
        "queen" => {Piece::Queen},
        "rook" => {Piece::Rook},
        "bishop" => {Piece::Bishop},
        "knight" => {Piece::Knight},
        "pawn" => {Piece::Pawn}
        _ => {panic!("Invalid piece name!")},
    }
}

// Creates a string representation of the current board
// Example:
// _R_|_Kn|_B_|_Q_|_K_|_B_|_Kn|_R_
// _P_|_P_|_P_|_P_|_P_|_P_|_P_|_P_
// _*_|_*_|_*_|_*_|_*_|_*_|_*_|_*_
// _*_|_*_|_*_|_*_|_*_|_*_|_*_|_*_
// _*_|_*_|_*_|_*_|_*_|_*_|_*_|_*_
// _*_|_*_|_*_|_*_|_*_|_*_|_*_|_*_
// _P_|_P_|_P_|_P_|_P_|_P_|_P_|_P_
// _R_|_Kn|_B_|_Q_|_K_|_B_|_Kn|_R_
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self.board.iter().map(|row| {
            row.iter().map(|item| {
                match item {
                    Some(piece) => {
                        match piece {
                            Piece::King => {"_K_".to_string()},
                            Piece::Queen => {"_Q_".to_string()},
                            Piece::Rook => {"_R_"}.to_string(),
                            Piece::Bishop => {"_B_".to_string()},
                            Piece::Knight => {"_Kn".to_string()},
                            Piece::Pawn => {"_P_".to_string()}
                        }
                    },
                    None => {"_*_".to_string()},
                }
            })
            .collect::<Vec<String>>()
            .join("|")
            })
        .collect::<Vec<String>>()
        .join("\n");
        
        write!(f, "{}\nBlack bits: {:0>64b}\nWhite bits: {:0>64b}", output, self.black, self.white)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        let game = Game::new();
        println!("{}", game)
    }

    #[test]
    fn set_color() {
        let mut game = Game::new();
        for y in 0..8 {
            for x in 0..8 {
                game.set_color_at(x, y, None);
                if x % 2 == 0 {
                    game.set_color_at(x, y, Some(Color::Black));
                }
                else {
                    game.set_color_at(x, y, Some(Color::White));
                }
            }
        }
        println!("{}", game)
    }

    #[test]
    fn in_check() {
        let mut game = Game::new();
        game.board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, Some(Piece::King), None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, Some(Piece::Pawn), None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        game.black = 0x00_00_10_00_00_00_00_00;
        game.white = 0x00_00_00_00_20_00_00_00;
        println!("{}", game);
        assert!(!game.is_in_check(Color::Black));
        game.move_piece(2, 4, 2, 3);
        println!("{}", game);
        assert!(game.is_in_check(Color::Black));
    }

    #[test]
    fn checkmated() {
        let mut game = Game::new();
        game.board = [
            [Some(Piece::Rook), None, None, Some(Piece::King), None, None, None, None],
            [Some(Piece::Rook), None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        game.black = 0x10_00_00_00_00_00_00_00;
        game.white = 0x80_80_00_00_00_00_00_00;
        println!("{}", game);
        assert!(game.is_in_check(Color::Black));
        assert!(game.has_no_moves(Color::Black));
        game.move_piece(0, 1, 0, 4);
        println!("{}", game);
        assert!(!game.has_no_moves(Color::Black));
    }

    #[test]
    fn check_after_move() {
        let mut game = Game::new();
        game.board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, Some(Piece::King), None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, Some(Piece::Pawn), None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        game.black = 0x00_00_10_00_00_00_00_00;
        game.white = 0x00_00_00_00_20_00_00_00;
        println!("{}", game);
        assert!(!game.is_in_check(Color::Black));
        assert!(game.in_check_after_move(2, 4, 2, 3, Color::Black));
        let moves = game.get_possible_moves("D6").unwrap();
        println!("{:?}", moves)

    }

    #[test]
    fn string_conversion() {
        let mut counter_x = 0;
        for letter in ["A", "B", "C", "D", "E", "F", "G", "H"] {
            let mut counter_y = 8;
            for digit in ["1", "2", "3", "4", "5", "6", "7", "8"] {
                counter_y -= 1;
                let coordinates = String::from(letter) + digit;
                println!("Before conversion: {}", coordinates);
                let coordinates = string_to_coordinates(&coordinates);
                println!("After conversion to number: {:?}", coordinates);
                assert!(coordinates == (counter_x, counter_y));
                let coordinates = coordinates_to_string(coordinates.0, coordinates.1);
                println!("After conversion back to string: {}", coordinates);
                assert!(coordinates == String::from(letter) + digit);
            }
            counter_x += 1;
        }
    }

    #[test]
    fn possible_moves () {
        let game = Game::new();
        for y in 0..8 {
            for x in 0..8 {
                match game.board[y][x] {
                    Some(piece) => {
                        println!("Piece {:?} at ({}, {})", piece, x, y);
                        println!("Possible moves: {:?}", piece.get_basic_moves(x, y, &game));
                        println!("");
                    },
                    None => {},
                } 
            }
        }
    }
}

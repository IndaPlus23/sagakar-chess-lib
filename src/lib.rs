use std::fmt;

// Stores piece types types to check moves/representation
#[derive(Clone, Copy, Debug)]
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
    pub fn get_basic_moves(&self, x : i32, y : i32, white : u64, black : u64) -> Vec<(i32, i32)> {
        let mut moves = vec![(-1, -1)]; // Initialize moves to something that will be trimmed
        
        match self {
            Self::King => {
                moves = vec![
                (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                (x - 1, y), (x + 1, y),
                (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)
                ]
            },
            Self::Queen => {
                moves.append(&mut self.get_moves_in_line(x, y, x, y + 7, black, white)); // South
                moves.append(&mut self.get_moves_in_line(x, y, x, y - 7, black, white)); // North
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y, black, white)); // East
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y, black, white)); // West
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y + 7, black, white)); // Southeast
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y - 7, black, white)); // Northeast
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y + 7, black, white)); // Southwest
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y - 7, black, white)); // Northwest
            },
            Self::Rook => {
                moves.append(&mut self.get_moves_in_line(x, y, x, y + 7, black, white)); // South
                moves.append(&mut self.get_moves_in_line(x, y, x, y - 7, black, white)); // North
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y, black, white)); // East
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y, black, white)); // West
            },
            Self::Bishop => {
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y + 7, black, white)); // Southeast
                moves.append(&mut self.get_moves_in_line(x, y, x + 7, y - 7, black, white)); // Northeast
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y + 7, black, white)); // Southwest
                moves.append(&mut self.get_moves_in_line(x, y, x - 7, y - 7, black, white)); // Northwest
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
                match get_color_at(x, y, black, white).unwrap() {
                    Color::Black => {
                        moves.push((x, y + 1));
                        if get_color_at(x + 1, y + 1, black, white) == Some(Color::White) {
                            moves.push((x + 1, y + 1));
                        }
                        if get_color_at(x - 1, y + 1, black, white) == Some(Color::White) {
                            moves.push((x - 1, y + 1));
                        }
                        if y == 1 {
                            moves.push((x, y + 2));
                        }
                    },
                    Color::White => {
                        moves.push((x, y - 1));
                        if get_color_at(x + 1, y - 1, black, white) == Some(Color::Black) {
                            moves.push((x + 1, y - 1));
                        }
                        if get_color_at(x - 1, y - 1, black, white) == Some(Color::Black) {
                            moves.push((x - 1, y - 1));
                        }
                        if y == 6 {
                            moves.push((x, y - 2));
                        }
                    }
                }
            }
        }
        let moves = moves.into_iter()
                        // Make sure all moves are in bounds
                         .filter(|(a, b)| ((-1 < *a && *a < 8) && (-1 < *b && *b < 8)))
                        //Remove all moves colliding with own color
                         .filter(|(a, b)| get_color_at(*a, *b, black, white) != get_color_at(x, y, black, white)) 
                         .collect();
        return moves;
    }

    // Returns a list of all legal moves in a straight line from (x, y) to (target_x, target_y), stopping if interrupted by another piece
    // Checks for coordinates out of bounds, meaning target_x and target_y can safely be outside of the board
    // Diagonals _must_ have slope +/- 1
    fn get_moves_in_line (&self, mut x : i32, mut y : i32, target_x : i32, target_y : i32, black : u64, white : u64) -> Vec<(i32, i32)> {
        let step_x = (target_x - x).signum();
        let step_y = (target_y - y).signum();
        let own_color = get_color_at(x, y, black, white).unwrap();
        let mut possible_moves = vec![(-1, -1)]; // Initialize possible moves to something that will get culled

        for i in 0..8 {
            x += step_x;
            y += step_y;

            // End loop if new x or y out of bounds
            if (x > 7) || (y > 7) || (x < 0) || (y < 0) {
                break;
            }

            match get_color_at(x, y, black, white) {
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
#[derive(Clone, Copy)]
pub enum GameState {
    InProgress, // State during normal gameplay, allow any move
    Check, // King is in check, move out of check should be forced
    GameOver, // Game has ended due to checkmate/resignation, further play disallowed
    Checkmate // Checkmate, award a win and end the game
}

#[derive(PartialEq, Eq, Debug)]
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
            player: Color::White // White to move
        }
    }

    // pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
    //     match self.state {
    //         GameState::InProgress => {
    //             Some(self.state)
    //         },
    //         _ => {
    //             None
    //         }
    //     }
    // }

    // pub fn set_promotion(&mut self, _piece: &str) -> () {

    // }

    // pub fn get_game_state(&self) -> GameState{
    //     self.state
    // }

//     pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>> {

//    }
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
        
        write!(f, "{}", output)
    }
}

fn get_color_at(x: i32, y: i32, black: u64, white: u64) -> Option<Color> {
    let index = 8 * y + x;
    let position : u64 = 0x80_00_00_00_00_00_00_00u64 >> index;

    match position & black {
        0 => {},
        _ => {return Some(Color::Black)}
    }

    match position & white {
        0 => {},
        _ => {return Some(Color::White);}
    }
    None
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
    fn possible_moves () {
        let game = Game::new();
        for y in 0..8 {
            println!("{y}");
            for x in 0..8 {
                match game.board[y][x] {
                    Some(piece) => {
                        println!("Piece {:?} at ({}, {})", piece, x, y);
                        println!("Possible moves: {:?}", piece.get_basic_moves(i32::try_from(x).unwrap(),i32::try_from(y).unwrap(), game.white, game.black));
                        println!("");
                    },
                    None => {},
                } 
            }
        }
    }
}

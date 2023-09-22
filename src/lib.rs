use std::fmt::{self};

// Stores piece types types to check moves/representation
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

// Game states
pub enum GameState {
    InProgress, // State during normal gameplay, allow any move
    Check, // King is in check, move out of check should be forced
    GameOver, // Game has ended due to checkmate/resignation, further play disallowed
    Checkmate // Checkmate, award a win and end the game
}

enum Color {
    Black,
    White
}

// Stores game state including:
// Game board with pieces and colors
// Player to move
pub struct Game {
    board: [Option<Piece>; 64],
    black: u64, // Positions of black pieces as a bitmap
    white: u64, // And the same for white pieces
    state: GameState,
    player: Color // The player to move
}

impl Game {
    // Constructs a Game instance
    pub fn new() -> Game {
        Game {
            // Places the pieces in the starting position
            board : [
                Some(Piece::Rook), Some(Piece::Knight), Some(Piece::Bishop), Some(Piece::Queen), Some(Piece::King), Some(Piece::Bishop), Some(Piece::Knight), Some(Piece::Rook),
                Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn),
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn), Some(Piece::Pawn),
                Some(Piece::Rook), Some(Piece::Knight), Some(Piece::Bishop), Some(Piece::Queen), Some(Piece::King), Some(Piece::Bishop), Some(Piece::Knight), Some(Piece::Rook)
            ],
            black: 0xFF_FF_00_00_00_00_00_00, // Sets top two rows to black
            white: 0x00_00_00_00_00_00_FF_FF, // And bottom two rows to white
            state: GameState::InProgress,
            player: Color::White // White to move
        }
    }
}

// Creates a string representation of the current board
// Example:
// | R | Kn| B | Q | K | B | Kn| R |
// | P | P | P | P | P | P | P | P |
// | * | * | * | * | * | * | * | * |
// | * | * | * | * | * | * | * | * |
// | * | * | * | * | * | * | * | * |
// | * | * | * | * | * | * | * | * |
// | P | P | P | P | P | P | P | P |
// | R | Kn| B | Q | K | B | Kn| R |
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self.board.iter().enumerate().map(|(index, item)| {
            let mut text = String::from("");
            if index == 0 {
                text += "|"
            }

            match item {
            Some(piece) => {
                match piece {
                    Piece::King => {text += " K "},
                    Piece::Queen => {text += " Q "},
                    Piece::Rook => {text += " R "},
                    Piece::Bishop => {text += " B "},
                    Piece::Knight => {text += " Kn"},
                    Piece::Pawn => {text += " P "}
                }
            },
            None => {text += " * "}
            }

            if (index + 1) % 8 == 0 {
                text += "|\n";
            }
            
            return text
            })
        .collect::<Vec<String>>()
        .join("|");
        
        write!(f, "{}", output)
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
}

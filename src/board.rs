//! A module for the game logic of Chess. Stores the Board object and contains
//! methods for finding potential moves, check/checkmate, etc.

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Represents the index of a single cell in the board.
pub struct Cell {
	row: u32,
	col: u32,
}

/// Represents the board state.
struct Board {
	color: Color,
	pub board: Vec<Vec<Option<Piece>>>,
}

#[derive(PartialEq, Eq)]
/// Represents the possible types of pieces in a given cell.
enum PieceType {
	Bishop,
	King,
	Knight,
	Pawn,
	Rook,
	Queen,
}

#[derive(PartialEq, Eq)]
/// Represents the color of a piece.
enum Color {
	Black,
	White,
}

/// Represents a single chess piece.
struct Piece {
	piece_type: PieceType,
	color: Color,
	cell: Cell,
}

impl Board {

	/// Initialize the board with starting positions.
	pub fn init() -> Board {
		Board {
			color: Color::White,
			board: Vec::new()
		}
	}

	/// Get the piece associated with a given cell index.
	fn get_piece(&self, cell: Cell) -> &Option<Piece> {
		&self.board[cell.row as usize][cell.row as usize]
	}

	/// Calculate the potential moves for a given cell index.
	pub fn potential_moves(&self, cell: Cell) -> Vec<Cell> {
		if let Some(ref piece) = self.board[cell.row as usize][cell.col as usize] {
			match piece.piece_type {
				_ => Vec::new()
			}
		} else {
			Vec::new()
		}
	}

	/// Helper function that checks for check
	fn check(&self) -> bool {
		let ref board = self.board;

		for row in board {
			//for piece_option in row {
			//	if let Some(piece) = piece_option {
			//		if (piece.color == self.color && self.contains_enemy_king(self.potential_moves(piece.cell))) {
			//			return true
			//		}
			//	}
			//}
		}
		false
	}

	/// Helper function that checks for checkmate
	fn checkmate(&self) -> bool {
		// check if king can move to get out
		// check if another piece can move in the way (which is a lot of computation)
		true
	}

	/// Helper function that moves a piece to a target cell
	fn move_piece(&mut self, mut piece: Piece, to: Cell) {
		let from = piece.cell.clone();
		self.board[from.row as usize][from.col as usize] = None;
		self.board[to.row as usize][to.col as usize] = Some(piece);
		piece.cell = to;
	}

	/// Helper function that checks if a list of potential 
	/// moves contains the king
	fn contains_enemy_king(&self, cells: Vec<Cell>) -> bool {
		for cell in cells {
			if let Some(ref piece) = self.board[cell.row as usize][cell.col as usize] {
				if (piece.piece_type == PieceType::King && piece.color == self.color) {
					return true
				}
			}
		}
		false
	}
}
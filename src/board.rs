//! A module for the game logic of Chess. Stores the Board object and contains
//! methods for finding potential moves, check/checkmate, etc.

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Represents the index of a single cell in the board.
pub struct Cell {
	row: u32,
	column: u32,
}

/// Represents all the pieces in the board.
struct Board {
	pub board: Vec<Vec<Piece>>,
}

/// Represents the possible types of pieces in a given cell.
enum Piece {
	Bishop,
	King,
	Knight,
	Pawn,
	Rook,
	Queen,
	None,
}

impl Board {

	/// Initialize the board with starting positions.
	pub fn init() {

	}

	/// Get the piece associated with a given cell index.
	fn get_piece(cell: Cell) -> Piece {
		Piece::None

	}

	/// Calculate the potential moves for a given cell index.
	pub fn potential_moves(cell: Cell) -> Vec<Cell> {
		Vec::new()
	}
}
//! A module for the game logic of Chess. Stores the Board object and contains
//! methods for finding potential moves, check/checkmate, etc.

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Represents the index of a single cell in the board.
pub struct Cell {
	row: i32,
	col: i32,
}

#[derive(Debug, Clone)]
/// Represents the board state.
pub struct Board {
	color: Color,
	pub board: Vec<Vec<Option<Piece>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents the possible types of pieces in a given cell.
enum PieceType {
	Bishop,
	King,
	Knight,
	Pawn,
	Rook,
	Queen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents the color of a piece.
enum Color {
	Black,
	White,
}

#[derive(Debug, Clone)]
/// Represents a single chess piece.
pub struct Piece {
	piece_type: PieceType,
	color: Color,
	cell: Cell,
}

impl Board {

	/// Initialize the board with starting positions.
	pub fn new() -> Board {
		Board {
			color: Color::White,
			board: Vec::new()
		}
	}

	/// Get the piece associated with a given cell index.
	fn get_piece(&self, cell: &Cell) -> &Option<Piece> {
		&self.board[cell.row as usize][cell.row as usize]
	}

	fn inbounds(&self, cell: &Cell) -> bool {
		cell.row < 8 && cell.row >= 0 && cell.col < 8 && cell.row >= 0
	}

	fn is_empty(&self, cell: &Cell) -> bool {
		!self.get_piece(cell).is_some()
	}

	fn is_enemy(&self, cell: &Cell) -> bool {
		if let Some(ref piece) = *self.get_piece(cell) {
			return piece.color != self.color
		}
		false
	}

	/// Returns the potential moves in the given directions, stopping upon a collision.
	fn moves_until_collision(&self, dirs: Vec<(i32, i32)>, mut cell: Cell) -> Vec<Cell> {
		let mut moves = Vec::new();
		for dir in &dirs {
			while self.inbounds(&cell) && self.is_empty(&cell) {
				moves.push(cell.clone());
				cell.row += dir.0;
				cell.col += dir.1;
			}
			// Case for enemy piece collision
			if self.is_enemy(&cell) {
				moves.push(cell.clone());
			}
		}
		moves
	}

	/// Helper function to implement pawn logic.
	fn pawn_moves(&self, cell: Cell) -> Vec<Cell> {
		let mut moves = Vec::new();
		if let Some(ref piece) = *self.get_piece(&cell) {
			let mut dir = 0;
			match piece.color {
				Color::Black => {
					if cell.row == 1 {
						moves.push(Cell{row: cell.row + 2, col: cell.col});
					}
					dir = 1;
				},
				Color::White => {
					if cell.row == 6 {
						moves.push(Cell{row: cell.row - 2, col: cell.col});
					}
					dir = -1;
					moves.push(Cell{row: cell.row - 1, col: cell.col});
				}
			}
			let vertical = Cell{row: cell.row + dir, col: cell.col};
			let diag_right = Cell{row: cell.row + dir, col: cell.col + 1};
			let diag_left = Cell{row: cell.row + dir, col: cell.col - 1};
			if self.is_empty(&vertical) {
				moves.push(vertical);
			}
			if self.is_enemy(&diag_right) {
				moves.push(diag_right);
			}
			if self.is_enemy(&diag_left) {
				moves.push(diag_left);
			}
		}
		moves
	}

	/// Helper function to implement logic for pieces with set directions.
	fn basic_moves(&self, dirs: Vec<(i32, i32)>, mut cell: Cell) -> Vec<Cell> {
		let mut moves = Vec::new();
		for dir in dirs {
			let mut new_cell = cell.clone();
			new_cell.row = new_cell.row + dir.0;
			new_cell.col = new_cell.col + dir.1;
			if self.is_empty(&new_cell) || self.is_enemy(&new_cell) {
				moves.push(new_cell);
			}
		}
		moves
	}

	/// Helper function to check if a move would place the current player's king in danger.
	fn self_check(&self, piece: &Piece, to: Cell) -> bool {
		let mut new_board = self.clone();
		new_board.move_piece(piece.clone(), to);
		new_board.switch_color();
		new_board.check()
	}

	/// Helper function to swap the current player.
	fn switch_color(&mut self) {
		match self.color {
			Color::Black => self.color = Color::White,
			Color::White => self.color = Color::Black
		}
	}

	/// Calculate the potential moves for a given cell index.
	pub fn potential_moves(&self, cell: Cell) -> Vec<Cell> {
		let mut moves: Vec<Cell> = Vec::new();
		if let Some(ref piece) = self.board[cell.row as usize][cell.col as usize] {
			match piece.piece_type {
				PieceType::Queen => {
					let dirs = vec![(0, 1), (1, 0), (-1, 0), (0, -1),
									(1, 1), (1, -1), (-1, 1), (-1, -1)];
					moves.append(&mut self.moves_until_collision(dirs, cell.clone()));					
				},
				PieceType::Bishop => {
					let dirs = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
					moves.append(&mut self.moves_until_collision(dirs, cell.clone()));	
				},
				PieceType::Rook => {
					let dirs = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
					moves.append(&mut self.moves_until_collision(dirs, cell.clone()));	
				},
				PieceType::Pawn => {
					moves.append(&mut self.pawn_moves(cell.clone()));
				},
				PieceType::King => {
					let dirs = vec![(0, 1), (1, 0), (-1, 0), (0, -1),
									(1, 1), (1, -1), (-1, 1), (-1, -1)];
					moves.append(&mut self.basic_moves(dirs, cell.clone()));
				},
				PieceType::Knight => {
					let dirs = vec![(2, 1), (1, -2), (-1, 2), (-2, -1),
									(1, 2), (-2, 1), (2, -1), (-1, -2)];
					moves.append(&mut self.basic_moves(dirs, cell.clone()));

				}
			}
			moves.retain(|m| !self.self_check(piece, m.clone()));
		}
		moves
	}


	/// Helper function that checks for check
	fn check(&self) -> bool {

		//for row in &self.board {
		//	for piece_option in row {
		//		if let Some(ref piece) = piece_option {
		//			if piece.color == self.color && self.contains_enemy_king(self.potential_moves(piece.cell)) {
		//				return true
		//			}
		//		}
		//	}
		//}
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
				if piece.piece_type == PieceType::King && piece.color == self.color {
					return true
				}
			}
		}
		false
	}
}
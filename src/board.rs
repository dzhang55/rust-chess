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

	/// Helper function to check if a proposed cell is in the bounds of the board.
	fn inbounds(&self, cell: &Cell) -> bool {
		cell.row < 8 && cell.row >= 0 && cell.col < 8 && cell.row >= 0
	}

	/// Helper function to check if a cell has a piece.
	fn is_empty(&self, cell: &Cell) -> bool {
		!self.get_piece(cell).is_some()
	}

	/// Helper function to check if a cell holds an enemy piece.
	fn is_enemy(&self, from: &Cell, to: &Cell) -> bool {
		if let &Some(ref from_piece) = self.get_piece(from) {
			if let &Some(ref to_piece) = self.get_piece(to) {
				return to_piece.color != from_piece.color
			}
		}
		false
	}

	/// Returns the potential moves in the given directions, stopping upon a collision.
	fn moves_until_collision(&self, dirs: Vec<(i32, i32)>, cell: Cell) -> Vec<Cell> {
		let mut moves = Vec::new();
		for dir in &dirs {
			let mut new_cell = cell.clone();
			while self.inbounds(&new_cell) && self.is_empty(&new_cell) {
				moves.push(cell.clone());
				new_cell.row += dir.0;
				new_cell.col += dir.1;
			}
			// Case for enemy piece collision
			if self.is_enemy(&cell, &new_cell) {
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
			if self.is_enemy(&cell, &diag_right) {
				moves.push(diag_right);
			}
			if self.is_enemy(&cell, &diag_left) {
				moves.push(diag_left);
			}
		}
		moves
	}

	/// Helper function to implement logic for pieces with set directions.
	fn basic_moves(&self, dirs: Vec<(i32, i32)>, cell: Cell) -> Vec<Cell> {
		let mut moves = Vec::new();
		for dir in dirs {
			let mut new_cell = cell.clone();
			new_cell.row = new_cell.row + dir.0;
			new_cell.col = new_cell.col + dir.1;
			if self.is_empty(&new_cell) || self.is_enemy(&cell, &new_cell) {
				moves.push(new_cell);
			}
		}
		moves
	}

	/// Calculate the potential moves for a given cell index.
	pub fn potential_moves(&self, cell: &Cell) -> Vec<Cell> {
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
			moves.retain(|m| !self.self_check(cell.clone(), m.clone()));
		}
		moves
	}

	/// Helper function to check if a move would place the current player's king in danger.
	fn self_check(&self, from: Cell, to: Cell) -> bool {
		let mut new_board = self.clone();
		new_board.move_piece(from, to);
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

	/// Helper function to return cells of all friendly pieces.
	fn friendly_pieces(&self) -> Vec<Cell> {
		let mut cells = Vec::new();
		for row in &self.board {
			for piece_option in row {
				if let &Some(ref piece) = piece_option {
					if piece.color == self.color {
						cells.push(piece.cell.clone());
					}
				}
			}
		}
		cells
	}

	/// Helper function to return cells of all enemy pieces.
	fn enemy_pieces(&self) -> Vec<Cell> {
		let mut cells = Vec::new();
		for row in &self.board {
			for piece_option in row {
				if let &Some(ref piece) = piece_option {
					if piece.color != self.color {
						cells.push(piece.cell.clone());
					}
				}
			}
		}
		cells
	}

	/// Helper function that checks for check, i.e. if the enemy king is in danger.
	fn check(&self) -> bool {
		for friendly_cell in self.friendly_pieces() {
			if self.contains_enemy_king(self.potential_moves(&friendly_cell)) {
				return true
			}
		}
		false
	}

	/// Helper function that checks for checkmate
	fn checkmate(&self) -> bool {
		for cell in self.enemy_pieces() {
			for potential_move in self.potential_moves(&cell) {
				let mut new_board = self.clone();
				new_board.move_piece(cell.clone(), potential_move.clone());
				if new_board.check() == false {
					return false
				}
			}
		}
		true
	}

	/// Helper function that moves a piece to a target cell
	fn move_piece(&mut self, from: Cell, to: Cell) -> bool {
		if let Some(ref mut piece) = self.get_piece(&from).clone() {
			self.board[from.row as usize][from.col as usize] = None;
			self.board[to.row as usize][to.col as usize] = Some(piece.clone());
			piece.cell = to;
			return true
		}
		false
	}

	/// Helper function that checks if a list of potential 
	/// moves contains the enemy king.
	fn contains_enemy_king(&self, cells: Vec<Cell>) -> bool {
		for cell in cells {
			if let &Some(ref piece) = self.get_piece(&cell) {
				if piece.piece_type == PieceType::King && piece.color != self.color {
					return true
				}
			}
		}
		false
	}
}
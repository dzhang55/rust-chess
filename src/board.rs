//! A module for the game logic of Chess. Stores the Board object and contains
//! methods for finding potential moves, check/checkmate, etc. It utilizes Cell
//! objects in order to index into the board, which is a Vec x Vec of Piece
//! options. The Piece object contains an enum of PieceType, which is used to
//! determine the game behavior for each piece.

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Represents the index of a single cell in the board.
pub struct Cell {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Represents the board state.
/// color represents the turn i.e. white indicates it is white's turn.
/// The board is represented by an 8x8 matrix of `Option<Piece>`.
pub struct Board {
    color: Color,
    pub board: Vec<Vec<Option<Piece>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(RustcDecodable, RustcEncodable)]
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
#[derive(RustcDecodable, RustcEncodable)]
/// Represents the color of a piece.
enum Color {
    Black,
    White,
}

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Represents a single chess piece.
/// Stores the type of the piece, color, as well as the cell it resides in.
pub struct Piece {
    piece_type: PieceType,
    color: Color,
    cell: Cell,
}

impl Cell {
    pub fn new(row: i32, col: i32) -> Cell {
        Cell{
            row: row,
            col: col
        }
    }
}

impl Board {
    /// Helper function to put the four symmetrical pieces on the board.
    fn symmetrical_pieces(black_i: i32, j: i32, board: &mut Vec<Vec<Option<Piece>>>, piece_type: PieceType) {
        board[black_i as usize][j as usize] = Some(Piece{
            piece_type: piece_type.clone(),
            color: Color::Black,
            cell: Cell::new(black_i, j)
        });
        board[black_i as usize][7 - j as usize] = Some(Piece{
            piece_type: piece_type.clone(),
            color: Color::Black,
            cell: Cell::new(black_i, 7 - j)
        });
        board[7 - black_i as usize][j as usize] = Some(Piece{
            piece_type: piece_type.clone(),
            color: Color::White,
            cell: Cell::new(7 - black_i, j)
        });
        board[7 - black_i as usize][7 - j as usize] = Some(Piece{
            piece_type: piece_type.clone(),
            color: Color::White,
            cell: Cell::new(7 - black_i, 7 - j)
        });
    }

    /// Initialize the board with starting positions.
    pub fn new() -> Board {
        let mut board = Vec::new();
        for _ in 0..8 {
            let mut row = Vec::new();
            for _ in 0..8 {
                row.push(None);
            }
            board.push(row);
        }
        for j in 0..8 {
            board[1][j] = Some(Piece{
                piece_type: PieceType::Pawn,
                color: Color::Black,
                cell: Cell::new(1, j as i32)
            });
            board[6][j] = Some(Piece{
                piece_type: PieceType::Pawn,
                color: Color::White,
                cell: Cell::new(6, j as i32)
            });
        }
        Board::symmetrical_pieces(0, 0, &mut board, PieceType::Rook);
        Board::symmetrical_pieces(0, 1, &mut board, PieceType::Knight);
        Board::symmetrical_pieces(0, 2, &mut board, PieceType::Bishop);
        board[0][3] = Some(Piece{
            piece_type: PieceType::Queen,
            color: Color::Black,
            cell: Cell::new(0, 3)
        });
        board[7][3] = Some(Piece{
            piece_type: PieceType::Queen,
            color: Color::White,
            cell: Cell::new(7, 3)
        });
        board[0][4] = Some(Piece{
            piece_type: PieceType::King,
            color: Color::Black,
            cell: Cell::new(0, 4)
        });
        board[7][4] = Some(Piece{
            piece_type: PieceType::King,
            color: Color::White,
            cell: Cell::new(7, 4)
        });

        Board {
            color: Color::White,
            board: board
        }
    }

    /// Helper function that checks if it is white's turn.
    pub fn white_turn(&self) -> bool {
        self.color == Color::White
    }

    /// Get the piece associated with a given cell index.
    fn get_piece(&self, cell: &Cell) -> &Option<Piece> {
        &self.board[cell.row as usize][cell.col as usize]
    }

    /// Helper function to check if a proposed cell is in the bounds of the board.
    fn inbounds(&self, cell: &Cell) -> bool {
        cell.row < 8 && cell.row >= 0 && cell.col < 8 && cell.col >= 0
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

    /// Helper function to check if a cell holds a friendly piece relative to board.
    pub fn is_friendly_board(&self, cell: &Cell) -> bool {
        if let &Some(ref piece) = self.get_piece(cell) {
            return piece.color == self.color
        }
        false
    }

    /// Returns the potential moves in the given directions, stopping upon a collision.
    /// Used for Bishop, Rook, and Queen.
    fn moves_until_collision(&self, dirs: Vec<(i32, i32)>, cell: Cell) -> Vec<Cell> {
        let mut moves = Vec::new();
        for dir in &dirs {
            let mut new_cell = cell.clone();
            new_cell.row += dir.0;
            new_cell.col += dir.1;
            while self.inbounds(&new_cell) && self.is_empty(&new_cell) {
                moves.push(new_cell.clone());
                new_cell.row += dir.0;
                new_cell.col += dir.1;
            }
            // Case for enemy piece collision
            if self.inbounds(&new_cell) && self.is_enemy(&cell, &new_cell) {
                moves.push(new_cell.clone());
            }
        }
        moves
    }

    /// Helper function to implement pawn logic.
    fn pawn_moves(&self, cell: Cell) -> Vec<Cell> {
        let mut moves = Vec::new();
        if let Some(ref piece) = *self.get_piece(&cell) {
            let dir;
            match piece.color {
                Color::Black => {
                    if cell.row == 1 {
                        let new_cell = Cell{row: cell.row + 2, col: cell.col};
                        if self.is_empty(&new_cell) {
                            moves.push(Cell{row: cell.row + 2, col: cell.col});
                        }
                    }
                    dir = 1;
                },
                Color::White => {
                    if cell.row == 6 {
                        let new_cell = Cell{row: cell.row - 2, col: cell.col};
                        if self.is_empty(&new_cell) {
                            moves.push(Cell{row: cell.row - 2, col: cell.col});
                        }
                    }
                    dir = -1;
                }
            }
            let vertical = Cell{row: cell.row + dir, col: cell.col};
            let diag_right = Cell{row: cell.row + dir, col: cell.col + 1};
            let diag_left = Cell{row: cell.row + dir, col: cell.col - 1};
            if self.inbounds(&vertical) && self.is_empty(&vertical) {
                moves.push(vertical);
            }
            if self.inbounds(&diag_right) && self.is_enemy(&cell, &diag_right) {
                moves.push(diag_right);
            }
            if self.inbounds(&diag_left) && self.is_enemy(&cell, &diag_left) {
                moves.push(diag_left);
            }
        }
        moves
    }

    /// Helper function to implement logic for pieces with set directions.
    /// Pieces include the Knight and the King.
    fn basic_moves(&self, dirs: Vec<(i32, i32)>, cell: Cell) -> Vec<Cell> {
        let mut moves = Vec::new();
        for dir in dirs {
            let mut new_cell = cell.clone();
            new_cell.row = new_cell.row + dir.0;
            new_cell.col = new_cell.col + dir.1;
            if self.inbounds(&new_cell) && (self.is_empty(&new_cell) ||
                                            self.is_enemy(&cell, &new_cell)) {
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
        }
        moves
    }

    /// Helper function to check if a move would place the current player's king in danger.
    /// Clones the board and makes the move, before running check() on the clone.
    pub fn self_check(&self, from: Cell, to: Cell) -> bool {
        let mut new_board = self.clone();
        new_board.move_piece(from, to);
        new_board.switch_color();
        new_board.check()
    }

    /// Helper function to swap the current player.
    pub fn switch_color(&mut self) {
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
    /// Iterates through all friendly pieces and checks if the enemy king is in any
    /// of their potential moves.
    pub fn check(&self) -> bool {
        for friendly_cell in self.friendly_pieces() {
            if self.contains_enemy_king(self.potential_moves(&friendly_cell)) {
                return true
            }
        }
        false
    }

    /// Helper function that checks for checkmate.
    /// Iterates through all enemy pieces and checks if any of their potential moves
    /// can bring them out of check.
    pub fn checkmate(&self) -> bool {
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

    /// Helper function that moves a piece from a cell to the target cell
    pub fn move_piece(&mut self, from: Cell, to: Cell) -> bool {
        if let Some(ref mut piece) = self.get_piece(&from).clone() {
            piece.cell = to.clone();
            self.board[from.row as usize][from.col as usize] = None;
            self.board[to.row as usize][to.col as usize] = Some(piece.clone());
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
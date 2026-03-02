// SPDX-License-Identifier: PMPL-1.0-or-later
//! Board management for the logic puzzle game.
//!
//! Provides operations for creating and manipulating the puzzle board,
//! including piece placement, removal, and spatial queries.

use super::{BoardState, LogicPiece};

impl BoardState {
    /// Create a new empty board with the specified dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pieces: Vec::new(),
        }
    }

    /// Create a board with pre-placed pieces.
    pub fn with_pieces(width: u32, height: u32, pieces: Vec<LogicPiece>) -> Self {
        Self {
            width,
            height,
            pieces,
        }
    }

    /// Check if a position is within board bounds.
    pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    /// Check if a position is occupied by any piece.
    pub fn is_occupied(&self, x: u32, y: u32) -> bool {
        self.pieces.iter().any(|p| p.position() == (x, y))
    }

    /// Get the piece at a specific position, if any.
    pub fn piece_at(&self, x: u32, y: u32) -> Option<&LogicPiece> {
        self.pieces.iter().find(|p| p.position() == (x, y))
    }

    /// Get a mutable reference to the piece at a specific position.
    pub fn piece_at_mut(&mut self, x: u32, y: u32) -> Option<&mut LogicPiece> {
        self.pieces.iter_mut().find(|p| p.position() == (x, y))
    }

    /// Add a piece to the board if the position is valid and unoccupied.
    /// Returns true if the piece was placed successfully.
    pub fn place_piece(&mut self, piece: LogicPiece) -> bool {
        let (x, y) = piece.position();

        if !self.in_bounds(x, y) {
            return false;
        }

        if self.is_occupied(x, y) {
            return false;
        }

        self.pieces.push(piece);
        true
    }

    /// Remove a piece at the specified position.
    /// Returns the removed piece if found.
    pub fn remove_piece(&mut self, x: u32, y: u32) -> Option<LogicPiece> {
        let index = self.pieces.iter().position(|p| p.position() == (x, y))?;
        Some(self.pieces.remove(index))
    }

    /// Move a piece from one position to another.
    /// Returns true if the move was successful.
    pub fn move_piece(&mut self, from: (u32, u32), to: (u32, u32)) -> bool {
        if !self.in_bounds(to.0, to.1) {
            return false;
        }

        if self.is_occupied(to.0, to.1) {
            return false;
        }

        if let Some(piece) = self.piece_at_mut(from.0, from.1) {
            piece.set_position(to);
            true
        } else {
            false
        }
    }

    /// Get all pieces within a given radius of a position.
    pub fn pieces_near(&self, x: u32, y: u32, radius: u32) -> Vec<&LogicPiece> {
        self.pieces
            .iter()
            .filter(|p| {
                let (px, py) = p.position();
                let dx = (px as i32 - x as i32).unsigned_abs();
                let dy = (py as i32 - y as i32).unsigned_abs();
                dx <= radius && dy <= radius
            })
            .collect()
    }

    /// Get all assumptions on the board.
    pub fn assumptions(&self) -> Vec<&LogicPiece> {
        self.pieces
            .iter()
            .filter(|p| matches!(p, LogicPiece::Assumption { .. }))
            .collect()
    }

    /// Get all goals on the board.
    pub fn goals(&self) -> Vec<&LogicPiece> {
        self.pieces
            .iter()
            .filter(|p| matches!(p, LogicPiece::Goal { .. }))
            .collect()
    }

    /// Get all logic gates (AND, OR, IMPLIES, NOT) on the board.
    pub fn gates(&self) -> Vec<&LogicPiece> {
        self.pieces
            .iter()
            .filter(|p| {
                matches!(
                    p,
                    LogicPiece::AndIntro { .. }
                        | LogicPiece::OrIntro { .. }
                        | LogicPiece::ImpliesIntro { .. }
                        | LogicPiece::NotIntro { .. }
                )
            })
            .collect()
    }

    /// Get all wires on the board.
    pub fn wires(&self) -> Vec<&LogicPiece> {
        self.pieces
            .iter()
            .filter(|p| matches!(p, LogicPiece::Wire { .. }))
            .collect()
    }

    /// Clear all pieces from the board.
    pub fn clear(&mut self) {
        self.pieces.clear();
    }

    /// Get the total number of pieces on the board.
    pub fn piece_count(&self) -> usize {
        self.pieces.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = BoardState::new(10, 10);
        assert_eq!(board.width, 10);
        assert_eq!(board.height, 10);
        assert!(board.pieces.is_empty());
    }

    #[test]
    fn test_bounds_check() {
        let board = BoardState::new(10, 10);
        assert!(board.in_bounds(0, 0));
        assert!(board.in_bounds(9, 9));
        assert!(!board.in_bounds(10, 0));
        assert!(!board.in_bounds(0, 10));
    }

    #[test]
    fn test_place_piece() {
        let mut board = BoardState::new(10, 10);
        let piece = LogicPiece::AndIntro { position: (5, 5) };

        assert!(board.place_piece(piece.clone()));
        assert!(board.is_occupied(5, 5));
        assert!(!board.is_occupied(6, 6));

        // Can't place another piece at the same position
        let piece2 = LogicPiece::OrIntro { position: (5, 5) };
        assert!(!board.place_piece(piece2));
    }

    #[test]
    fn test_remove_piece() {
        let mut board = BoardState::new(10, 10);
        let piece = LogicPiece::AndIntro { position: (5, 5) };
        board.place_piece(piece);

        assert!(board.is_occupied(5, 5));
        let removed = board.remove_piece(5, 5);
        assert!(removed.is_some());
        assert!(!board.is_occupied(5, 5));
    }

    #[test]
    fn test_move_piece() {
        let mut board = BoardState::new(10, 10);
        let piece = LogicPiece::AndIntro { position: (5, 5) };
        board.place_piece(piece);

        assert!(board.move_piece((5, 5), (7, 7)));
        assert!(!board.is_occupied(5, 5));
        assert!(board.is_occupied(7, 7));
    }

    #[test]
    fn test_pieces_near() {
        let mut board = BoardState::new(10, 10);
        board.place_piece(LogicPiece::AndIntro { position: (5, 5) });
        board.place_piece(LogicPiece::OrIntro { position: (6, 5) });
        board.place_piece(LogicPiece::NotIntro { position: (9, 9) });

        let near = board.pieces_near(5, 5, 2);
        assert_eq!(near.len(), 2);
    }
}

#[derive(Debug)]
pub struct ChessPosition {
    column: i32,
    row: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 8 || file >= 8 ||
            rank < 0 || file < 0 { return None; }
        Some(ChessPosition {
            column: rank,
            row: file,
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            position,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.column == other.position.column ||
        self.position.row == other.position.row {
            return true;
        } else if (self.position.column - other.position.column).abs() ==
        (self.position.row - other.position.row).abs() {
            return true;
        } else {
            return false;
        }
    }
}

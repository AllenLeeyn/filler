#[derive(Debug, Clone)]
pub struct Piece {
    pub rows: usize,
    pub cols: usize,
    pub cels: Vec<Vec<char>>,
}
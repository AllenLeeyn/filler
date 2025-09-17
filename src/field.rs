#[derive(Debug, Clone)]
pub struct Field {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<char>>,
}
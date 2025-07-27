#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Symbol {
    Empty,
    Cross,
    Circle,
}

impl Symbol {
    pub fn to_str(&self) -> &str {
        match self {
            Symbol::Empty => " ",
            Symbol::Cross => "X",
            Symbol::Circle => "O",
        }
    }
}

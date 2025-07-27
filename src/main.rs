mod grid;
mod players;
mod symbols;

fn main() {
    // let symbol = symbols::Symbol::Cross;
    // println!("The symbol is: {}", symbol.to_str());

    // let mut grid = grid::Grid::new();
    // grid.set_symbol(0, symbols::Symbol::Circle);
    // grid.set_symbol(1, symbols::Symbol::Circle);
    // grid.set_symbol(2, symbols::Symbol::Circle);
    // grid.set_symbol(3, symbols::Symbol::Circle);
    // grid.print();
    // println!("Is the grid full? {}", grid.is_full());
    // println!("Has someone won? {}", grid.is_won());

    let mut player1 = players::RandomPlayer::new("Randomak".to_string(), symbols::Symbol::Cross);
    let mut player2 = players::HumanPlayer::new("Jitus".to_string(), symbols::Symbol::Circle);
    players::play(&mut player1, &mut player2);
}

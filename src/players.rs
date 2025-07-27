use crate::grid::Grid;
use crate::symbols::Symbol;
use rand::Rng;

// Common behavior for all player types
pub trait Player {
    fn play_turn(&self, grid: &mut Grid) -> bool; // Returns true if the turn was successful
    fn you_won(&self); // Notify the player that they have won
    fn get_symbol(&self) -> Symbol;
}

pub fn play(player1: &mut dyn Player, player2: &mut dyn Player) {
    // Initialize the grid and set the first player
    let mut grid = Grid::new();
    let mut current_player: &mut dyn Player = player1;

    // Game loop, max 9 turns in 3x3 grid
    println!("Starting the game!");
    while !grid.is_full() {
        grid.print();
        if current_player.play_turn(&mut grid) {
            if grid.is_won() {
                grid.print();
                current_player.you_won();
                return;
            }
            // Switch players
            current_player = if current_player.get_symbol() == player1.get_symbol() {
                player2
            } else {
                player1
            };
        }
    }

    grid.print();
    println!("The game ended in a draw!");
}

pub struct RandomPlayer {
    name: String,
    symbol: Symbol,
}

impl RandomPlayer {
    pub fn new(name: String, symbol: Symbol) -> Self {
        RandomPlayer { name, symbol }
    }
}

impl Player for RandomPlayer {
    fn get_symbol(&self) -> Symbol {
        self.symbol
    }

    // TODO: Remove prints when it works
    fn play_turn(&self, grid: &mut Grid) -> bool {
        let empty_positions: Vec<usize> = (0..9).filter(|&i| grid.0[i] == Symbol::Empty).collect();
        let index = empty_positions[rand::rng().random_range(0..empty_positions.len())];

        if grid.set_symbol(index, self.symbol) {
            println!(
                "{} ({}) played at index {}",
                self.name,
                self.symbol.to_str(),
                index
            );
            true
        } else {
            println!(
                "{} tried to play at index {}, but it was invalid",
                self.name, index
            );
            false
        }
    }

    fn you_won(&self) {
        println!(
            "{} ({}) is celebrating their victory!",
            self.name,
            self.symbol.to_str()
        );
    }
}

pub struct HumanPlayer {
    name: String,
    symbol: Symbol,
}

impl HumanPlayer {
    pub fn new(name: String, symbol: Symbol) -> Self {
        HumanPlayer { name, symbol }
    }
}

impl Player for HumanPlayer {
    fn get_symbol(&self) -> Symbol {
        self.symbol
    }

    // TODO: Remove prints when it works
    fn play_turn(&self, grid: &mut Grid) -> bool {
        let mut success = false;
        let mut input = 0;
        while !success {
            println!(
                "{} ({}) - Enter your move (1-9): ",
                self.name,
                self.symbol.to_str()
            );
            let mut input_str = String::new();
            std::io::stdin().read_line(&mut input_str).unwrap();
            input = input_str.trim().parse::<usize>().unwrap() - 1; // Convert to 0-based index
            success = grid.set_symbol(input, self.symbol);
            if !success {
                println!("Invalid move! Try again.");
            }
        }
        println!(
            "{} ({}) played at index {}",
            self.name,
            self.symbol.to_str(),
            input
        );
        true
    }

    fn you_won(&self) {
        println!(
            "{} ({}) is celebrating their victory!",
            self.name,
            self.symbol.to_str()
        );
    }
}

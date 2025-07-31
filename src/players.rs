use crate::grid::{Grid, grid_to_index, index_to_grid};
use crate::symbols::Symbol;
use rand::Rng;

// For weighted random selection
use rand::distributions::WeightedIndex;
use rand::prelude::*;

// Common behavior for all player types
pub trait Player {
    fn play_turn(&mut self, grid: &mut Grid) -> bool; // Returns true if the turn was successful
    fn you_won(&mut self); // Notify the player that they have won
    fn get_symbol(&self) -> Symbol;
    fn get_name(&self) -> &str;
    fn you_lost(&mut self);
    fn its_a_tie(&mut self);
}

pub fn play(player1: &mut dyn Player, player2: &mut dyn Player) -> usize {
    // Initialize the grid and set the first player
    let mut grid = Grid::new();
    let mut current_player: &mut dyn Player = player1;

    while !grid.is_full() {
        // grid.print();
        // println!();
        if current_player.play_turn(&mut grid) {
            if grid.is_won() {
                // grid.print();
                current_player.you_won();
                if current_player.get_symbol() == player1.get_symbol() {
                    player2.you_lost();
                    // println!(
                    //     "Player {} ({}) wins!",
                    //     player1.get_name(),
                    //     player1.get_symbol().to_str()
                    // );
                    return 1; // Player 1 won
                } else {
                    player1.you_lost();
                    // println!(
                    //     "Player {} ({}) wins!",
                    //     player2.get_name(),
                    //     player2.get_symbol().to_str()
                    // );
                    return 2; // Player 2 won
                }
            }
            // Switch players
            current_player = if current_player.get_symbol() == player1.get_symbol() {
                player2
            } else {
                player1
            };
        }
    }

    // grid.print();
    // println!("The game ended in a draw!");
    player1.its_a_tie();
    player2.its_a_tie();
    0 // Return 0 for a tie
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

fn random_empty_position(grid: &Grid) -> usize {
    let empty_positions: Vec<usize> = (0..9).filter(|&i| grid.0[i] == Symbol::Empty).collect();
    let mut rng = rand::thread_rng();
    empty_positions[rng.gen_range(0..empty_positions.len())]
}

impl Player for RandomPlayer {
    fn get_symbol(&self) -> Symbol {
        self.symbol
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    // TODO: Remove prints when it works
    fn play_turn(&mut self, grid: &mut Grid) -> bool {
        let index = random_empty_position(grid);

        if grid.set_symbol(index, self.symbol) {
            // println!(
            //     "{} ({}) played at index {}",
            //     self.name,
            //     self.symbol.to_str(),
            //     index
            // );
            true
        } else {
            println!(
                "{} tried to play at index {}, but it was invalid",
                self.name, index
            );
            false
        }
    }

    fn you_won(&mut self) {}
    fn you_lost(&mut self) {}
    fn its_a_tie(&mut self) {}
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

    fn get_name(&self) -> &str {
        &self.name
    }

    // TODO: Remove prints when it works
    fn play_turn(&mut self, grid: &mut Grid) -> bool {
        let mut success = false;
        let mut input = 0;
        println!(
            "{} ({}) - Enter your move (1-9): ",
            self.name,
            self.symbol.to_str()
        );
        while !success {
            let mut input_str = String::new();
            std::io::stdin().read_line(&mut input_str).unwrap();

            // Handle invalid input
            match input_str.trim().parse::<usize>() {
                Ok(parsed_input) => {
                    input = parsed_input - 1; // Convert to 0-based index
                    success = grid.set_symbol(input, self.symbol);
                    if !success {
                        println!("Invalid move! Try again.");
                    }
                }
                Err(_) => {
                    println!("Please enter a valid number (1-9)!");
                }
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

    fn you_won(&mut self) {}
    fn you_lost(&mut self) {}
    fn its_a_tie(&mut self) {}
}

pub struct MenacePlayer {
    name: String,
    symbol: Symbol,
    grids: Vec<[usize; 9]>,          // vector of matchboxes
    grids_this_game: Vec<usize>,     // indexes of grids played this game
    positions_this_game: Vec<usize>, // positions played this game
}

impl MenacePlayer {
    pub fn new(name: String, symbol: Symbol) -> Self {
        let initial_matches = [255, 255, 255, 255, 255, 255, 255, 255, 255];
        let grids = vec![initial_matches; 3_usize.pow(9) - 1];
        MenacePlayer {
            name,
            symbol,
            grids,
            grids_this_game: Vec::new(),
            positions_this_game: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.grids_this_game.clear();
        self.positions_this_game.clear();
    }

    pub fn get_grids(&self) -> &Vec<[usize; 9]> {
        &self.grids
    }
}

impl Player for MenacePlayer {
    fn get_symbol(&self) -> Symbol {
        self.symbol
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn play_turn(&mut self, grid: &mut Grid) -> bool {
        let index = grid_to_index(grid);
        let mut matchbox = self.grids[index];

        // If all possible moves in the matchbox are 0, refill it
        let mut refill = true;
        for (i, &weight) in matchbox.iter().enumerate() {
            if grid.0[i] == Symbol::Empty && weight != 0 {
                refill = false;
            }
        }
        if refill {
            // Refill the matchbox
            matchbox = [255; 9];
            self.grids[index] = matchbox;
        }

        // Weighted random selection https://docs.rs/nannou/latest/nannou/rand/rand/distributions/struct.WeightedIndex.htmls
        let mut weights: Vec<usize> = Vec::new();
        for i in 0..9 {
            if grid.0[i] == Symbol::Empty {
                weights.push(matchbox[i]);
            } else {
                weights.push(0);
            }
        }

        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::thread_rng();
        let selected_index = dist.sample(&mut rng);
        // println!("{:?} index {}", weights, selected_index);

        self.grids_this_game.push(grid_to_index(grid));
        self.positions_this_game.push(selected_index);

        grid.set_symbol(selected_index, self.symbol);
        // println!(
        //     "{} ({}) played at index {}",
        //     self.name,
        //     self.symbol.to_str(),
        //     selected_index
        // );
        true
    }

    fn you_won(&mut self) {
        // println!("I won!");
        // // let mut weights_before = Vec::new();
        // // let mut weights_after = Vec::new();
        // println!("{:?}", self.grids_this_game);
        // println!("{:?}", self.positions_this_game);
        // for grid in self.grids_this_game.iter() {
        //     println!("Grid index: {}", grid);
        //     index_to_grid(*grid).print();
        // }

        for (i, &grid_index) in self.grids_this_game.iter().enumerate() {
            // weights_before.push(self.grids[grid_index]);
            self.grids[grid_index][self.positions_this_game[i]] += 1; // Increase the weight of the winning move
            // weights_after.push(self.grids[grid_index]);
        }
        // println!("Weights before: {:?}", weights_before);
        // println!("Weights after: {:?}", weights_after);
        self.reset();
    }

    fn you_lost(&mut self) {
        for (i, &grid_index) in self.grids_this_game.iter().enumerate() {
            // Decrease the weight of the losing move, but min 0 using https://users.rust-lang.org/t/is-there-a-more-idiomatic-way-of-doing-this/56768/6
            self.grids[grid_index][self.positions_this_game[i]] =
                match self.grids[grid_index][self.positions_this_game[i]].checked_sub(5) {
                    Some(value) => value,
                    None => 0, // Prevent underflow
                };
        }

        // print number of overall modified matchboxes for debugging
        // let mut x = 0;
        // for (i, grid) in self
        //     .grids
        //     .iter()
        //     .filter(|grid| grid.iter().any(|&weight| weight != 100))
        //     .enumerate()
        // {
        //     x += 1;
        // }
        // println!("Number of modified matchboxes: {}", x);

        self.reset();
    }

    fn its_a_tie(&mut self) {
        // println!("It's a tie!");
        // let mut x = 0;
        // for (i, grid) in self
        //     .grids
        //     .iter()
        //     .filter(|grid| grid.iter().any(|&weight| weight != 100))
        //     .enumerate()
        // {
        //     x += 1;
        // }
        // println!("Number of modified matchboxes: {}", x);
        self.reset();
    }
}

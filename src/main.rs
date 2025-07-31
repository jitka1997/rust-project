mod grid;
mod players;
mod symbols;
use std::fs::File;
use std::io::{BufWriter, Write};

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

    // let mut player1 = players::RandomPlayer::new("Randomak".to_string(), symbols::Symbol::Cross);
    // let mut player2 = players::HumanPlayer::new("Jitus".to_string(), symbols::Symbol::Circle);
    // players::play(&mut player1, &mut player2);

    // let mut grid = grid::Grid::new();
    // grid.set_symbol(0, symbols::Symbol::Cross);
    // grid.set_symbol(1, symbols::Symbol::Empty);
    // grid.set_symbol(2, symbols::Symbol::Cross);

    // grid.print();
    // let index = grid::grid_to_index(&grid);
    // println!("Grid index: {}", index);
    // let new_grid = grid::index_to_grid(index);
    // new_grid.print();

    let mut player1 = players::MenacePlayer::new("Menace".to_string(), symbols::Symbol::Circle);
    let mut player2 = players::RandomPlayer::new("Randomak".to_string(), symbols::Symbol::Cross);
    let mut results = Vec::new();
    for i in 0..10000000 {
        if i % 1000000 == 0 {
            println!("Game {}", i + 1);
        }
        let result = players::play(&mut player1, &mut player2);
        results.push(match result {
            0 => 0,   // Tie
            1 => 1,   // Player 1 won
            2 => -1,  // Player 2 won
            _ => 999, // Unknown result, should not happen
        });
    }
    let grids = player1.get_grids();
    let file = File::create("matchboxes").unwrap();
    let mut writer = BufWriter::new(file);
    for matchbox in grids {
        writeln!(writer, "{:?}", matchbox).unwrap();
    }

    let initial_matchbox = [255, 255, 255, 255, 255, 255, 255, 255, 255];
    let changed = grids
        .iter()
        .filter(|&matchbox| *matchbox != initial_matchbox)
        .collect::<Vec<_>>();
    let changed_indexes: Vec<usize> = grids
        .iter()
        .enumerate()
        .filter_map(|(i, &matchbox)| {
            if matchbox != initial_matchbox {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    let changed_count = changed.len();

    let file = File::create("matchboxes").unwrap();
    let mut writer = BufWriter::new(file);
    writeln!(writer, "Changed matchboxes: {}", changed_count).unwrap();
    for (i, matchbox) in changed.iter().enumerate() {
        writeln!(writer, "{:?}", matchbox).unwrap();
        writeln!(writer, "Index: {}", changed_indexes[i]).unwrap();
        write!(
            writer,
            "{}",
            grid::index_to_grid(changed_indexes[i]).get_pretty_string()
        )
        .unwrap(); // Use write! not writeln!
        writeln!(writer, "").unwrap(); // Add extra newline for separation
    }

    println!("Number of matchboxes that changed: {}", changed_count);
    println!("Total matchboxes: {}", grids.len());
    println!(
        "Percentage changed: {:.2}%",
        (changed_count as f64 / grids.len() as f64) * 100.0
    );

    let json_string = serde_json::to_string(&results).expect("Failed to serialize results");
    let mut file = File::create("results.json").expect("Failed to create file");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to file");

    println!("Results written to results.json");

    // TODO: skusit si vypisat ktore gridy su zmenene, realne v human readable formate a ci davaju zmysel tie vahy
}

mod grid;
mod players;
mod symbols;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut player1 = players::MenacePlayer::new("Menace".to_string(), symbols::Symbol::Circle);
    let mut player2 = players::RandomPlayer::new("Random".to_string(), symbols::Symbol::Cross);
    let number_of_games = 10_000_000;
    let print_every = 1_000_000;

    // Learn as first player
    let mut results1 = Vec::new();
    for i in 0..number_of_games {
        if i % print_every == 0 {
            println!("Game {}", i);
        }
        let result = players::play(&mut player1, &mut player2, false);
        results1.push(match result {
            0 => 0,   // Tie
            1 => 1,   // Player 1 won
            2 => -1,  // Player 2 won
            _ => 999, // Unknown result, should not happen
        });
    }

    // Learn as second player
    let mut results2 = Vec::new();
    for i in 0..number_of_games {
        if i % print_every == 0 {
            println!("Game {}", number_of_games + i);
        }
        let result = players::play(&mut player2, &mut player1, false);
        results2.push(match result {
            0 => 0,   // Tie
            1 => -1,  // Player 1 won (Menace lost)
            2 => 1,   // Player 2 won (Menace won)
            _ => 999, // Unknown result, should not happen
        });
    }

    // Combine results like: [1,2,3] and [3,4,5] -> [1,3,2,4,3,5] for the python script to analyze correctly
    let results: Vec<i32> = results1
        .iter()
        .zip(results2.iter())
        .flat_map(|(r1, r2)| [*r1, *r2])
        .collect();

    let json_string = serde_json::to_string(&results).expect("Failed to serialize results");
    let mut file = File::create("results.json").expect("Failed to create file");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to file");

    println!("Results written to results.json");

    // // Uncomment this if you want to play vs trained Menace player
    // let mut human_player = players::HumanPlayer::new("Human".to_string(), symbols::Symbol::Cross);
    // for i in 0..1 {
    //     players::play(&mut human_player, &mut player1, true);
    // }
}

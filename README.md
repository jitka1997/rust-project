## Tic tac toe project

This repo implements a game of simple tic tac toe with 3 possible players

- Random -> plays random moves
- Human -> plays what human writes into the console
- Menace -> learns to play the optimal strategy using simple machine learning technique

### What is where

- `symbols.rs` -> has Symbol type
- `grid.rs` -> has Grid type and everything to do with grid. Two "advanced" functions for transforming specific grid into integer and the inverse operation
- `players.rs` -> module with the 3 players and `play` function to start a game between two players
- `main.rs` -> Trains menace player and includes possibility to play against him as a human

#### Rest of files

- `task` -> the task for the project
- `parse_results.py` -> script from the task to analyze results
- some rust/cargo files

### Results

File `results.json` has currently results of 20_000_000 games of menace player vs random player (half as first player and half as second).

Graph in `graph_10mil.png` analyses how the menace player performs with these hyperparameters.

Its not really visible with this high number of games played, but if you let the menace player train/play for less games (around 100/1000) you will see that the probability of victory fluctuates at the start. This makes sense because menace didnt even play every possible state of the grid and he plays as a random player on a grid he hasnt seen. Also he needs few iterations on a specific grid to "master" it.

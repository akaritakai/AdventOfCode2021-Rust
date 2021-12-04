use crate::puzzle::AbstractPuzzle;
use crate::puzzle01::Puzzle01;
use crate::puzzle02::Puzzle02;
use crate::puzzle03::Puzzle03;
use crate::puzzle04::Puzzle04;
use crate::puzzle_input_fetcher::PuzzleInputFetcher;

mod puzzle;
mod puzzle01;
mod puzzle02;
mod puzzle03;
mod puzzle04;
mod puzzle_input_fetcher;

fn main() {
    let mut fetcher = PuzzleInputFetcher::create();
    let puzzles: Vec<Box<dyn AbstractPuzzle>> = vec![
        Puzzle01::create(fetcher.fetch_puzzle_input(1).unwrap()),
        Puzzle02::create(fetcher.fetch_puzzle_input(2).unwrap()),
        Puzzle03::create(fetcher.fetch_puzzle_input(3).unwrap()),
        Puzzle04::create(fetcher.fetch_puzzle_input(4).unwrap()),
    ];
    for puzzle in puzzles.iter() {
        let day = format!("{:02}", puzzle.get_day());
        println!("Day {} Part 1: {}", day, puzzle.solve_part_1());
        println!("Day {} Part 2: {}", day, puzzle.solve_part_2());
    }
}

use crate::puzzle::AbstractPuzzle;
use crate::puzzle01::Puzzle01;
use crate::puzzle02::Puzzle02;
use crate::puzzle03::Puzzle03;
use crate::puzzle04::Puzzle04;
use crate::puzzle05::Puzzle05;
use crate::puzzle06::Puzzle06;
use crate::puzzle07::Puzzle07;
use crate::puzzle08::Puzzle08;
use crate::puzzle09::Puzzle09;
use crate::puzzle10::Puzzle10;
use crate::puzzle11::Puzzle11;
use crate::puzzle12::Puzzle12;
use crate::puzzle13::Puzzle13;
use crate::puzzle14::Puzzle14;
use crate::puzzle15::Puzzle15;
use crate::puzzle16::Puzzle16;
use crate::puzzle17::Puzzle17;
use crate::puzzle_input_fetcher::PuzzleInputFetcher;

mod letter_ocr;
mod puzzle;
mod puzzle01;
mod puzzle02;
mod puzzle03;
mod puzzle04;
mod puzzle05;
mod puzzle06;
mod puzzle07;
mod puzzle08;
mod puzzle09;
mod puzzle10;
mod puzzle11;
mod puzzle12;
mod puzzle13;
mod puzzle14;
mod puzzle15;
mod puzzle16;
mod puzzle17;
mod puzzle_input_fetcher;

fn main() {
    let mut fetcher = PuzzleInputFetcher::create();
    let puzzles: Vec<Box<dyn AbstractPuzzle>> = vec![
        Puzzle01::create(fetcher.fetch_puzzle_input(1).unwrap()),
        Puzzle02::create(fetcher.fetch_puzzle_input(2).unwrap()),
        Puzzle03::create(fetcher.fetch_puzzle_input(3).unwrap()),
        Puzzle04::create(fetcher.fetch_puzzle_input(4).unwrap()),
        Puzzle05::create(fetcher.fetch_puzzle_input(5).unwrap()),
        Puzzle06::create(fetcher.fetch_puzzle_input(6).unwrap()),
        Puzzle07::create(fetcher.fetch_puzzle_input(7).unwrap()),
        Puzzle08::create(fetcher.fetch_puzzle_input(8).unwrap()),
        Puzzle09::create(fetcher.fetch_puzzle_input(9).unwrap()),
        Puzzle10::create(fetcher.fetch_puzzle_input(10).unwrap()),
        Puzzle11::create(fetcher.fetch_puzzle_input(11).unwrap()),
        Puzzle12::create(fetcher.fetch_puzzle_input(12).unwrap()),
        Puzzle13::create(fetcher.fetch_puzzle_input(13).unwrap()),
        Puzzle14::create(fetcher.fetch_puzzle_input(14).unwrap()),
        Puzzle15::create(fetcher.fetch_puzzle_input(15).unwrap()),
        Puzzle16::create(fetcher.fetch_puzzle_input(16).unwrap()),
        Puzzle17::create(fetcher.fetch_puzzle_input(17).unwrap()),
    ];
    for puzzle in puzzles.iter() {
        let day = format!("{:02}", puzzle.get_day());
        println!("Day {} Part 1: {}", day, puzzle.solve_part_1());
        println!("Day {} Part 2: {}", day, puzzle.solve_part_2());
    }
}

use crate::puzzle::AbstractPuzzle;

pub struct Puzzle04 {
    numbers: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl AbstractPuzzle for Puzzle04 {
    fn get_day(&self) -> u8 {
        4
    }

    fn solve_part_1(&self) -> String {
        let mut boards = self.boards.clone();
        for number in self.numbers.iter() {
            for board in boards.iter_mut() {
                board.add_number(*number);
                if board.won {
                    return board.score().to_string();
                }
            }
        }
        unreachable!()
    }

    fn solve_part_2(&self) -> String {
        let mut boards = self.boards.clone();
        let length = boards.len();
        for number in self.numbers.iter() {
            for i in 0..length {
                boards[i].add_number(*number);
                if boards.iter_mut().all(|board| board.won) {
                    return boards[i].score().to_string();
                }
            }
        }
        unreachable!()
    }
}

impl Puzzle04 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let mut lines = input.lines();
        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let boards = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|token| token.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect::<Vec<u32>>()
            .chunks(25)
            .into_iter()
            .map(BingoBoard::new)
            .collect::<Vec<BingoBoard>>();
        Box::new(Puzzle04 { numbers, boards })
    }
}

#[derive(Clone)]
struct BingoBoard {
    won: bool,
    last_number: u32,
    board: [[u32; 5]; 5],
    marks: [[bool; 5]; 5],
}

impl BingoBoard {
    fn new(numbers: &[u32]) -> BingoBoard {
        let mut board: [[u32; 5]; 5] = [[0; 5]; 5];
        let mut row = 0;
        let mut col = 0;
        for number in numbers {
            board[row][col] = *number;
            col += 1;
            if col == 5 {
                row += 1;
                col = 0;
            }
        }
        BingoBoard {
            won: false,
            last_number: 0,
            board,
            marks: [[false; 5]; 5],
        }
    }

    fn add_number(&mut self, number: u32) {
        if self.won {
            return;
        }
        self.last_number = number;
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col] == number {
                    self.marks[row][col] = true;
                }
            }
        }
        for i in 0..5 {
            let mut row = true;
            let mut col = true;
            for j in 0..5 {
                row &= self.marks[i][j];
                col &= self.marks[j][i];
            }
            if row || col {
                self.won = true;
                return;
            }
        }
    }

    fn score(&self) -> u32 {
        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.marks[row][col] {
                    sum += self.board[row][col];
                }
            }
        }
        sum * self.last_number
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle04::Puzzle04;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
\n\
22 13 17 11  0\n\
 8  2 23  4 24\n\
21  9 14 16  7\n\
 6 10  3 18  5\n\
 1 12 20 15 19\n\
\n\
 3 15  0  2 22\n\
 9 18 13 17  5\n\
19  8  7 25 23\n\
20 11 10 24  4\n\
14 21 16 12  6\n\
\n\
14 21 17 24  4\n\
10 16 15  9 19\n\
18  8 23 26 20\n\
22 11 13  6  5\n\
 2  0 12  3  7";
        let puzzle = Puzzle04::create(&input);
        assert_eq!(puzzle.solve_part_1(), "4512");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/4")).unwrap();
        let puzzle = Puzzle04::create(&input);
        assert_eq!(puzzle.solve_part_1(), "87456");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
\n\
22 13 17 11  0\n\
 8  2 23  4 24\n\
21  9 14 16  7\n\
 6 10  3 18  5\n\
 1 12 20 15 19\n\
\n\
 3 15  0  2 22\n\
 9 18 13 17  5\n\
19  8  7 25 23\n\
20 11 10 24  4\n\
14 21 16 12  6\n\
\n\
14 21 17 24  4\n\
10 16 15  9 19\n\
18  8 23 26 20\n\
22 11 13  6  5\n\
 2  0 12  3  7";
        let puzzle = Puzzle04::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1924");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/4")).unwrap();
        let puzzle = Puzzle04::create(&input);
        assert_eq!(puzzle.solve_part_2(), "15561");
    }
}

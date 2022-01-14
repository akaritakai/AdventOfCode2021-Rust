use crate::puzzle::AbstractPuzzle;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Puzzle05 {
    diagonal_lines: Vec<Vec<u32>>,
    non_diagonal_lines: Vec<Vec<u32>>,
}

impl AbstractPuzzle for Puzzle05 {
    fn get_day(&self) -> u8 {
        5
    }

    fn solve_part_1(&self) -> String {
        let mut count = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                if self.non_diagonal_lines[x][y] > 1 {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut count = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                if self.diagonal_lines[x][y] + self.non_diagonal_lines[x][y] > 1 {
                    count += 1;
                }
            }
        }
        count.to_string()
    }
}

impl Puzzle05 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        }
        let mut diagonal_lines = vec![vec![0_u32; 1000]; 1000];
        let mut non_diagonal_lines = vec![vec![0_u32; 1000]; 1000];
        for cap in RE.captures_iter(input) {
            let x1 = cap[1].parse::<i32>().unwrap();
            let y1 = cap[2].parse::<i32>().unwrap();
            let x2 = cap[3].parse::<i32>().unwrap();
            let y2 = cap[4].parse::<i32>().unwrap();
            let dx = (x2 - x1).signum();
            let dy = (y2 - y1).signum();
            let mut x = x1;
            let mut y = y1;
            while x != x2 + dx || y != y2 + dy {
                if x1 != x2 && y1 != y2 {
                    diagonal_lines[y as usize][x as usize] += 1;
                } else {
                    non_diagonal_lines[y as usize][x as usize] += 1;
                }
                x += dx;
                y += dy;
            }
        }
        Box::new(Puzzle05 {
            diagonal_lines,
            non_diagonal_lines,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle05::Puzzle05;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "0,9 -> 5,9\n\
8,0 -> 0,8\n\
9,4 -> 3,4\n\
2,2 -> 2,1\n\
7,0 -> 7,4\n\
6,4 -> 2,0\n\
0,9 -> 2,9\n\
3,4 -> 1,4\n\
0,0 -> 8,8\n\
5,5 -> 8,2";
        let puzzle = Puzzle05::create(&input);
        assert_eq!(puzzle.solve_part_1(), "5");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/5")).unwrap();
        let puzzle = Puzzle05::create(&input);
        assert_eq!(puzzle.solve_part_1(), "6113");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "0,9 -> 5,9\n\
8,0 -> 0,8\n\
9,4 -> 3,4\n\
2,2 -> 2,1\n\
7,0 -> 7,4\n\
6,4 -> 2,0\n\
0,9 -> 2,9\n\
3,4 -> 1,4\n\
0,0 -> 8,8\n\
5,5 -> 8,2";
        let puzzle = Puzzle05::create(&input);
        assert_eq!(puzzle.solve_part_2(), "12");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/5")).unwrap();
        let puzzle = Puzzle05::create(&input);
        assert_eq!(puzzle.solve_part_2(), "20373");
    }
}

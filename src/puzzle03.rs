use crate::puzzle::AbstractPuzzle;

pub struct Puzzle03 {
    report: Vec<String>,
    length: usize,
}

impl AbstractPuzzle for Puzzle03 {
    fn get_day(&self) -> u8 {
        3
    }

    fn solve_part_1(&self) -> String {
        let mut gamma: u32 = 0;
        let mut epsilon: u32 = 0;
        for i in 0..self.length {
            let mut zeros = 0;
            let mut ones = 0;
            for s in self.report.iter() {
                if s.chars().nth(i).unwrap() == '0' {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }
            if zeros > ones {
                gamma <<= 1;
                epsilon = (epsilon << 1) | 1;
            } else {
                gamma = (gamma << 1) | 1;
                epsilon <<= 1;
            }
        }
        (gamma * epsilon).to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut report = self.report.clone();
        report.sort();
        let mut low = 0;
        let mut high = report.len();
        for i in 0..self.length {
            if high - low == 1 {
                break;
            }
            let mid = find_mid(&report, i, low, high);
            if high - mid >= (high - low + 1) / 2 {
                low = mid;
            } else {
                high = mid;
            }
        }
        let oxygen_rating = usize::from_str_radix(&report[low], 2).unwrap();
        low = 0;
        high = report.len();
        for i in 0..self.length {
            if high - low == 1 {
                break;
            }
            let mid = find_mid(&report, i, low, high);
            if high - mid >= (high - low + 1) / 2 {
                high = mid;
            } else {
                low = mid;
            }
        }
        let co2_rating = usize::from_str_radix(&report[low], 2).unwrap();
        (oxygen_rating * co2_rating).to_string()
    }
}

impl Puzzle03 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let report = input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let length = report[0].len();
        Box::new(Puzzle03 { report, length })
    }
}

fn find_mid(report: &[String], position: usize, mut low: usize, mut high: usize) -> usize {
    while low < high {
        let mid = low + (high - low) / 2;
        if report[mid].chars().nth(position).unwrap() == '1' {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

#[cfg(test)]
mod tests {
    use crate::puzzle03::Puzzle03;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let puzzle = Puzzle03::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "198");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/3")).unwrap();
        let puzzle = Puzzle03::create(&input);
        assert_eq!(puzzle.solve_part_1(), "3885894");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let puzzle = Puzzle03::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "230");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/3")).unwrap();
        let puzzle = Puzzle03::create(&input);
        assert_eq!(puzzle.solve_part_2(), "4375225");
    }
}

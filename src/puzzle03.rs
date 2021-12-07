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
            if most_common_bit(&self.report, i) == '0' {
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
        let mut oxygen_values = self.report.clone();
        for i in 0..self.length {
            if oxygen_values.len() == 1 {
                break;
            }
            let most_common_bit = most_common_bit(&oxygen_values, i);
            oxygen_values.retain(|line| line.chars().nth(i).unwrap() == most_common_bit);
        }
        let mut co2_values = self.report.clone();
        for i in 0..self.length {
            if co2_values.len() == 1 {
                break;
            }
            let least_common_bit = least_common_bit(&co2_values, i);
            co2_values.retain(|line| line.chars().nth(i).unwrap() == least_common_bit);
        }
        let oxygen_rating = usize::from_str_radix(&*oxygen_values[0], 2).unwrap();
        let co2_rating = usize::from_str_radix(&*co2_values[0], 2).unwrap();
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

fn most_common_bit(report: &[String], position: usize) -> char {
    let mut counts = [0; 2];
    for line in report {
        match line.chars().nth(position) {
            Some('0') => counts[0] += 1,
            Some('1') => counts[1] += 1,
            _ => panic!("Invalid character"),
        }
    }
    if counts[0] > counts[1] {
        '0'
    } else {
        '1'
    }
}

fn least_common_bit(report: &[String], position: usize) -> char {
    if most_common_bit(report, position) == '0' {
        '1'
    } else {
        '0'
    }
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

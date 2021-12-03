use crate::puzzle::AbstractPuzzle;

pub struct Puzzle03 {
    input: String,
}

impl AbstractPuzzle for Puzzle03 {
    fn get_day(&self) -> u8 {
        3
    }

    fn solve_part_1(&self) -> String {
        let report = self.input.lines().collect::<Vec<&str>>();
        let length = report[0].len();
        let mut gamma: u32 = 0;
        let mut epsilon: u32 = 0;
        for i in 0..length {
            if more_zeros(&report, i) {
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
        let report = self.input.lines().collect::<Vec<&str>>();
        let length = report[0].len();
        let mut oxygen_values = report.clone();
        for i in 0..length {
            if oxygen_values.len() == 1 {
                break;
            }
            if more_zeros(&oxygen_values, i) {
                oxygen_values.retain(|&line| line.chars().nth(i).unwrap() == '0');
            } else {
                oxygen_values.retain(|&line| line.chars().nth(i).unwrap() == '1');
            }
        }
        let mut co2_values = report.clone();
        for i in 0..length {
            if co2_values.len() == 1 {
                break;
            }
            if more_zeros(&co2_values, i) {
                co2_values.retain(|&line| line.chars().nth(i).unwrap() == '1');
            } else {
                co2_values.retain(|&line| line.chars().nth(i).unwrap() == '0');
            }
        }
        let oxygen_rating = usize::from_str_radix(oxygen_values[0], 2).unwrap();
        let co2_rating = usize::from_str_radix(co2_values[0], 2).unwrap();
        (oxygen_rating * co2_rating).to_string()
    }
}

impl Puzzle03 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle03 {
            input: input.to_string(),
        })
    }
}

fn more_zeros(report: &[&str], position: usize) -> bool {
    let mut count = 0;
    for line in report {
        if line.chars().nth(position).unwrap() == '0' {
            count += 1;
        }
    }
    count > report.len() / 2
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

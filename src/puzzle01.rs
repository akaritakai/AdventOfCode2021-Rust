use crate::puzzle::AbstractPuzzle;

pub struct Puzzle01 {
    depths: Vec<u32>,
}

impl AbstractPuzzle for Puzzle01 {
    fn get_day(&self) -> u8 {
        1
    }

    fn solve_part_1(&self) -> String {
        self.count_increases(1).to_string()
    }

    fn solve_part_2(&self) -> String {
        self.count_increases(3).to_string()
    }
}

impl Puzzle01 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle01 {
            depths: input
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .collect(),
        })
    }

    fn count_increases(&self, window_size: usize) -> u32 {
        let mut count = 0;
        let mut prev_sum = 0;
        for i in 0..window_size {
            prev_sum += self.depths[i];
        }
        for i in window_size..self.depths.len() {
            let sum = prev_sum + self.depths[i] - self.depths[i - window_size];
            if sum > prev_sum {
                count += 1;
            }
            prev_sum = sum;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle01::Puzzle01;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "7");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/1")).unwrap();
        let puzzle = Puzzle01::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1532");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "5");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/1")).unwrap();
        let puzzle = Puzzle01::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1571");
    }
}

use crate::puzzle::AbstractPuzzle;

pub struct Puzzle01 {
    input: String,
}

impl AbstractPuzzle for Puzzle01 {
    fn get_day(&self) -> u8 {
        1
    }

    fn solve_part_1(&self) -> String {
        let depths = self
            .input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut count = 0;
        for i in 1..depths.len() {
            if depths[i] > depths[i - 1] {
                count += 1;
            }
        }
        count.to_string()
    }

    fn solve_part_2(&self) -> String {
        let depths = self
            .input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut count = 0;
        let mut prev_sum = depths[0] + depths[1] + depths[2];
        for i in 3..depths.len() {
            let sum = depths[i - 2] + depths[i - 1] + depths[i];
            if sum > prev_sum {
                count += 1;
            }
            prev_sum = sum;
        }
        count.to_string()
    }
}

impl Puzzle01 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle01 {
            input: input.to_string(),
        })
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

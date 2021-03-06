use crate::puzzle::AbstractPuzzle;
use std::cmp::min;

pub struct Puzzle07 {
    positions: Vec<i32>,
}

impl AbstractPuzzle for Puzzle07 {
    fn get_day(&self) -> u8 {
        7
    }

    fn solve_part_1(&self) -> String {
        let median = self.positions[self.positions.len() / 2];
        self.positions
            .iter()
            .map(|&x| (x - median).abs())
            .sum::<i32>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let mean = (self.positions.iter().sum::<i32>() as f64) / (self.positions.len() as f64);
        let floor_cost = self
            .positions
            .iter()
            .map(|&x| (x as f64 - mean.floor()).abs() as i32)
            .map(|x| x * (x + 1) / 2)
            .sum::<i32>();
        let ceil_cost = self
            .positions
            .iter()
            .map(|&x| (x as f64 - mean.ceil()).abs() as i32)
            .map(|x| x * (x + 1) / 2)
            .sum::<i32>();
        min(floor_cost, ceil_cost).to_string()
    }
}

impl Puzzle07 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let mut positions = input
            .trim()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let length = positions.len();
        let _ = positions.select_nth_unstable(length / 2);
        Box::new(Puzzle07 { positions })
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle07::Puzzle07;
    use std::cmp::min;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let puzzle = Puzzle07::create(&input);
        assert_eq!(puzzle.solve_part_1(), "37");
    }

    #[test]
    fn test_part_1_example_2() {
        // We test all input sizes from 1 to 200 and generate a random number (0-2000) for each
        for size in 1..201 {
            let positions = (0..size)
                .map(|_| (rand::random::<u32>() % 2001) as i32)
                .collect::<Vec<i32>>();

            // Use the naive solution for part 1 as a sanity check
            let mut min_cost = i32::MAX;
            for i in 0..2001 {
                let mut cost = 0;
                for &x in &positions {
                    cost += (x - i).abs();
                }
                min_cost = min(cost, min_cost);
            }

            let puzzle = Puzzle07::create(
                &positions
                    .iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
            assert_eq!(puzzle.solve_part_1(), min_cost.to_string());
        }
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/7")).unwrap();
        let puzzle = Puzzle07::create(&input);
        assert_eq!(puzzle.solve_part_1(), "356922");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let puzzle = Puzzle07::create(&input);
        assert_eq!(puzzle.solve_part_2(), "168");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/7")).unwrap();
        let puzzle = Puzzle07::create(&input);
        assert_eq!(puzzle.solve_part_2(), "100347031");
    }
}

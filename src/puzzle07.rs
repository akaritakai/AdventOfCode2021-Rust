use crate::puzzle::AbstractPuzzle;
use std::cmp::min;

pub struct Puzzle07 {
    positions: Vec<i32>,
    min_position: i32,
    max_position: i32,
}

impl AbstractPuzzle for Puzzle07 {
    fn get_day(&self) -> u8 {
        7
    }

    fn solve_part_1(&self) -> String {
        let mut min_cost = i32::MAX;
        for i in self.min_position..=self.max_position {
            let mut cost = 0;
            for position in self.positions.iter() {
                cost += (position - i).abs();
            }
            min_cost = min(min_cost, cost);
        }
        min_cost.to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut min_cost = i32::MAX;
        for i in self.min_position..=self.max_position {
            let mut cost = 0;
            for position in self.positions.iter() {
                let distance = (position - i).abs();
                cost += distance * (distance + 1) / 2;
            }
            min_cost = min(min_cost, cost);
        }
        min_cost.to_string()
    }
}

impl Puzzle07 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let positions = input
            .trim()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let min_position = *positions.iter().min().unwrap();
        let max_position = *positions.iter().max().unwrap();
        Box::new(Puzzle07 {
            positions,
            min_position,
            max_position,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle07::Puzzle07;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let puzzle = Puzzle07::create(&input);
        assert_eq!(puzzle.solve_part_1(), "37");
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

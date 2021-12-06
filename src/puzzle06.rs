use crate::puzzle::AbstractPuzzle;

pub struct Puzzle06 {
    input: String,
}

impl AbstractPuzzle for Puzzle06 {
    fn get_day(&self) -> u8 {
        6
    }

    fn solve_part_1(&self) -> String {
        simulate(self.input.as_str(), 80).to_string()
    }

    fn solve_part_2(&self) -> String {
        simulate(self.input.as_str(), 256).to_string()
    }
}

impl Puzzle06 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle06 {
            input: input.to_string(),
        })
    }
}

fn simulate(input: &str, days: usize) -> u64 {
    let mut fish: [u64; 9] = [0; 9];
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|x| fish[x] += 1);
    let mut base = 0;
    for _ in 0..days {
        fish[(base + 7) % 9] += fish[base];
        base = (base + 1) % 9;
    }
    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::puzzle06::Puzzle06;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "3,4,3,1,2";
        let puzzle = Puzzle06::create(&input);
        assert_eq!(puzzle.solve_part_1(), "5934");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/6")).unwrap();
        let puzzle = Puzzle06::create(&input);
        assert_eq!(puzzle.solve_part_1(), "349549");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "3,4,3,1,2";
        let puzzle = Puzzle06::create(&input);
        assert_eq!(puzzle.solve_part_2(), "26984457539");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/6")).unwrap();
        let puzzle = Puzzle06::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1589590444365");
    }
}

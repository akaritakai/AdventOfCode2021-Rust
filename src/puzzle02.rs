use crate::puzzle::AbstractPuzzle;

pub struct Puzzle02 {
    instructions: Vec<Instruction>,
}

impl AbstractPuzzle for Puzzle02 {
    fn get_day(&self) -> u8 {
        2
    }

    fn solve_part_1(&self) -> String {
        let mut x = 0;
        let mut y = 0;
        for instruction in &self.instructions {
            match instruction.command.as_str() {
                "forward" => x += instruction.value,
                "down" => y += instruction.value,
                "up" => y -= instruction.value,
                _ => {}
            }
        }
        (x * y).to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;
        for instruction in &self.instructions {
            match instruction.command.as_str() {
                "forward" => {
                    x += instruction.value;
                    y += aim * instruction.value;
                }
                "down" => aim += instruction.value,
                "up" => aim -= instruction.value,
                _ => {}
            }
        }
        (x * y).to_string()
    }
}

impl Puzzle02 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle02 {
            instructions: input
                .lines()
                .map(|line| {
                    let tokens = line.split_whitespace().collect::<Vec<&str>>();
                    Instruction {
                        command: tokens[0].to_string(),
                        value: tokens[1].parse::<i32>().unwrap(),
                    }
                })
                .collect(),
        })
    }
}

struct Instruction {
    command: String,
    value: i32,
}

#[cfg(test)]
mod tests {
    use crate::puzzle02::Puzzle02;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let puzzle = Puzzle02::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "150");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/2")).unwrap();
        let puzzle = Puzzle02::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1604850");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let puzzle = Puzzle02::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "900");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/2")).unwrap();
        let puzzle = Puzzle02::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1685186100");
    }
}

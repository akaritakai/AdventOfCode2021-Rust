use crate::puzzle::AbstractPuzzle;

pub struct Puzzle10 {
    lines: Vec<String>,
}

impl AbstractPuzzle for Puzzle10 {
    fn get_day(&self) -> u8 {
        10
    }

    fn solve_part_1(&self) -> String {
        let mut score = 0;
        for line in &self.lines {
            let mut stack = Vec::new();
            for c in line.chars() {
                if c == '(' || c == '[' || c == '{' || c == '<' {
                    stack.push(c);
                } else if stack.is_empty() {
                    break;
                } else if c == ')' && stack.last().unwrap() == &'('
                    || c == ']' && stack.last().unwrap() == &'['
                    || c == '}' && stack.last().unwrap() == &'{'
                    || c == '>' && stack.last().unwrap() == &'<'
                {
                    stack.pop();
                } else {
                    match c {
                        ')' => score += 3,
                        ']' => score += 57,
                        '}' => score += 1197,
                        '>' => score += 25137,
                        _ => panic!("Invalid character: {}", c),
                    }
                    break;
                }
            }
        }
        score.to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut costs: Vec<u64> = Vec::new();
        for line in &self.lines {
            let mut corrupted = false;
            let mut stack = Vec::new();
            for c in line.chars() {
                if c == '(' || c == '[' || c == '{' || c == '<' {
                    stack.push(c);
                } else if stack.is_empty() {
                    break;
                } else if (c == ')' && stack.last().unwrap() == &'(')
                    || (c == ']' && stack.last().unwrap() == &'[')
                    || (c == '}' && stack.last().unwrap() == &'{')
                    || (c == '>' && stack.last().unwrap() == &'<')
                {
                    stack.pop();
                } else {
                    corrupted = true;
                    break;
                }
            }
            if !corrupted {
                let mut cost = 0;
                while !stack.is_empty() {
                    cost *= 5;
                    match stack.pop() {
                        Some('(') => cost += 1,
                        Some('[') => cost += 2,
                        Some('{') => cost += 3,
                        Some('<') => cost += 4,
                        _ => panic!("Invalid character"),
                    }
                }
                costs.push(cost);
            }
        }
        let mid = costs.len() / 2;
        costs.select_nth_unstable(mid);
        costs[mid].to_string()
    }
}

impl Puzzle10 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle10 {
            lines: input.lines().map(|x| x.to_string()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle10::Puzzle10;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "[({(<(())[]>[[{[]{<()<>>\n\
[(()[<>])]({[<{<<[]>>(\n\
{([(<{}[<>[]}>{[]{[(<()>\n\
(((({<>}<{<{<>}{[]{[]{}\n\
[[<[([]))<([[{}[[()]]]\n\
[{[{({}]{}}([{[{{{}}([]\n\
{<[[]]>}<{[{[{[]{()[[[]\n\
[<(<(<(<{}))><([]([]()\n\
<{([([[(<>()){}]>(<<{{\n\
<{([{{}}[<[[[<>{}]]]>[]]";
        let puzzle = Puzzle10::create(&input);
        assert_eq!(puzzle.solve_part_1(), "26397");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/10")).unwrap();
        let puzzle = Puzzle10::create(&input);
        assert_eq!(puzzle.solve_part_1(), "271245");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "[({(<(())[]>[[{[]{<()<>>\n\
[(()[<>])]({[<{<<[]>>(\n\
{([(<{}[<>[]}>{[]{[(<()>\n\
(((({<>}<{<{<>}{[]{[]{}\n\
[[<[([]))<([[{}[[()]]]\n\
[{[{({}]{}}([{[{{{}}([]\n\
{<[[]]>}<{[{[{[]{()[[[]\n\
[<(<(<(<{}))><([]([]()\n\
<{([([[(<>()){}]>(<<{{\n\
<{([{{}}[<[[[<>{}]]]>[]]";
        let puzzle = Puzzle10::create(&input);
        assert_eq!(puzzle.solve_part_2(), "288957");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/10")).unwrap();
        let puzzle = Puzzle10::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1685293086");
    }
}

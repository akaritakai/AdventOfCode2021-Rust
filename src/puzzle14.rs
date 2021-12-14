use crate::puzzle::AbstractPuzzle;
use std::collections::HashMap;

pub struct Puzzle14 {
    template: String,
    rules: HashMap<String, String>,
}

impl AbstractPuzzle for Puzzle14 {
    fn get_day(&self) -> u8 {
        14
    }

    fn solve_part_1(&self) -> String {
        let mut counter = self.make_counter();
        for _ in 0..10 {
            counter = self.do_step(&counter);
        }
        self.score(&counter).to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut counter = self.make_counter();
        for _ in 0..40 {
            counter = self.do_step(&counter);
        }
        self.score(&counter).to_string()
    }
}

impl Puzzle14 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let parts = input.trim().split("\n\n").collect::<Vec<&str>>();
        let template = parts[0].to_string();
        let rules = parts[1]
            .split('\n')
            .map(|line| {
                let parts = line.split(" -> ").collect::<Vec<&str>>();
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect::<HashMap<String, String>>();
        Box::new(Puzzle14 { template, rules })
    }

    fn make_counter(&self) -> HashMap<String, u64> {
        let mut counter = HashMap::new();
        for i in 1..self.template.len() {
            let mut key = String::new();
            key.push(self.template.chars().nth(i - 1).unwrap());
            key.push(self.template.chars().nth(i).unwrap());
            *counter.entry(key).or_insert(0) += 1;
        }
        counter
    }

    fn do_step(&self, counter: &HashMap<String, u64>) -> HashMap<String, u64> {
        let mut next = HashMap::new();
        for (pair, value) in counter {
            if self.rules.contains_key(pair) {
                let left = pair.chars().next().unwrap();
                let right = pair.chars().nth(1).unwrap();
                let middle = self.rules.get(pair).unwrap();
                *next.entry(format!("{}{}", left, middle)).or_insert(0) += value;
                *next.entry(format!("{}{}", middle, right)).or_insert(0) += value;
            }
        }
        next
    }

    fn score(&self, counter: &HashMap<String, u64>) -> u64 {
        let mut map = [0; 26];
        for (pair, value) in counter {
            let c = pair.chars().next().unwrap() as usize - 'A' as usize;
            map[c] += *value;
        }
        let c = self.template.chars().last().unwrap() as usize - 'A' as usize;
        map[c] += 1;
        let max = map.iter().filter(|&&x| x != 0).max().unwrap();
        let min = map.iter().filter(|&&x| x != 0).min().unwrap();
        max - min
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle14::Puzzle14;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "NNCB\n\
\n\
CH -> B\n\
HH -> N\n\
CB -> H\n\
NH -> C\n\
HB -> C\n\
HC -> B\n\
HN -> C\n\
NN -> C\n\
BH -> H\n\
NC -> B\n\
NB -> B\n\
BN -> B\n\
BB -> N\n\
BC -> B\n\
CC -> N\n\
CN -> C";
        let puzzle = Puzzle14::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1588");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/14")).unwrap();
        let puzzle = Puzzle14::create(&input);
        assert_eq!(puzzle.solve_part_1(), "3247");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "NNCB\n\
\n\
CH -> B\n\
HH -> N\n\
CB -> H\n\
NH -> C\n\
HB -> C\n\
HC -> B\n\
HN -> C\n\
NN -> C\n\
BH -> H\n\
NC -> B\n\
NB -> B\n\
BN -> B\n\
BB -> N\n\
BC -> B\n\
CC -> N\n\
CN -> C";
        let puzzle = Puzzle14::create(&input);
        assert_eq!(puzzle.solve_part_2(), "2188189693529");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/14")).unwrap();
        let puzzle = Puzzle14::create(&input);
        assert_eq!(puzzle.solve_part_2(), "4110568157153");
    }
}

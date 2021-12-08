use crate::puzzle::AbstractPuzzle;

pub struct Puzzle08 {
    displays: Vec<Display>,
}

impl AbstractPuzzle for Puzzle08 {
    fn get_day(&self) -> u8 {
        8
    }

    fn solve_part_1(&self) -> String {
        self.displays
            .iter()
            .map(|display| display.outputs.iter())
            .flatten()
            .filter(|&x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.displays
            .iter()
            .map(|display| display.decode())
            .sum::<usize>()
            .to_string()
    }
}

impl Puzzle08 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let displays = input
            .lines()
            .map(|line| {
                let parts = line
                    .split(" | ")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                let outputs = parts[1]
                    .split(' ')
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                let patterns = line
                    .split(" | ")
                    .map(|part| {
                        part.split(' ')
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                    })
                    .flatten()
                    .collect::<Vec<String>>();
                Display { patterns, outputs }
            })
            .collect::<Vec<Display>>();
        Box::new(Puzzle08 { displays })
    }
}

struct Display {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

impl Display {
    fn decode(&self) -> usize {
        // We can deduce '1', '4', '7', and '8' by their length
        let one = self
            .patterns
            .iter()
            .filter(|x| x.len() == 2)
            .collect::<Vec<&String>>()[0];
        let four = self
            .patterns
            .iter()
            .filter(|x| x.len() == 4)
            .collect::<Vec<&String>>()[0];
        let seven = self
            .patterns
            .iter()
            .filter(|x| x.len() == 3)
            .collect::<Vec<&String>>()[0];
        let eight = self
            .patterns
            .iter()
            .filter(|x| x.len() == 7)
            .collect::<Vec<&String>>()[0];
        // We can deduce '6' as it is the only number to have length 6 and share 1 value in common with '1'
        let six = self
            .patterns
            .iter()
            .filter(|x| x.len() == 6)
            .filter(|x| one.chars().filter(|&y| x.contains(y)).count() == 1)
            .collect::<Vec<&String>>()[0];
        // We can deduce f as the intersection of '6' and '1'
        let f = one
            .chars()
            .filter(|&x| six.contains(x))
            .collect::<String>()
            .chars()
            .next()
            .unwrap();
        // We can deduce c as '1' set minus f
        let c = one
            .chars()
            .filter(|x| x != &f)
            .collect::<String>()
            .chars()
            .next()
            .unwrap();
        // We can deduce '3' as it is the only number to have length 5 and contain both c and f
        let three = self
            .patterns
            .iter()
            .filter(|x| x.len() == 5 && x.contains(c) && x.contains(f))
            .collect::<Vec<&String>>()[0];
        // We can deduce '2' as it is the only number to have length 5 and share 2 values in common with '4'
        let two = self
            .patterns
            .iter()
            .filter(|x| x.len() == 5)
            .filter(|x| x.chars().filter(|&y| four.contains(y)).count() == 2)
            .collect::<Vec<&String>>()[0];
        // We can deduce b as '4' set minus '3'
        let b = four
            .chars()
            .filter(|&x| !three.contains(x))
            .collect::<String>()
            .chars()
            .next()
            .unwrap();
        // We can deduce '5' as it is the only number to have length 5 and contain b
        let five = self
            .patterns
            .iter()
            .filter(|x| x.len() == 5 && x.contains(b))
            .collect::<Vec<&String>>()[0];
        // We can deduce d as '4' set minus '1' set minus 'b'
        let d = four
            .chars()
            .filter(|&x| !one.contains(x) && x != b)
            .collect::<String>()
            .chars()
            .next()
            .unwrap();
        // We can deduce '0' as it is the only number to have length 6 and not contain d
        let zero = self
            .patterns
            .iter()
            .filter(|x| x.len() == 6 && !x.contains(d))
            .collect::<Vec<&String>>()[0];
        // We can deduce '9' as it is the only number to have length 6 and contain both c and d
        let nine = self
            .patterns
            .iter()
            .filter(|x| x.len() == 6 && x.contains(c) && x.contains(d))
            .collect::<Vec<&String>>()[0];
        let digits = vec![
            sort(zero),
            sort(one),
            sort(two),
            sort(three),
            sort(four),
            sort(five),
            sort(six),
            sort(seven),
            sort(eight),
            sort(nine),
        ];
        // We can now decode the output
        let mut output = 0;
        for pattern in &self.outputs {
            let sorted_pattern = sort(pattern);
            for (i, digit) in digits.iter().enumerate().take(10) {
                if sorted_pattern == *digit {
                    output = 10 * output + i;
                }
            }
        }
        output
    }
}

fn sort(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::puzzle08::Puzzle08;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let puzzle = Puzzle08::create(&input);
        assert_eq!(puzzle.solve_part_1(), "26");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/8")).unwrap();
        let puzzle = Puzzle08::create(&input);
        assert_eq!(puzzle.solve_part_1(), "318");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let puzzle = Puzzle08::create(&input);
        assert_eq!(puzzle.solve_part_2(), "61229");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/8")).unwrap();
        let puzzle = Puzzle08::create(&input);
        assert_eq!(puzzle.solve_part_2(), "996280");
    }
}

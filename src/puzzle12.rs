use crate::puzzle::AbstractPuzzle;
use std::collections::{HashMap, HashSet, LinkedList};

pub struct Puzzle12 {
    edges: HashMap<String, HashSet<String>>,
}

impl AbstractPuzzle for Puzzle12 {
    fn get_day(&self) -> u8 {
        12
    }

    fn solve_part_1(&self) -> String {
        let mut paths = LinkedList::new();
        self.count_paths("start", &mut paths, true).to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut paths = LinkedList::new();
        self.count_paths("start", &mut paths, false).to_string()
    }
}

impl Puzzle12 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let mut edges = HashMap::new();
        for line in input.lines() {
            let mut parts = line.split('-');
            let from = parts.next().unwrap();
            let to = parts.next().unwrap();
            edges
                .entry(from.to_string())
                .or_insert_with(HashSet::new)
                .insert(to.to_string());
            edges
                .entry(to.to_string())
                .or_insert_with(HashSet::new)
                .insert(from.to_string());
        }
        Box::new(Puzzle12 { edges })
    }

    fn count_paths(&self, cave: &str, path: &mut LinkedList<String>, mut seen_twice: bool) -> u32 {
        if cave == "end" {
            return 1;
        }
        if is_small_cave(cave) && path.contains(&cave.to_string()) {
            if seen_twice || cave == "start" {
                return 0;
            }
            seen_twice = true;
        }
        path.push_back(cave.to_string());
        let mut count = 0;
        for next in self.edges.get(cave).unwrap() {
            count += self.count_paths(next, path, seen_twice);
        }
        path.pop_back();
        count
    }
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().next().unwrap() >= 'a'
}

#[cfg(test)]
mod tests {
    use crate::puzzle12::Puzzle12;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "start-A\n\
start-b\n\
A-c\n\
A-b\n\
b-d\n\
A-end\n\
b-end";
        let puzzle = Puzzle12::create(&input);
        assert_eq!(puzzle.solve_part_1(), "10");
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "dc-end\n\
HN-start\n\
start-kj\n\
dc-start\n\
dc-HN\n\
LN-dc\n\
HN-end\n\
kj-sa\n\
kj-HN\n\
kj-dc";
        let puzzle = Puzzle12::create(&input);
        assert_eq!(puzzle.solve_part_1(), "19");
    }

    #[test]
    fn test_part_1_example_3() {
        let input = "fs-end\n\
he-DX\n\
fs-he\n\
start-DX\n\
pj-DX\n\
end-zg\n\
zg-sl\n\
zg-pj\n\
pj-he\n\
RW-he\n\
fs-DX\n\
pj-RW\n\
zg-RW\n\
start-pj\n\
he-WI\n\
zg-he\n\
pj-fs\n\
start-RW";
        let puzzle = Puzzle12::create(&input);
        assert_eq!(puzzle.solve_part_1(), "226");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/12")).unwrap();
        let puzzle = Puzzle12::create(&input);
        assert_eq!(puzzle.solve_part_1(), "4338");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "start-A\n\
start-b\n\
A-c\n\
A-b\n\
b-d\n\
A-end\n\
b-end";
        let puzzle = Puzzle12::create(&input);
        assert_eq!(puzzle.solve_part_2(), "36");
    }

    #[test]
    fn test_part_2_example_2() {
        let input = "dc-end\n\
HN-start\n\
start-kj\n\
dc-start\n\
dc-HN\n\
LN-dc\n\
HN-end\n\
kj-sa\n\
kj-HN\n\
kj-dc";
        let puzzle = Puzzle12::create(&input);
        assert_eq!(puzzle.solve_part_2(), "103");
    }

    #[test]
    fn test_part_2_example_3() {
        let input = "fs-end\n\
he-DX\n\
fs-he\n\
start-DX\n\
pj-DX\n\
end-zg\n\
zg-sl\n\
zg-pj\n\
pj-he\n\
RW-he\n\
fs-DX\n\
pj-RW\n\
zg-RW\n\
start-pj\n\
he-WI\n\
zg-he\n\
pj-fs\n\
start-RW";
        let puzzle = Puzzle12::create(&input);
        assert_eq!(puzzle.solve_part_2(), "3509");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/12")).unwrap();
        let puzzle = Puzzle12::create(&input);
        assert_eq!(puzzle.solve_part_2(), "114189");
    }
}

use crate::puzzle::AbstractPuzzle;
use std::collections::{HashMap, HashSet};

pub struct Puzzle12 {
    start: usize,
    end: usize,
    size: usize,
    small_caves: Vec<bool>,
    edges: Vec<Vec<bool>>,
}

impl AbstractPuzzle for Puzzle12 {
    fn get_day(&self) -> u8 {
        12
    }

    fn solve_part_1(&self) -> String {
        self.count_paths(&mut vec![0; self.size], self.start, true)
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.count_paths(&mut vec![0; self.size], self.start, false)
            .to_string()
    }
}

impl Puzzle12 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let mut string_edges = HashMap::new();
        for line in input.lines() {
            let mut parts = line.split('-');
            let from = parts.next().unwrap();
            let to = parts.next().unwrap();
            string_edges
                .entry(from.to_string())
                .or_insert_with(HashSet::new)
                .insert(to.to_string());
            string_edges
                .entry(to.to_string())
                .or_insert_with(HashSet::new)
                .insert(from.to_string());
        }
        let mut caves = string_edges.keys().cloned().collect::<Vec<String>>();
        caves.sort();
        let size = caves.len();
        let mut edges = vec![vec![false; size]; size];
        for i in 0..size {
            for j in 0..size {
                if string_edges.get(&caves[i]).unwrap().contains(&caves[j]) {
                    edges[i][j] = true;
                }
            }
        }
        let start = caves.binary_search(&"start".to_string()).unwrap();
        let end = caves.binary_search(&"end".to_string()).unwrap();
        let mut small_caves = vec![false; size];
        for i in 0..size {
            if caves[i].chars().next().unwrap() >= 'a' {
                small_caves[i] = true;
            }
        }
        Box::new(Puzzle12 {
            start,
            end,
            size,
            small_caves,
            edges,
        })
    }

    fn count_paths(&self, path: &mut Vec<i8>, cave: usize, mut seen_twice: bool) -> u32 {
        if cave == self.end {
            return 1;
        }
        if self.small_caves[cave] && path[cave] > 0 {
            if seen_twice || cave == self.start {
                return 0;
            }
            seen_twice = true;
        }
        path[cave] += 1;
        let mut count = 0;
        for next in 0..self.size {
            if self.edges[cave][next] {
                count += self.count_paths(path, next, seen_twice);
            }
        }
        path[cave] -= 1;
        count
    }
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

use crate::puzzle::AbstractPuzzle;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct Puzzle05 {
    segments: Vec<LineSegment>,
}

impl AbstractPuzzle for Puzzle05 {
    fn get_day(&self) -> u8 {
        5
    }

    fn solve_part_1(&self) -> String {
        let mut points: HashMap<(i32, i32), usize> = HashMap::new();
        for segment in &self.segments {
            if segment.is_vertical_or_horizontal() {
                for point in segment.points() {
                    points.insert(point, points.get(&point).unwrap_or(&0) + 1);
                }
            }
        }
        let count = points.values().filter(|&x| *x >= 2).count();
        count.to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut points: HashMap<(i32, i32), usize> = HashMap::new();
        for segment in &self.segments {
            for point in segment.points() {
                points.insert(point, points.get(&point).unwrap_or(&0) + 1);
            }
        }
        let count = points.values().filter(|&x| *x >= 2).count();
        count.to_string()
    }
}

impl Puzzle05 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle05 {
            segments: input
                .lines()
                .map(LineSegment::parse)
                .collect::<Vec<LineSegment>>(),
        })
    }
}

struct LineSegment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl LineSegment {
    fn parse(line: &str) -> LineSegment {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        }
        let caps = RE.captures(line).unwrap();
        LineSegment {
            x1: caps[1].parse().unwrap(),
            y1: caps[2].parse().unwrap(),
            x2: caps[3].parse().unwrap(),
            y2: caps[4].parse().unwrap(),
        }
    }

    fn is_vertical_or_horizontal(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn points(&self) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        let dx = (self.x2 - self.x1).signum();
        let dy = (self.y2 - self.y1).signum();
        let mut x = self.x1;
        let mut y = self.y1;
        while x != self.x2 + dx || y != self.y2 + dy {
            points.push((x, y));
            x += dx;
            y += dy;
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle05::Puzzle05;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "0,9 -> 5,9\n\
8,0 -> 0,8\n\
9,4 -> 3,4\n\
2,2 -> 2,1\n\
7,0 -> 7,4\n\
6,4 -> 2,0\n\
0,9 -> 2,9\n\
3,4 -> 1,4\n\
0,0 -> 8,8\n\
5,5 -> 8,2";
        let puzzle = Puzzle05::create(&input);
        assert_eq!(puzzle.solve_part_1(), "5");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/5")).unwrap();
        let puzzle = Puzzle05::create(&input);
        assert_eq!(puzzle.solve_part_1(), "6113");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "0,9 -> 5,9\n\
8,0 -> 0,8\n\
9,4 -> 3,4\n\
2,2 -> 2,1\n\
7,0 -> 7,4\n\
6,4 -> 2,0\n\
0,9 -> 2,9\n\
3,4 -> 1,4\n\
0,0 -> 8,8\n\
5,5 -> 8,2";
        let puzzle = Puzzle05::create(&input);
        assert_eq!(puzzle.solve_part_2(), "12");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/5")).unwrap();
        let puzzle = Puzzle05::create(&input);
        assert_eq!(puzzle.solve_part_2(), "20373");
    }
}

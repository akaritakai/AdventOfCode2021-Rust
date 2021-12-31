use crate::puzzle::AbstractPuzzle;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;

pub struct Puzzle17 {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    min_dx: i32,
    max_dx: i32,
    min_dy: i32,
    max_dy: i32,
    max_steps: u32,
}

impl AbstractPuzzle for Puzzle17 {
    fn get_day(&self) -> u8 {
        17
    }

    fn solve_part_1(&self) -> String {
        let mut max_height = i32::MIN;
        for dx in self.min_dx..self.max_dx {
            for dy in self.min_dy..self.max_dy {
                max_height = max(max_height, self.find_max_height(dx, dy));
            }
        }
        max_height.to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut count = 0;
        for dx in self.min_dx..self.max_dx {
            for dy in self.min_dy..self.max_dy {
                if self.is_valid_vector(dx, dy) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }
}

impl Puzzle17 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
        }
        let caps = RE.captures(input).unwrap();
        let x1 = caps[1].parse::<i32>().unwrap();
        let x2 = caps[2].parse::<i32>().unwrap();
        let y1 = caps[3].parse::<i32>().unwrap();
        let y2 = caps[4].parse::<i32>().unwrap();
        let min_dx = if x1 < 0 {
            x1 - 1
        } else {
            (0.5 * (((8 * x1.abs() + 1) as f64).sqrt() - 1.0)).ceil() as i32
        };
        let max_dx = if x1 < 0 {
            (0.5 * (((8 * x2.abs() + 1) as f64).sqrt() - 1.0)).ceil() as i32 + 1
        } else {
            x2 + 2
        };
        let min_dy = if y1 < 0 {
            y1 - 1
        } else {
            (0.5 * (((8 * y1.abs() + 1) as f64).sqrt() - 1.0)).ceil() as i32
        };
        let max_dy = if y1 < 0 { (y1 - 1).abs() + 1 } else { y2 + 2 };
        let max_x_steps = min_dx.abs();
        let max_y_steps = if y1 < 0 {
            2 * (max_dy.abs() + 1)
        } else {
            max_dy.abs() + 1
        };
        let max_steps = max(max_x_steps, max_y_steps) as u32;
        Box::new(Puzzle17 {
            x1,
            y1,
            x2,
            y2,
            min_dx,
            max_dx,
            min_dy,
            max_dy,
            max_steps,
        })
    }

    fn find_max_height(&self, mut dx: i32, mut dy: i32) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut max_height = i32::MIN;
        let mut hit_target = false;
        for _ in 0..self.max_steps {
            x += dx;
            y += dy;
            dx = max(0, dx - 1);
            dy -= 1;
            max_height = max(y, max_height);
            if x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2 {
                hit_target = true;
                break;
            }
        }
        if hit_target {
            max_height
        } else {
            i32::MIN
        }
    }

    fn is_valid_vector(&self, mut dx: i32, mut dy: i32) -> bool {
        let mut x = 0;
        let mut y = 0;
        for _ in 0..self.max_steps {
            x += dx;
            y += dy;
            dx = max(0, dx - 1);
            dy -= 1;
            if x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle17::Puzzle17;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "target area: x=20..30, y=-10..-5";
        let puzzle = Puzzle17::create(&input);
        assert_eq!(puzzle.solve_part_1(), "45");
    }

    #[test]
    fn test_part_1_example_2() {
        // Example where maximum height won't be reached by y * (|y| - 1) / 2 due to x restrictions
        let input = "target area: x=22..27, y=-10..-5";
        let puzzle = Puzzle17::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/17")).unwrap();
        let puzzle = Puzzle17::create(&input);
        assert_eq!(puzzle.solve_part_1(), "7626");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "target area: x=20..30, y=-10..-5";
        let puzzle = Puzzle17::create(&input);
        assert_eq!(puzzle.solve_part_2(), "112");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/17")).unwrap();
        let puzzle = Puzzle17::create(&input);
        assert_eq!(puzzle.solve_part_2(), "2032");
    }
}

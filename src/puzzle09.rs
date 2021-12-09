use std::collections::{BinaryHeap, HashSet};
use crate::puzzle::AbstractPuzzle;

pub struct Puzzle09 {
    grid: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl AbstractPuzzle for Puzzle09 {
    fn get_day(&self) -> u8 {
        9
    }

    fn solve_part_1(&self) -> String {
        self.low_points().iter().map(|&(x, y)| self.grid[y][x] + 1).sum::<u32>().to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut basin_sizes = BinaryHeap::new();
        let mut seen = HashSet::new();
        for point in self.low_points() {
            let mut size = 0;
            let mut queue = vec![point];
            while let Some((x, y)) = queue.pop() {
                if seen.contains(&(x, y)) {
                    continue;
                }
                seen.insert((x, y));
                size += 1;
                for (x, y) in self.adjacent_rising(x, y) {
                    queue.push((x, y));
                }
            }
            basin_sizes.push(size);
        }
        // Take the largest 3 values of basin_sizes and multiply them together
        basin_sizes.iter().take(3).product::<u32>().to_string()
    }
}

impl Puzzle09 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let grid = input.lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        let height = grid.len();
        let width = grid[0].len();
        Box::new(Puzzle09 {
            grid,
            height,
            width,
        })
    }

    fn low_points(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.adjacent(x, y).iter().all(|&(i, j)| self.grid[j][i] > self.grid[y][x]) {
                    points.push((x, y));
                }
            }
        }
        points
    }

    fn adjacent_rising(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut adjacent = self.adjacent(x, y);
        adjacent.retain(|&(i, j)| self.grid[j][i] > self.grid[y][x]);
        adjacent.retain(|&(i, j)| self.grid[j][i] != 9);
        adjacent
    }

    fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();
        if x > 0 {
            adjacent.push((x - 1, y));
        }
        if x < self.width - 1 {
            adjacent.push((x + 1, y));
        }
        if y > 0 {
            adjacent.push((x, y - 1));
        }
        if y < self.height - 1 {
            adjacent.push((x, y + 1));
        }
        adjacent
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle09::Puzzle09;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "2199943210\n\
3987894921\n\
9856789892\n\
8767896789\n\
9899965678";
        let puzzle = Puzzle09::create(&input);
        assert_eq!(puzzle.solve_part_1(), "15");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/9")).unwrap();
        let puzzle = Puzzle09::create(&input);
        assert_eq!(puzzle.solve_part_1(), "550");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "2199943210\n\
3987894921\n\
9856789892\n\
8767896789\n\
9899965678";
        let puzzle = Puzzle09::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1134");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/9")).unwrap();
        let puzzle = Puzzle09::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1100682");
    }
}

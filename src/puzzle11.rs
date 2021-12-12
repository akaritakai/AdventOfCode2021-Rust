use crate::puzzle::AbstractPuzzle;

pub struct Puzzle11 {
    input: String,
    height: usize,
    width: usize,
}

impl AbstractPuzzle for Puzzle11 {
    fn get_day(&self) -> u8 {
        11
    }

    fn solve_part_1(&self) -> String {
        let mut grid = self.input_to_grid();
        let mut flashes = 0;
        for _ in 0..100 {
            flashes += self.do_step(&mut grid);
        }
        flashes.to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut grid = self.input_to_grid();
        let mut step = 1;
        loop {
            let count = self.do_step(&mut grid);
            if count == self.height * self.width {
                return step.to_string();
            }
            step += 1;
        }
    }
}

impl Puzzle11 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        Box::new(Puzzle11 {
            input: input.to_string(),
            height,
            width,
        })
    }

    fn do_step(&self, grid: &mut Vec<Vec<u8>>) -> usize {
        let mut num_flashed = 0;
        for row in grid.iter_mut().take(self.height) {
            for x in row.iter_mut().take(self.width) {
                *x += 1;
            }
        }
        let mut flashed = vec![vec![false; self.width]; self.height];
        let mut any_flashed = true;
        while any_flashed {
            any_flashed = false;
            for y in 0..self.height {
                for x in 0..self.width {
                    if grid[y][x] > 9 && !flashed[y][x] {
                        flashed[y][x] = true;
                        any_flashed = true;
                        num_flashed += 1;
                        for point in self.adjacent(x, y) {
                            grid[point.1][point.0] += 1;
                        }
                    }
                }
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                if flashed[y][x] {
                    grid[y][x] = 0;
                }
            }
        }
        num_flashed
    }

    fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();
        if x > 0 && y > 0 {
            adjacent.push((x - 1, y - 1));
        }
        if y > 0 {
            adjacent.push((x, y - 1));
        }
        if x < self.width - 1 && y > 0 {
            adjacent.push((x + 1, y - 1));
        }
        if x > 0 {
            adjacent.push((x - 1, y));
        }
        if x < self.width - 1 {
            adjacent.push((x + 1, y));
        }
        if x > 0 && y < self.height - 1 {
            adjacent.push((x - 1, y + 1));
        }
        if y < self.height - 1 {
            adjacent.push((x, y + 1));
        }
        if x < self.width - 1 && y < self.height - 1 {
            adjacent.push((x + 1, y + 1));
        }
        adjacent
    }

    fn input_to_grid(&self) -> Vec<Vec<u8>> {
        let mut grid = Vec::new();
        for line in self.input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as u8);
            }
            grid.push(row);
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle11::Puzzle11;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "5483143223\n\
2745854711\n\
5264556173\n\
6141336146\n\
6357385478\n\
4167524645\n\
2176841721\n\
6882881134\n\
4846848554\n\
5283751526";
        let puzzle = Puzzle11::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1656");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/11")).unwrap();
        let puzzle = Puzzle11::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1634");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "5483143223\n\
2745854711\n\
5264556173\n\
6141336146\n\
6357385478\n\
4167524645\n\
2176841721\n\
6882881134\n\
4846848554\n\
5283751526";
        let puzzle = Puzzle11::create(&input);
        assert_eq!(puzzle.solve_part_2(), "195");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/11")).unwrap();
        let puzzle = Puzzle11::create(&input);
        assert_eq!(puzzle.solve_part_2(), "210");
    }
}

use crate::puzzle::AbstractPuzzle;
use std::collections::BinaryHeap;

pub struct Puzzle15 {
    maze: Vec<Vec<i32>>,
}

impl AbstractPuzzle for Puzzle15 {
    fn get_day(&self) -> u8 {
        15
    }

    fn solve_part_1(&self) -> String {
        minimum_risk(&self.maze).to_string()
    }

    fn solve_part_2(&self) -> String {
        minimum_risk(&self.expanded_maze()).to_string()
    }
}

impl Puzzle15 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let maze = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        Box::new(Puzzle15 { maze })
    }

    fn expanded_maze(&self) -> Vec<Vec<i32>> {
        let height = self.maze.len();
        let width = self.maze[0].len();
        (0..(5 * self.maze.len()))
            .map(|y| {
                (0..(5 * self.maze[0].len()))
                    .map(|x| {
                        (self.maze[y % height][x % width]
                            + (x / width) as i32
                            + (y / height) as i32
                            - 1)
                            % 9
                            + 1
                    })
                    .collect::<Vec<i32>>()
            })
            .collect()
    }
}

fn minimum_risk(maze: &[Vec<i32>]) -> i32 {
    let height = maze.len();
    let width = maze[0].len();
    let mut dist = vec![vec![(10 * height * width) as i32; width]; height];
    let mut queue = BinaryHeap::new();
    queue.push((0, 0, 0));
    while let Some((cost, x, y)) = queue.pop() {
        if x == width - 1 && y == height - 1 {
            return -cost;
        }
        if dist[y][x] < -cost {
            continue;
        }
        for (i, j) in adjacent(x, y, maze) {
            let new_cost = -cost + maze[j][i];
            if new_cost < dist[j][i] {
                dist[j][i] = new_cost;
                queue.push((-new_cost, i, j));
            }
        }
    }
    unreachable!()
}

fn adjacent(x: usize, y: usize, maze: &[Vec<i32>]) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    if x > 0 {
        adjacent.push((x - 1, y));
    }
    if x < maze[0].len() - 1 {
        adjacent.push((x + 1, y));
    }
    if y > 0 {
        adjacent.push((x, y - 1));
    }
    if y < maze.len() - 1 {
        adjacent.push((x, y + 1));
    }
    adjacent
}

#[cfg(test)]
mod tests {
    use crate::puzzle15::Puzzle15;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "1163751742\n\
1381373672\n\
2136511328\n\
3694931569\n\
7463417111\n\
1319128137\n\
1359912421\n\
3125421639\n\
1293138521\n\
2311944581";
        let puzzle = Puzzle15::create(&input);
        assert_eq!(puzzle.solve_part_1(), "40");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/15")).unwrap();
        let puzzle = Puzzle15::create(&input);
        assert_eq!(puzzle.solve_part_1(), "458");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "1163751742\n\
1381373672\n\
2136511328\n\
3694931569\n\
7463417111\n\
1319128137\n\
1359912421\n\
3125421639\n\
1293138521\n\
2311944581";
        let puzzle = Puzzle15::create(&input);
        assert_eq!(puzzle.solve_part_2(), "315");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/15")).unwrap();
        let puzzle = Puzzle15::create(&input);
        assert_eq!(puzzle.solve_part_2(), "2800");
    }
}

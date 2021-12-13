use crate::letter_ocr::ocr_image;
use crate::puzzle::AbstractPuzzle;
use std::collections::HashSet;

pub struct Puzzle13 {
    points: HashSet<(usize, usize)>,
    instructions: Vec<(char, usize)>,
}

impl AbstractPuzzle for Puzzle13 {
    fn get_day(&self) -> u8 {
        13
    }

    fn solve_part_1(&self) -> String {
        let mut points = self.points.clone();
        let instruction = self.instructions[0];
        if instruction.0 == 'x' {
            fold_x(&mut points, instruction.1);
        } else {
            fold_y(&mut points, instruction.1);
        }
        points.len().to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut points = self.points.clone();
        for instruction in &self.instructions {
            if instruction.0 == 'x' {
                fold_x(&mut points, instruction.1);
            } else {
                fold_y(&mut points, instruction.1);
            }
        }
        let image = to_image(&points);
        ocr_image(&image)
    }
}

fn fold_x(grid: &mut HashSet<(usize, usize)>, location: usize) {
    let points = grid.iter().copied().collect::<Vec<(usize, usize)>>();
    for point in points {
        if point.0 > location {
            grid.remove(&point);
            grid.insert(((2 * location) - point.0, point.1));
        }
    }
}

fn fold_y(grid: &mut HashSet<(usize, usize)>, location: usize) {
    let points = grid.iter().copied().collect::<Vec<(usize, usize)>>();
    for point in points {
        if point.1 > location {
            grid.remove(&point);
            grid.insert((point.0, (2 * location) - point.1));
        }
    }
}

fn to_image(grid: &HashSet<(usize, usize)>) -> Vec<Vec<bool>> {
    let min_x = grid.iter().map(|(x, _)| x).min().unwrap();
    let max_x = grid.iter().map(|(x, _)| x).max().unwrap();
    let min_y = grid.iter().map(|(_, y)| y).min().unwrap();
    let max_y = grid.iter().map(|(_, y)| y).max().unwrap();
    let mut image = vec![vec![false; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for point in grid {
        image[(point.1 - min_y) as usize][(point.0 - min_x) as usize] = true;
    }
    image
}

impl Puzzle13 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        let parts = input.split("\n\n").collect::<Vec<&str>>();
        let points = parts[0]
            .lines()
            .map(|line| {
                let parts = line.split(',').collect::<Vec<&str>>();
                (
                    parts[0].parse::<usize>().unwrap(),
                    parts[1].parse::<usize>().unwrap(),
                )
            })
            .collect::<HashSet<(usize, usize)>>();
        let instructions = parts[1]
            .lines()
            .map(|line| {
                let line = line.replace("fold along ", "");
                let parts = line.split('=').collect::<Vec<&str>>();
                (
                    parts[0].chars().next().unwrap(),
                    parts[1].parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<(char, usize)>>();
        Box::new(Puzzle13 {
            points,
            instructions,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle13::Puzzle13;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "6,10\n\
0,14\n\
9,10\n\
0,3\n\
10,4\n\
4,11\n\
6,0\n\
6,12\n\
4,1\n\
0,13\n\
10,12\n\
3,4\n\
3,0\n\
8,4\n\
1,10\n\
2,14\n\
8,10\n\
9,0\n\
\n\
fold along y=7\n\
fold along x=5";
        let puzzle = Puzzle13::create(&input);
        assert_eq!(puzzle.solve_part_1(), "17");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/13")).unwrap();
        let puzzle = Puzzle13::create(&input);
        assert_eq!(puzzle.solve_part_1(), "655");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/13")).unwrap();
        let puzzle = Puzzle13::create(&input);
        assert_eq!(puzzle.solve_part_2(), "JPZCUAUR");
    }
}

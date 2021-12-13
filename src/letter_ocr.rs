use lazy_static::lazy_static;
use std::collections::HashMap;

const LETTER_WIDTH: usize = 4;
const LETTER_HEIGHT: usize = 6;

lazy_static! {
    static ref LETTERS: HashMap<char, Vec<Vec<bool>>> = vec![
        ('A', vec![
            vec![false, true, true, false],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, true, true, true],
            vec![true, false, false, true],
            vec![true, false, false, true]]),
        ('B', vec![
            vec![true, true, true, false],
            vec![true, false, false, true],
            vec![true, true, true, false],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, true, true, false]]),
        ('C', vec![
            vec![false, true, true, false],
            vec![true, false, false, true],
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![true, false, false, true],
            vec![false, true, true, false]]),
        // Letter D is unknown
        ('E', vec![
            vec![true, true, true, true],
            vec![true, false, false, false],
            vec![true, true, true, false],
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![true, true, true, true]]),
        ('F', vec![
            vec![true, true, true, true],
            vec![true, false, false, false],
            vec![true, true, true, false],
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![true, false, false, false]]),
        ('G', vec![
            vec![false, true, true, false],
            vec![true, false, false, true],
            vec![true, false, false, false],
            vec![true, false, true, true],
            vec![true, false, false, true],
            vec![false, true, true, true]]),
        ('H', vec![
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, true, true, true],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, false, false, true]
        ]),
        ('I', vec![
            vec![false, true, true, true],
            vec![false, false, true, false],
            vec![false, false, true, false],
            vec![false, false, true, false],
            vec![false, false, true, false],
            vec![false, true, true, true]
        ]),
        ('J', vec![
            vec![false, false, true, true],
            vec![false, false, false, true],
            vec![false, false, false, true],
            vec![false, false, false, true],
            vec![true, false, false, true],
            vec![false, true, true, false]
        ]),
        ('K', vec![
            vec![true, false, false, true],
            vec![true, false, true, false],
            vec![true, true, false, false],
            vec![true, false, true, false],
            vec![true, false, true, false],
            vec![true, false, false, true]
        ]),
        ('L', vec![
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![true, true, true, true]
        ]),
        // Letter M is unknown
        // Letter N is unknown
        ('O', vec![
            vec![false, true, true, false],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![false, true, true, false]]),
        ('P', vec![
            vec![true, true, true, false],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, true, true, false],
            vec![true, false, false, false],
            vec![true, false, false, false]]),
        // Letter Q is unknown
        ('R', vec![
            vec![true, true, true, false],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, true, true, false],
            vec![true, false, true, false],
            vec![true, false, false, true]]),
        ('S', vec![
            vec![false, true, true, true],
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![false, true, true, false],
            vec![false, false, false, true],
            vec![true, true, true, false]
        ]),
        // Letter T is unknown
        ('U', vec![
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![true, false, false, true],
            vec![false, true, true, false]
        ]),
        // Letter V is unknown
        // Letter W is unknown
        // Letter X is unknown
        ('Y', vec![
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![false, true, false, true],
            vec![false, false, true, false],
            vec![false, false, true, false],
            vec![false, false, true, false]]),
        ('Z', vec![
            vec![true, true, true, true],
            vec![false, false, false, true],
            vec![false, false, true, false],
            vec![false, true, false, false],
            vec![true, false, false, false],
            vec![true, true, true, true]])
    ]
    .into_iter()
    .collect();
}

fn ocr_letter(
    image: &[Vec<bool>],
    row_offset: usize,
    col_offset: usize,
) -> Result<char, &'static str> {
    for (letter, pattern) in LETTERS.iter() {
        let mut all_match = true;
        'outer: for (row, pattern_row) in pattern.iter().enumerate().take(LETTER_HEIGHT) {
            for (col, point) in pattern_row.iter().enumerate().take(LETTER_WIDTH) {
                let image_row = row + row_offset;
                let image_col = col + col_offset;
                if image_row >= image.len() || image_col >= image[image_row].len() {
                    return Err("Image is too small to contain a letter");
                }
                if image[image_row][image_col] != *point {
                    all_match = false;
                    break 'outer;
                }
            }
        }
        if all_match {
            return Ok(*letter);
        }
    }
    Err("No match for letter OCR")
}

pub fn ocr_image(image: &[Vec<bool>]) -> String {
    let mut success = true;
    let mut result = String::new();
    let mut col = 0;
    while col <= image[0].len() - LETTER_WIDTH {
        match ocr_letter(image, 0, col) {
            Ok(letter) => result.push(letter),
            Err(_) => {
                success = false;
                break;
            }
        }
        col += 5;
    }
    if success {
        return result;
    }
    let mut result = String::new();
    result.push('\n');
    for row in 0..image.len() {
        for col in 0..image[0].len() {
            if image[row][col] {
                result.push('▌');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }
    result.pop();
    result
}

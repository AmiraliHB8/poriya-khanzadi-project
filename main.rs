`rust
use rand::{thread_rng, Rng};

fn main() {
    // Define the list of words
    let words = ["CAT", "DOG", "DONKEY", "GOAT", "HORSE", "PIG", "SHEEP", "TURTLE"];

    // Set the dimensions of the puzzle grid
    let grid_size = (10, 10);

    // Initialize an empty grid
    let mut grid = vec![vec![String::from(" "); grid_size.1]; grid_size.0];

    // Function to check if a word can be placed in the grid
    fn can_place_word(word: &str, row: usize, col: usize, direction: &str) -> bool {
        if direction == "horizontal" {
            for i in 0..word.len() {
                if row + i >= grid_size.0 || grid[row][col + i] != " " {
                    return false;
                }
            }
        } else if direction == "vertical" {
            for i in 0..word.len() {
                if row + i >= grid_size.1 || grid[row + i][col] != " " {
                    return false;
                }
            }
        } else if direction == "diagonal_up_right" {
            for i in 0..word.len() {
                if row - i < 0  col + i >= grid_size.0  grid[row - i][col + i] != " " {
                    return false;
                }
            }
        } else if direction == "diagonal_down_right" {
            for i in 0..word.len() {
                if row + i >= grid_size.1  col + i >= grid_size.0  grid[row + i][col + i] != " " {
                    return false;
                }
            }
        } else if direction == "diagonal_up_left" {
            for i in 0..word.len() {
                if row - i < 0  col - i < 0  grid[row - i][col - i] != " " {
                    return false;
                }
            }
        } else if direction == "diagonal_down_left" {
            for i in 0..word.len() {
                if row + i >= grid_size.1  col - i < 0  grid[row + i][col - i] != " " {
                    return false;
                }
            }
        }
        true
    }

    // Function to place a word in the grid
    fn place_word(word: &str, row: usize, col: usize, direction: &str) {ss
        if direction == "horizontal" {
            for i in 0..word.len() {
                grid[row][col + i] = word[i..i + 1].to_string();
            }
        } else if direction == "vertical" {
            for i in 0..word.len() {
                grid[row + i][col] = word[i..i + 1].to_string();
            }
        } else if direction == "diagonal_up_right" {
            for i in 0..word.len() {
                grid[row - i][col + i] = word[i..i + 1].to_string();
            }
        } else if direction == "diagonal_down_right" {
            for i in 0..word.len() {
                grid[row + i][col + i] = word[i..i + 1].to_string();
            }
        } else if direction == "diagonal_up_left" {
            for i in 0..word.len() {
                grid[row - i][col - i] = word[i..i + 1].to_string();
            }
        } else if direction == "diagonal_down_left" {
            for i in 0..word.len() {
                grid[row + i][col - i] = word[i..i + 1].to_string();
            }
        }
    }
    let mut rng = thread_rng();
    for word in words {
        loop {
            let row = rng.gen_range(0, grid_size.0);
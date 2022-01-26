use std::fs;

const INPUT_PATH: &str = "input.txt";

#[derive(Clone, Copy, Debug)]
struct BingoCell {
    value: i32,
    called: bool,
}

trait BingoBoardExtensions {
    fn get_uncalled_numbers_total(&self) -> i32;
}

impl BingoBoardExtensions for BingoBoard {
    fn get_uncalled_numbers_total(&self) -> i32 {
        let mut uncalled_sum = 0;
        for row in self {
            for bingo_cell in row {
                if bingo_cell.called == false {
                    uncalled_sum += bingo_cell.value
                }
            }
        }
        uncalled_sum
    }
}

type BingoBoard = [[BingoCell; 5]; 5];

fn main() {
    let import_text = import_text_file(INPUT_PATH);
    let split_text = import_text.split("\n").collect::<Vec<&str>>();
    let instructions: Vec<i32> = split_text[0]
        .split(",")
        .map(|x| str::parse::<i32>(x).expect("Failed to parse integer from &str"))
        .collect();
    let bingo_board_data = split_text[1..split_text.len() - 1].to_vec();
    let mut bingo_boards = Vec::<BingoBoard>::new();
    let mut winning_bingo_boards = Vec::new();

    let mut row_counter = 0;

    // load bingo boards
    for element in bingo_board_data {
        if element == "" {
            bingo_boards.push(
                [[BingoCell {
                    value: 0,
                    called: false,
                }; 5]; 5],
            );
            row_counter = 0;
        } else {
            let length = bingo_boards.len();
            let parsed_numbers: Vec<i32> = element
                .split_whitespace()
                .map(|x| str::parse::<i32>(x).expect("Failed to parse integer from &str"))
                .collect();
            bingo_boards[length - 1][row_counter] = [
                BingoCell {
                    value: parsed_numbers[0],
                    called: false,
                },
                BingoCell {
                    value: parsed_numbers[1],
                    called: false,
                },
                BingoCell {
                    value: parsed_numbers[2],
                    called: false,
                },
                BingoCell {
                    value: parsed_numbers[3],
                    called: false,
                },
                BingoCell {
                    value: parsed_numbers[4],
                    called: false,
                },
            ];
            row_counter += 1;
        }
    }

    // loop through instructions and register which numbers have been called and check if an entire row or column has been checked within a single bingo card
    let mut bingo_board_index = 0;
    'name: for number in instructions {
        for bingo_board in &mut bingo_boards {
            let mut col_results = [0; 5];
            for row in &mut *bingo_board {
                let mut row_result = true;
                let mut cell_index = 0;
                for cell in row {
                    if cell.value == number {
                        cell.called = true
                    }
                    if cell.called == false {
                        row_result = false;
                    } else {
                        col_results[cell_index] += 1;
                    }
                    cell_index += 1;
                }
                if row_result == true {
                    // PART ONE
                    // println!("{number}");
                    // println!(
                    //     "PART 1: {} | from row",
                    //     number * bingo_board.get_uncalled_numbers_total()
                    // );
                    // break 'name;

                    // PART TWO
                    if !winning_bingo_boards.contains(&bingo_board_index) {
                        winning_bingo_boards.push(bingo_board_index);
                    }
                }
            }
            for col_result in col_results {
                if col_result == 5 {
                    // PART ONE
                    // println!(
                    //     "PART 1: {} | from col",
                    //     number * bingo_board.get_uncalled_numbers_total()
                    // );
                    // break 'name;

                    // PART TWO
                    if !winning_bingo_boards.contains(&bingo_board_index) {
                        winning_bingo_boards.push(bingo_board_index);
                    }
                }
            }
            if winning_bingo_boards.len() == 100 {
                let last_winning_index = winning_bingo_boards[winning_bingo_boards.len() - 1];

                println!("{number}");
                
                println!(
                    "{}",
                    bingo_boards[last_winning_index].get_uncalled_numbers_total() * number
                );

                break 'name;
            }
            bingo_board_index += 1;
        }
        bingo_board_index = 0;
    }
}

fn import_text_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to import text file")
}

// output value = sum of non-selected numbers * final called number (winning number for winning board)

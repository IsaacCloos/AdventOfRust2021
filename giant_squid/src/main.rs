use std::fs;

const INPUT_PATH: &str = "input.txt";

type BingoBoard = [[i32; 5]; 5];

fn main() {
    let import_text = import_text_file(INPUT_PATH);
    let split_text = import_text.split("\n").collect::<Vec<&str>>();
    let instructions: Vec<i32> = split_text[0]
                .split(",")
                .map(|x| str::parse::<i32>(x).expect("Failed to parse integer from &str"))
                .collect();
    let bingo_board_data = split_text[1..split_text.len() - 1].to_vec();
    let mut bingo_boards = Vec::<BingoBoard>::new();

    let mut row_counter = 0;

    // load bingo boards
    for element in bingo_board_data {
        if element == "" {
            bingo_boards.push([[0; 5]; 5]);
            row_counter = 0;
        } else {
            let length = bingo_boards.len();
            let parsed_numbers: Vec<i32> = element
                .split_whitespace()
                .map(|x| str::parse::<i32>(x).expect("Failed to parse integer from &str"))
                .collect();
            bingo_boards[length - 1][row_counter] = [parsed_numbers[0],parsed_numbers[1],parsed_numbers[2],parsed_numbers[3],parsed_numbers[4]];
            row_counter += 1;
        }
    }

    // testing checks out
    // println!("{:?}", bingo_boards[99]);
    // println!("{:?}", bingo_boards[99][4]);
    // println!("{:?}", bingo_boards[99][4][4]);
    // println!("{:?}", instructions);


    // loop through instructions and register which numbers have been called and check if an entire row or column has been checked within a single bingo card
    for number in instructions {
        
    }
}

fn import_text_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to import text file")
}

// output value = sum of non-selected numbers * final called number (winning number for winning board)

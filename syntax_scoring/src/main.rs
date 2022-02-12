use std::fs;

/*
    ): 3 points.
    ]: 57 points.
    }: 1197 points.
    >: 25137 points.
*/

const INPUT_PATH: &str = "input.txt";

fn main() {
    let navigation_lines = get_navigation_lines_from_input_file(INPUT_PATH);

    let mut scores = navigation_lines.iter().map(score_line_completion).collect::<Vec<_>>();

    scores.retain(|score| *score != 0);

    scores.sort();

    let middle_score = scores[(scores.len() / 2)];

    println!("{middle_score}")
}

// PART 2
fn score_line_completion(line: &String) -> i64 {
    let mut closure_queue = vec!['_'];

    for character in line.chars() {
        match character {
            '(' => closure_queue.push('('),
            '[' => closure_queue.push('['),
            '{' => closure_queue.push('{'),
            '<' => closure_queue.push('<'),
            ')' => {
                if closure_queue.remove(closure_queue.len() - 1) != '(' {
                    return 0
                }
            }
            ']' => {
                if closure_queue.remove(closure_queue.len() - 1) != '[' {
                    return 0
                }
            }
            '}' => {
                if closure_queue.remove(closure_queue.len() - 1) != '{' {
                    return 0
                }
            }
            '>' => {
                if closure_queue.remove(closure_queue.len() - 1) != '<' {
                    return 0
                }
            }
            _ => panic!("Invalid navigation character found"),
        }
    }

    let mut result: i64 = 0;

    /*
        ): 1 point.
        ]: 2 points.
        }: 3 points.
        >: 4 points.
    */

    // remove buffer
    closure_queue.remove(0);
    closure_queue.reverse();

    for open_closure in closure_queue {
        result *= 5;
        match open_closure {
            '(' => result += 1,
            '[' => result += 2,
            '{' => result += 3,
            '<' => result += 4,
            _ => panic!("Invalid navigation character found"),
        }
    }

    result
}

// PART 1
fn score_line_error(line: &String) -> i32 {
    let mut closure_queue = vec!['_'];

    for character in line.chars() {
        match character {
            '(' => closure_queue.push('('),
            '[' => closure_queue.push('['),
            '{' => closure_queue.push('{'),
            '<' => closure_queue.push('<'),
            ')' => {
                if closure_queue.remove(closure_queue.len() - 1) != '(' {
                    return 3
                }
            }
            ']' => {
                if closure_queue.remove(closure_queue.len() - 1) != '[' {
                    return 57
                }
            }
            '}' => {
                if closure_queue.remove(closure_queue.len() - 1) != '{' {
                    return 1197
                }
            }
            '>' => {
                if closure_queue.remove(closure_queue.len() - 1) != '<' {
                    return 25137
                }
            }
            _ => panic!("Invalid navigation character found"),
        }
    }

    0
}

fn get_navigation_lines_from_input_file(input_path: &str) -> Vec<String> {
    fs::read_to_string(input_path)
        .expect("Failed to parse provided path")
        .trim_end()
        .split_whitespace()
        .map(String::from)
        .collect::<_>()
}

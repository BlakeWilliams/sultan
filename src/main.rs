extern crate regex;
extern crate rustbox;

use regex::Regex;
use rustbox::{Color, RustBox};
use rustbox::Key;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let all_lines: Vec<String> = read_stdin(stdin);

    let mut matches: Vec<&String> = Vec::new();
    let mut input: String = String::new();

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    loop {
        matches.clear();
        find_matches(input.clone(), &all_lines, &mut matches);
        render_lines(&rustbox, input.clone(), &matches);

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Enter) => {
                        print!("{}", matches.first().unwrap());
                        break;
                    },
                    Some(Key::Esc) => { break; },
                    Some(Key::Backspace) => {
                        input.pop();
                    },
                    Some(Key::Char(character)) => {
                        input = format!("{}{}", input, character)
                    },
                    _ => { }
                }
            },

            Err(e) => panic!("{}", e),

            _ => { }
        }
    }
}

fn read_stdin(stdin: io::Stdin) -> Vec<String> {
    stdin.lock().lines().map(|line| {
        line.unwrap().trim().to_string()
    }).collect()
}

fn find_matches<'a>(search: String, base: &'a Vec<String>, matches: &mut Vec<&'a String>) {
    let regex_string: String = search.split("").fold(String::new(), |acc, input| {
        acc + ".*" + input
    });

    let re = Regex::new(&regex_string).unwrap();

    for line in base {
        if re.is_match(&line) {
            matches.push(&line)
        }
    }
}

fn render_lines(rustbox: &RustBox, search_string: String, matches: &Vec<&String>) {
    rustbox.clear();

    rustbox.print(1, 0, rustbox::RB_BOLD, Color::White, Color::Black, &search_string);

    let mut index = 1;
    for line in matches {
        rustbox.print(1, index, rustbox::RB_NORMAL, Color::White, Color::Black, &line);
        index += 1;
    }

    rustbox.present();
}

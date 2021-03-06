extern crate rustbox;
extern crate sultan;

use rustbox::{Color, RustBox};
use rustbox::Key;
use std::io;
use std::io::BufRead;

use sultan::matcher::{Matcher};

fn main() {
    let stdin = io::stdin();
    let all_lines: Vec<String> = read_stdin(stdin);

    let mut matcher = Matcher::new(&all_lines);

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    render_lines(&rustbox, &mut matcher);

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Enter) => {
                        print!("{}", matcher.selected_match());
                        break;
                    },
                    Some(Key::Esc) => { break; },
                    Some(Key::Backspace) => {
                        matcher.pop_char()
                    },
                    Some(Key::Down) => {
                        if matcher.selection != rustbox.height() {
                            matcher.move_down();
                        }
                    },
                    Some(Key::Up) => {
                        matcher.move_up();
                    },
                    Some(Key::Char(character)) => {
                        matcher.push_char(character);
                    },
                    _ => { }
                }
            },

            Err(e) => panic!("{}", e),

            _ => { }
        }

        render_lines(&rustbox, &mut matcher);
    }
}

fn read_stdin(stdin: io::Stdin) -> Vec<String> {
    stdin.lock().lines().map(|line| {
        line.unwrap().trim().to_string()
    }).collect()
}

fn render_lines(rustbox: &RustBox, matcher: &mut Matcher) {
    rustbox.clear();

    let search_box_text = format!("> {}", matcher.input);
    rustbox.print(0, 0, rustbox::RB_NORMAL, Color::White, Color::Black, &search_box_text);

    let counts = format!("{}/{}", matcher.matches.len(), matcher.lines.len());
    rustbox.print(0, 1, rustbox::RB_NORMAL, Color::Yellow, Color::Black, &counts);

    let mut index = 2;

    for line in matcher.matches.clone() {
        if index == matcher.selection + 2 {
            rustbox.print(0, index, rustbox::RB_BOLD, Color::Red, Color::Black, ">");
            rustbox.print(2, index, rustbox::RB_BOLD, Color::Red, Color::Black, &line);
        } else {
            rustbox.print(2, index, rustbox::RB_NORMAL, Color::White, Color::Black, &line);
        }
        index += 1;
    }

    rustbox.present();
}

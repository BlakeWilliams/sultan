extern crate regex;

use self::regex::Regex;

pub struct Matcher<'a> {
    pub lines: &'a Vec<String>,
    pub matches: Vec<&'a String>,
    pub input: String,
    pub selection: usize,
}

impl<'a> Matcher<'a> {
    pub fn new(lines: &'a Vec<String>) -> Matcher<'a> {
        let mut matches: Vec<&'a String> = Vec::new();

        for line in lines {
            matches.push(&line);
        }

        Matcher {
            lines: lines,
            input: String::new(),
            selection: 0,
            matches: matches,
        }
    }

    pub fn push_char(&mut self, input: char) {
        self.input = format!("{}{}", self.input, input);
        self.update_matches();
    }

    pub fn pop_char(&mut self) {
        self.input.pop();
        self.update_matches();
    }

    pub fn selected_match(&mut self) -> String {
        match self.matches.get(self.selection) {
            Some(match_text) => match_text.clone().to_string(),
            None => "".to_string(),
        }
    }

    pub fn move_up(&mut self) {
        if self.selection != 0 {
            self.selection -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selection != self.matches.len() {
            self.selection += 1;
        }
    }

    fn update_matches(&mut self) {
        self.matches.clear();

        let regex_string: String = self.input.split("").fold(String::new(), |acc, input| {
            acc + ".*" + input
        });

        let re = Regex::new(&regex_string).unwrap();

        for line in self.lines {
            if re.is_match(&line) {
                self.matches.push(&line);
            }
        }
    }
}

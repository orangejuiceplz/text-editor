use termion::color;

pub struct Highlighter;

impl Highlighter {
    pub fn highlight(&self, line: &str) -> String {
        let mut result = String::new();
        let mut in_string = false;

        for c in line.chars() {
            match c {
                '"' => {
                    in_string = !in_string;
                    if in_string {
                        result.push_str(&format!("{}", color::Fg(color::Green)));
                    } else {
                        result.push_str(&format!("{}", color::Fg(color::Reset)));
                    }
                    result.push(c);
                }
                _ if in_string => result.push(c),
                '0'..='9' => result.push_str(&format!("{}{}{}", color::Fg(color::Blue), c, color::Fg(color::Reset))),
                _ => result.push(c),
            }
        }

        result
    }
}
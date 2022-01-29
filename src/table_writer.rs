use std::cmp::{max, min};
use std::fmt::Write;
use std::io::Lines;

use crate::new_table::Width;

pub struct TableWriter<Target, const Columns: usize>
    where Target: Write {
    column_widths: [Width; Columns],
    target: Target,
}

impl<Target, const Columns: usize> TableWriter<Target, Columns>
    where Target: Write {
    pub fn print(self, rows: Vec<[String; Columns]>) {
        let mut widths = rows.iter()
            .fold([0; Columns], |mut acc, row| {
                for i in 0..Columns {
                    let string = &row[i];

                    acc[i] = match &self.column_widths[i] {
                        Width::Dynamic => max(acc[i], n_chars(string)),
                        Width::Max(limit) => min(max(acc[i], n_chars(string)), *limit)
                    };
                }
                acc
            });
    }

    fn row_to_lines(&self, row: [String; Columns], widths: [usize; Columns]) -> Vec<Line> {
        for (i, string) in row.into_iter().enumerate() {
            let s = Self::adapt_string_to_width(string, widths[i]);
        }

        vec![]
    }

    fn adapt_string_to_width(string: String, width: usize) -> String {
        if n_chars(&string) <= width { return string; }

        string.lines()
            .map(|line| Self::adapt_line_to_width(line, width))
            .fold(String::new(), |mut result, s| {
                result.push_str(&s);
                result.push('\n');
                result
            })
    }

    fn adapt_line_to_width(line: &str, width: usize) -> String {
        line.split(" ")
            .filter(|word| word_not_empty(word))
            .fold((String::new(), 0), |(mut result, mut current_line_len), word| {
                let word_len = n_chars(word);

                match word_len + current_line_len > width {
                    true if word_len > width => {
                        result.push('\n');
                        result.push_str(&word[0..(width - 3)]);
                        result.push_str("...");
                        current_line_len = width
                    }
                    true => {
                        result.push('\n');
                        result.push_str(word);
                        current_line_len = word_len
                    }
                    false if word_len + current_line_len + 1 > width => {
                        result.push('\n');
                        result.push_str(word);
                        current_line_len = word_len
                    }
                    false => {
                        if current_line_len > 0 {
                            result.push(' ');
                            current_line_len += 1;
                        }

                        result.push_str(word);
                        current_line_len += word_len
                    }
                }

                (result, current_line_len)
            }).0
    }
}

enum Line {
    Full,
    Empty,
    Content,
}

fn n_chars(string: &str) -> usize {
    string.chars().count()
}

fn word_not_empty(word: &str) -> bool {
    word.chars().map(|c| !c.is_whitespace()).min().unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::table_writer::TableWriter;

    #[test]
    fn convert_works() {
        let string = "Das Haus vom Tobias ist gigantisch und warm die da die".to_string();

        let new = TableWriter::<String, 4>::adapt_string_to_width(string, 7);
        println!("{new}");
    }
}
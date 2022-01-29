use std::cmp::{max, min};
use std::io::Write;

use crate::table::Width;

pub struct TableWriter<const C: usize> {
    column_widths: [Width; C],
}

impl<const C: usize> TableWriter<C> {
    pub fn new(column_widths: [Width; C]) -> Self {
        TableWriter { column_widths }
    }

    pub fn write<T: Write>(self, rows: Vec<[String; C]>, target: T) {
        let widths = rows.iter()
            .fold([0; C], |mut acc, row| {
                for i in 0..C {
                    let string = &row[i];

                    acc[i] = match &self.column_widths[i] {
                        Width::Dynamic => max(acc[i], n_chars(string)),
                        Width::Max(limit) => min(max(acc[i], n_chars(string)), *limit)
                    };
                }
                acc
            });

        let lines = Some(Line::Full).into_iter()
            .chain(rows.into_iter().flat_map(|row| self.row_to_lines(row, widths)))
            .collect();

        self.write_lines(lines, widths, target)
    }

    fn row_to_lines(&self, row: [String; C], widths: [usize; C]) -> Vec<Line<C>> {
        let row: [String; C] = row.into_iter()
            .enumerate()
            .map(|(i, string)| Self::adapt_string_to_width(string, widths[i]))
            .collect::<Vec<_>>()
            .try_into().unwrap();

        let mut row_lines = RowLines::new(row);
        let mut lines = vec![];
        lines.push(Line::Empty);

        while let Some(line) = row_lines.next_line() {
            lines.push(Line::Content(line))
        }
        lines.push(Line::Empty);
        lines.push(Line::Full);

        lines
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

    fn write_lines<T: Write>(mut self, lines: Vec<Line<C>>, widths: [usize; C], mut target: T) {
        lines.into_iter()
            .map(|line| self.write_line(line, widths) + "\n")
            .for_each(|line_string| { target.write(line_string.as_bytes()).unwrap(); })
    }

    fn write_line(&mut self, line: Line<C>, widths: [usize; C]) -> String {
        match line {
            Line::Full => self.write_full_line(widths),
            Line::Empty => self.write_empty_line(widths),
            Line::Content(content) => self.write_content_line(widths, content)
        }
    }

    fn write_full_line(&mut self, widths: [usize; C]) -> String {
        let mut string = "+".to_string();
        string.extend(widths.into_iter().map(|width| format!("{}+", char_n_times('-', width + 2))));
        string
    }

    fn write_empty_line(&mut self, widths: [usize; C]) -> String {
        let mut string = "|".to_string();
        string.extend(widths.into_iter().map(|width| format!("{}|", char_n_times(' ', width + 2))));
        string
    }

    fn write_content_line(&mut self, widths: [usize; C], strings: [String; C]) -> String {
        let mut string = "|".to_string();
        string.extend((0..C).into_iter()
            .map(|i| {
                let width = widths[i];
                let string = &strings[i];
                let whitespace_len = width - n_chars(string);
                format!(" {}{} |", string, char_n_times(' ', whitespace_len))
            })
        );
        string
    }
}

struct RowLines<const C: usize> {
    lines: [Vec<String>; C],
    current_line: usize,
}

impl<const C: usize> RowLines<C> {
    // maybe use MaybeUnit to avoid allocation
    fn new(row: [String; C]) -> Self {
        let lines = (0..C).into_iter()
            .map(|i| row[i].lines().map(str::to_string).collect())
            .collect::<Vec<_>>();

        RowLines {
            lines: lines.try_into().unwrap(),
            current_line: 0,
        }
    }

    fn next_line(&mut self) -> Option<[String; C]> {
        let (all_none, line) = (0..C).into_iter()
            .map(|i| self.lines[i].get(self.current_line))
            .fold((true, vec![]), |(mut all_none, mut line), string_opt| match string_opt {
                Some(string) => {
                    all_none = false;
                    line.push(string.clone());
                    (all_none, line)
                }
                None => {
                    line.push("".to_string());
                    (all_none, line)
                }
            });

        match all_none {
            true => None,
            false => {
                self.current_line += 1;
                Some(line.try_into().unwrap())
            }
        }
    }
}

#[derive(Debug)]
enum Line<const C: usize> {
    Full,
    Empty,
    Content([String; C]),
}

fn n_chars(string: &str) -> usize {
    string.chars().count()
}

fn word_not_empty(word: &str) -> bool {
    word.chars().map(|c| !c.is_whitespace()).min().unwrap_or(false)
}

fn char_n_times(c: char, len: usize) -> String {
    vec![c; len].into_iter().collect()
}
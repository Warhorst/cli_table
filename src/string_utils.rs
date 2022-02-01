/// Return a string which has the maximum given width. This means if
/// the distance between the start and a newline or two newlines
/// is bigger than width, a newline is added to separate the words
/// properly.
/// If a word is longer than width, its (0..width) substring is used.
/// If the string is already smaller than width, it is returned as is.
pub fn adapt_to_width(string: &str, width: usize) -> String {
    if num_chars(&string) <= width { return string.to_string(); }

    let num_lines = string.lines().count();

    string.lines()
        .map(|line| adapt_line_to_width(line, width))
        .enumerate()
        .fold(String::new(), |result, (i, s)| result + &s + if i == num_lines - 1 { "" } else { "\n" })
}

fn adapt_line_to_width(line: &str, width: usize) -> String {
    line.split_whitespace()
        .fold((String::new(), 0), |(mut result, mut current_line_len), word| {
            let word_len = num_chars(word);

            match word_len + current_line_len > width {
                true if word_len > width => {
                    result.push_str(if current_line_len == 0 { "" } else { "\n" });
                    result.push_str(&word.chars().take(width).collect::<String>());
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

/// Return the number of chars the gives string consists of.
pub fn num_chars(string: &str) -> usize {
    string.chars().count()
}

/// Return if a string is empty or consist only of whitespace.
pub fn is_empty(string: &str) -> bool {
    string.len() == 0 || string.chars().map(|c| c.is_whitespace()).min().unwrap_or(true)
}

/// Return a string which consists of n times the given char.
pub fn char_n_times(c: char, n: usize) -> String {
    vec![c; n].into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::string_utils::{adapt_to_width, char_n_times, is_empty, num_chars};

    #[test]
    fn adapt_to_width_width_zero_works() {
        assert_eq!(adapt_to_width("foo", 0), "")
    }

    #[test]
    fn adapt_to_width_works() {
        vec![
            ("foo", "foo"),
            ("foo bar baz", "foo bar\nbaz"),
            ("foobarbaz", "foobarb"),
            ("foo barbazoof", "foo\nbarbazo")
        ].into_iter()
            .for_each(|(l, r)| assert_eq!(adapt_to_width(l, 7), r))
    }

    #[test]
    fn num_chars_works() {
        vec![
            "abcd",
            "äöüß",
            "abäö"
        ].into_iter().for_each(|s| assert_eq!(num_chars(s), 4))
    }

    #[test]
    fn is_empty_works() {
        vec![
            ("", true),
            ("foo", false),
            ("  foo  ", false),
            ("      ", true)
        ].into_iter().for_each(|(s, expected)| assert_eq!(is_empty(s), expected))
    }

    #[test]
    fn char_n_times_works() {
        vec![
            ('a', "aaaa"),
            (' ', "    "),
            ('ö', "öööö")
        ].into_iter().for_each(|(c, expected)| assert_eq!(char_n_times(c, 4), expected))
    }
}
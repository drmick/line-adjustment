extern crate core;

use std::cmp::{max, min};

pub fn transform(input: &str, line_width: u32) -> String {
    let line_width = line_width as usize;

    // Split input string by whitespace and simple formatting
    let words = input.split_whitespace().collect::<Vec<&str>>();
    let mut result_string = String::from("");
    let mut line_words_length = 0;
    let mut line_words = 0;
    let mut line_start_index = 0;

    for (current_word_index, word) in words.iter().enumerate() {
        // Get next word length
        let next_word_length = match words.get(current_word_index + 1) {
            Some(el) => el.len(),
            _ => 0,
        };

        line_words_length += word.len();

        // Calc current line length with space between words
        let current_line_size = line_words_length + line_words;
        line_words += 1;

        // If current line has space for current word
        // and doesn't have space for next word
        // or word is very wide
        if (current_line_size <= line_width
            && (current_line_size + next_word_length + 1 > line_width
                || next_word_length == 0))
            || word.len() > line_width
        {
            // Select words for new line
            let line = &words[line_start_index..line_start_index + line_words];

            // Transform slice to string and add spaces
            let line = transform_line(line, line_words_length, line_width);
            result_string.push_str(&line);

            // Add new line symbol if line is not the last
            if current_word_index != words.len() - 1 {
                result_string.push('\n');
            }

            // Clear counters
            line_words = 0;
            line_start_index = current_word_index + 1;
            line_words_length = 0;
        }
    }
    result_string
}

fn transform_line(
    line: &[&str],
    line_words_length: usize,
    line_width: usize,
) -> String {
    // Calc number of intervals between words (min = 1)
    let line_words_except_one = max(line.len() - 1, 1);
    let mut result = String::from("");

    // Calc the empty space in the line in which we will place spaces (considering very big words)
    let free_space_in_line = line_width - min(line_words_length, line_width);

    // Calc spaces for every interval
    let spaces = free_space_in_line / line_words_except_one;

    // Calc number of intervals which has a extra space (from start of line)
    let extra_spaces_amount = free_space_in_line % line_words_except_one;

    // Making result line string with spaces
    for (i, word) in line.iter().enumerate() {
        // Push word
        result.push_str(word);

        // If Only one word or word is not the last then push spaces after that word
        if line.len() != i + 1 || line.len() == 1 {
            result.push_str(" ".repeat(spaces).as_str());
        }

        // If The interval should have a extra space then add it
        if i < extra_spaces_amount {
            result.push(' ');
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      ")
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), expected);
        }
    }
}

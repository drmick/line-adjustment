extern crate core;

use std::cmp::max;

pub fn transform(input: &str, line_width: u32) -> String {
    const DELIMITER: &str = " ";
    const NEW_LINE: &str = "\n";
    let line_width = line_width as usize;

    // Split input string by whitespace and simple formatting
    let slices = input.split_whitespace().collect::<Vec<&str>>();
    let mut result_string = String::from("");
    let mut row_words_length = 0;
    let mut row_words = 0;
    let mut row_start_index = 0;

    for (current_word_index, word) in slices.iter().enumerate() {
        // Get next word length
        let next_word_length = match slices.get(current_word_index + 1) {
            Some(el) => el.len(),
            _ => 0,
        };

        row_words_length += word.len();

        // Calc current row length with space between words
        let current_row_size = row_words_length + row_words;
        row_words += 1;

        // If current row has space for current word
        // and doesn't have space for next word
        // or word is very wide
        if (current_row_size <= line_width
            && (current_row_size + next_word_length + 1 > line_width
                || next_word_length == 0))
            || word.len() > line_width
        {
            // Select words for new row
            let row = &slices[row_start_index..row_start_index + row_words];

            // Calc number of intervals between words (min = 1)
            let row_words_except_one = max(row_words - 1, 1);

            // Calc spaces for every interval
            let spaces = (line_width - row_words_length) / row_words_except_one;

            // Calc number of intervals which has a extra space (from start of row)
            let extra_spaces_amount =
                (line_width - row_words_length) % row_words_except_one;

            // Making result row string with spaces
            for (j, word) in row.iter().enumerate() {
                // Push word
                result_string.push_str(word);

                // If Only one word or word is not the last then push spaces after that word
                if row.len() != j + 1 || row.len() == 1 {
                    result_string.push_str(DELIMITER.repeat(spaces).as_str());
                }

                // If The interval should have a extra space then add it
                if j < extra_spaces_amount {
                    result_string.push_str(DELIMITER);
                }
            }

            // Add new line symbol if row is not the last
            if current_word_index != slices.len() - 1 {
                result_string.push_str(NEW_LINE);
            }

            // Clear counters
            row_words = 0;
            row_start_index = current_word_index + 1;
            row_words_length = 0;
        }
    }
    result_string
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

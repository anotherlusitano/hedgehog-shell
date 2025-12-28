/// This function iterates through all the characters of the input.
/// It creates words separated by whitespace.
/// If characters are inside quotes (' or "), it will join them together.
/// At the end, it returns a Vector of those words.
pub fn parse_input(input: &str) -> Vec<String> {
    let mut tokens = vec![];

    // Will allow us to join characters inside quotes
    let mut is_to_join = false;

    // These are the characters that form the command
    let mut word: Vec<char> = vec![];

    // We need to check if the next character is a whitespace
    // which is why it needs to be peekable
    let mut iter = input.chars().peekable();

    // Will allow us to have words with quotes
    let mut current_quote = ' ';

    // Eliminate all whitespaces before the first character
    while iter.next_if(|&c| c == ' ').is_some() {}

    while let Some(c) = iter.next() {
        let next_char_is_whitespace = iter.peek() == Some(&' ');
        let has_next = iter.peek().is_some();
        let is_backslash = c == '\\';

        if is_backslash && has_next {
            let next_char = iter.next().unwrap();

            if next_char.is_whitespace() {
                // Will push the word if exists
                if !word.is_empty() {
                    let final_word = word.clone().into_iter().collect();
                    tokens.push(final_word);
                    word.clear();
                }

                // and will push the character after the backslash
                tokens.push(next_char.to_string());
            } else {
                // just push the char if there is no whitespace
                word.push(next_char);
            }

            // Push the word if there are no remaining characters
            if iter.peek().is_none() {
                let final_word = word.clone().into_iter().collect();
                tokens.push(final_word);
                break;
            }
            continue;
        }

        // If it contains more than one whitespace, just ignore it
        if !is_to_join && c == ' ' && next_char_is_whitespace {
            continue;
        }

        if c == '\'' || c == '"' {
            // Will begin/end joining if the same quote is detected
            if current_quote == ' ' || current_quote == c {
                is_to_join = !is_to_join;
                current_quote = c;
            }

            // If the last character is a quote, push the word
            if !has_next && current_quote == c {
                let final_word = word.clone().into_iter().collect();
                tokens.push(final_word);
            }

            // Don't include the current quote on the word
            if current_quote == c {
                continue;
            }
        }

        // Keep adding characters until its not to join
        if is_to_join {
            word.push(c);

            // TODO: maybe add a new prompt until the user insert the current_quote?
            if !has_next {
                todo!("The user didn't close the current quote...")
            }
            continue;
        }

        // Add the whitespace and the word as separate elements
        if c == ' ' && has_next {
            // Ignore whitespace if no characters are available to form a word
            if word.is_empty() {
                continue;
            }
            let final_word = word.clone().into_iter().collect();
            tokens.push(final_word);

            word.clear();

            continue;
        }

        // If its the last character, will push the word
        if !has_next {
            if c != ' ' {
                word.push(c);
            }
            let final_word = word.clone().into_iter().collect();
            tokens.push(final_word);
        }

        word.push(c);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use crate::parse_input::parse_input;

    #[test]
    fn test_parse_input() {
        let input = "  echo  ".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["echo".to_string()];

        assert_eq!(tokens, expected_output);

        let input = "exit".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["exit".to_string()];

        assert_eq!(tokens, expected_output);

        let input = "echo echo echo".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["echo".to_string(), "echo".to_string(), "echo".to_string()];

        assert_eq!(tokens, expected_output);

        let input = "'hello    world :)'".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["hello    world :)".to_string()];

        assert_eq!(tokens, expected_output);

        let input = "echo 'hello    world :)'".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["echo".to_string(), "hello    world :)".to_string()];

        assert_eq!(tokens, expected_output);

        let input = "echo \"hello    world :)\"".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["echo".to_string(), "hello    world :)".to_string()];

        assert_eq!(tokens, expected_output);

        let input = "''".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["".to_string()];

        assert_eq!(tokens, expected_output);

        let input = "\"\"".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["".to_string()];

        assert_eq!(tokens, expected_output);

        let input = "echo 'hello''world'".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["echo".to_string(), "helloworld".to_string()];

        assert_eq!(tokens, expected_output);

        let input = "echo hello''world".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["echo".to_string(), "helloworld".to_string()];

        assert_eq!(tokens, expected_output);

        let input = "'  hello  world  ' a".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["  hello  world  ".to_string(), "a".to_string()];

        assert_eq!(tokens, expected_output);

        let input = "echo 'hello    \"   \"   world'".to_string();
        let tokens: Vec<String> = parse_input(&input);
        let expected_output = vec!["echo".to_string(), "hello    \"   \"   world".to_string()];

        assert_eq!(tokens, expected_output);

        let input = r"echo three\ \ \ spaces";
        let tokens: Vec<String> = parse_input(input);
        let expected_output = vec![
            "echo".to_string(),
            "three".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            "spaces".to_string(),
        ];

        assert_eq!(tokens, expected_output);

        let input = r"echo before\     after";
        let tokens: Vec<String> = parse_input(input);
        let expected_output = vec![
            "echo".to_string(),
            "before".to_string(),
            " ".to_string(),
            "after".to_string(),
        ];

        assert_eq!(tokens, expected_output);

        let input = r"echo test\nexample";
        let tokens: Vec<String> = parse_input(input);
        let expected_output = vec!["echo".to_string(), "testnexample".to_string()];

        assert_eq!(tokens, expected_output);

        let input = r"echo hello\\world";
        let tokens: Vec<String> = parse_input(input);
        let expected_output = vec!["echo".to_string(), r"hello\world".to_string()];

        assert_eq!(tokens, expected_output);

        let input = r"echo \'hello\'";
        let tokens: Vec<String> = parse_input(input);
        let expected_output = vec!["echo".to_string(), r"'hello'".to_string()];

        assert_eq!(tokens, expected_output);
    }
}

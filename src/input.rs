//! Handles user communication.

use std::io::{self, Write};

/// Defaults for an yes-or-no prompt.
pub enum DefaultBool {
    YES,
    NO,
    None,
}

/// Asks the user a yes or no question.
pub fn prompt_yes_no(prompt: &str, default: DefaultBool) -> Option<bool> {
    let y_n = match default {
        DefaultBool::YES => "(Y/n)",
        DefaultBool::NO => "(y/N)",
        DefaultBool::None => "(y/n)",
    };

    output_prompt(&format!("{} {} ", prompt, y_n));

    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer).expect("Could not read input.");

    let lower_input = buffer.trim().to_lowercase();

    match default {
        DefaultBool::YES => {
            if lower_input == "n" {
                Some(false)
            } else {
                Some(true)
            }
        }
        DefaultBool::NO => {
            if lower_input == "y" {
                Some(true)
            } else {
                Some(false)
            }
        }
        DefaultBool::None => {
            if lower_input == "y" {
                Some(true)
            } else if lower_input == "n" {
                Some(false)
            } else {
                None
            }
        }
    }
}

pub fn get_string(prompt: &str, default: Option<String>, allow_empty: Option<bool>) -> String {
    let allow_empty = if let Some(allow) = allow_empty { allow } else { true };

    let stdin = io::stdin();

    let mut buffer;

    'input_loop: loop {
        output_prompt(&format!("{} [{}] ", prompt, if let Some(text) = &default { &text } else { "" }));

        buffer = String::new();
        stdin.read_line(&mut buffer).expect("Could not read input.");

        buffer = buffer.trim().to_string();

        if buffer.len() == 0 {
            if let None = default {
                if allow_empty == false {
                    continue 'input_loop
                }
            }
        }

        break 'input_loop;
    }

    if buffer.len() == 0 {
        if let Some(text) = default {
            return text;
        }
    }

    buffer
}

fn output_prompt(prompt: &str) {
    print!("{}", prompt);

    io::stdout().flush().expect("Could not output a prompt.");
}

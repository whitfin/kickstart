use std::io::{self, Write, BufRead};

use toml;

use errors::Result;


/// Wait for user input and return what they typed
fn read_line() -> Result<String> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();
    lines
        .next()
        .and_then(|l| l.ok())
        .ok_or_else(|| "Unable to read from std".into())
}

/// Ask a yes/no question to the user
pub fn ask_bool(question: &str, default: bool) -> Result<bool> {
    print!("{} {}: ", question, if default { "[Y/n]" } else { "[y/N]" });
    let _ = io::stdout().flush();
    let input = read_line()?;

    let res = match &*input {
        "y" | "Y" | "yes" | "YES" | "true" => true,
        "n" | "N" | "no" | "NO" | "false" => false,
        "" => default,
        _ => {
            println!("Invalid choice: '{}'", input);
            ask_bool(question, default)?
        },
    };

    Ok(res)
}

/// Ask a question to the user where they can write any string
pub fn ask_string(question: &str, default: &str) -> Result<String> {
    print!("{} ({}): ", question, default);
    let _ = io::stdout().flush();
    let input = read_line()?;

    let res = match &*input {
        "" => default.to_string(),
        _ => input,
    };

    Ok(res)
}

/// Ask a question to the user where they can write any string
pub fn ask_choices(question: &str, default: usize, choices: &Vec<toml::Value>) -> Result<toml::Value> {
    println!("{}: ", question);
    let mut nums = vec![];
    for (index, choice) in choices.iter().enumerate() {
        println!("{} - {}", index + 1, choice.as_str().unwrap());
        nums.push(format!("{}", index + 1));
    }

    print!("Choose from {} ({}): ", nums.join(", "), default);

    let _ = io::stdout().flush();
    let input = read_line()?;


    let res = match &*input {
        "" => choices[default - 1].clone(),
        _ => {
            if let Ok(num) = input.parse::<usize>() {
                if num > choices.len() {
                    println!("Invalid choice: '{}'", input);
                    ask_choices(question, default, choices)?
                } else {
                    choices[num - 1].clone()
                }
            } else {
                println!("Invalid choice: '{}'", input);
                ask_choices(question, default, choices)?
            }
        },
    };

    Ok(res)
}
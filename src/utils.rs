use std::io::stdin;

fn uppercase_if_true(c: char, uppercase: bool) -> char {
    if uppercase {
        c.to_uppercase().next().unwrap_or(c)
    } else {
        c.to_lowercase().next().unwrap_or(c)
    }
}

pub fn confirm_choice(question: String, default_choice: bool) -> bool {
    // {choice} (y/n)
    println!(
        "{} ({}/{})",
        question,
        uppercase_if_true('y', default_choice),
        uppercase_if_true('n', !default_choice)
    );

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Read input");
    input = input.trim().to_string();

    if let Some(response) = input.get(0..1) {
        return response.to_lowercase() == "y";
    }

    default_choice
}

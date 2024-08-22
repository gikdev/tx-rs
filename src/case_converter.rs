/// Converts a string to PascalCase.
#[allow(dead_code)]
pub fn kebab_to_pascal(input: &str) -> String {
    input
        .split_whitespace() // Split the input by whitespace
        .map(|word| {
            // Capitalize the first letter and make the rest lowercase
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>() // Collect the capitalized words into a vector
        .join("") // Join them without spaces
}

/// Converts a string to kebab-case.
pub fn pascal_to_kebab(input: &str) -> String {
    let mut kebab_case = String::new();

    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            kebab_case.push('-'); // Add a hyphen before uppercase letters (except the first character)
        }
        kebab_case.push(c.to_lowercase().next().unwrap()); // Convert to lowercase and add to the result
    }

    kebab_case
}

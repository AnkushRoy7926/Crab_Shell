use std::env;

// Expand environment variables in the command line
// This function checks if a token starts with '$' and replaces it with the corresponding environment variable value.
// If the variable is not found, it returns an empty string.
fn expand_variables(token: &str) -> String {

    // Check if the token starts with '$'
    if token.starts_with('$') {

        // If it does, we remove the '$' and get the variable name
        let var = &token[1..];

        // Check if the variable name is empty
        if var.is_empty() {
            return token.to_string(); // Return the original token if the variable name is empty
        }

        // Get the value of the environment variable
        env::var(var).unwrap_or_else(|_| String::new())

    } else {

        // If the token does not start with '$', we return it as is
        token.to_string()
    }
}

/// Splits a command line into tokens, handling simple whitespace separation.
pub fn parse_command(line: &str) -> Vec<String> {
    line
        .split_whitespace() // Split by whitespace
        .map(expand_variables) // Convert to String
        .collect() // Collect into a Vec<String>
}


fn main() {
    let name = "Rustacean";
    println!("{}", greet(name));
}

/// A function that generates a greeting message.
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let result = greet("Rustacean");
        assert_eq!(result, "Hello, Rustacean!");
    }

    #[test]
    fn test_greet_empty_name() {
        let result = greet("");
        assert_eq!(result, "Hello, !");
    }
}


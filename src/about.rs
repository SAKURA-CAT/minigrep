/// Prints a greeting message
pub fn run() {
    println!("Hey, I'm minigrep created by {}!", env!("CARGO_PKG_AUTHORS"));
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn test_env_author() {
        assert_eq!(env!("CARGO_PKG_AUTHORS"), "Cunyue");
    }
}
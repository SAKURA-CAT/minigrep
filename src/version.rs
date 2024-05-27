pub fn run() {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn test_env_version() {
        assert!(env!("CARGO_PKG_VERSION").len() > 0);
    }
}

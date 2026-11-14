fn greeting() -> &'static str {
    "Hello, PEPPEC!"
}

fn main() {
    println!("{}", greeting());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_peppec() {
        assert_eq!(greeting(), "Hello, PEPPEC!");
    }
}

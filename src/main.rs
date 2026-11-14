fn greeting() -> &'static str {
    "Hello, PEPPEC!"
}

fn farewell() -> &'static str {
    "Goodbye, PEPPEC!"
}

fn main() {
    println!("{}", greeting());
    println!("{}", farewell());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_peppec() {
        assert_eq!(greeting(), "Hello, PEPPEC!");
    }

    #[test]
    fn farewells_peppec() {
        assert_eq!(farewell(), "Goodbye, PEPPEC!");
    }
}

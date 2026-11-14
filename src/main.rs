fn greeting() -> &'static str {
    "Hello, PEPPEC!"
}

fn farewell() -> &'static str {
    "Bye!"
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
    fn says_bye_after_the_greeting() {
        assert_eq!(farewell(), "Bye!");
    }
}

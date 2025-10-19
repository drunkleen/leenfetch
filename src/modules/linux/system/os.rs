pub fn get_os() -> String {
    String::from("Linux")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_linux_literal() {
        assert_eq!(get_os(), "Linux");
    }
}

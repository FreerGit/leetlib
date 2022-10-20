pub fn is_palindrome(s: String) -> bool {
    let lower_alpha: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    lower_alpha == lower_alpha.chars().rev().collect::<String>()
}

#[cfg(test)]
mod test {
    use crate::lc_0125_valid_palindrome::is_palindrome;

    #[test]
    fn ex1() {
        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(is_palindrome("race a car".to_string()), false);
        assert_eq!(is_palindrome(" ".to_string()), true)
    }
}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut map = std::collections::HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
    map.into_values().all(|v| v == 0)
}

#[cfg(test)]
mod test {
    use super::is_anagram;

    #[test]
    fn ex1() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
        assert_eq!(is_anagram("ccac".to_string(), "aacc".to_string()), false);
    }
}

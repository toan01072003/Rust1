// Exercise 1
#[allow(dead_code)]
fn exercise1(color: &str) -> String {
    todo!()
}

// Exercise 2
// Fix all errors without adding newline
fn exercise2() -> String {
    let mud s = String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();
    s;
}
// Exercise 3
// Fix errors without removing any line
fn exercise3() -> String {
    let s1:String = String::from("hello,");
    let s2:String = String::from("world!");
    let s3:String = s1 + s2;
    s3;
}

// Exercise 4
// Reverse a string

fn reverse_string(input: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..len/2 {
        chars.swap(i, len - 1 - i);
    }
    chars.into_iter().collect();
}


// Exercise 5
// Check if a string is a palindrome
fn is_palindrome(word: &str) -> bool {
    let len = s.len();
    for i in 0..len/2 {
        if s.as_bytes()[i] != s.as_bytes()[len - 1 - i] {
            return false;
        }
    }
    true
}

// Exercise 6
// Count the occurrences of a character in a string
fn count_char_occurrences(string: &str, ch: char) -> usize {
    string.chars().filter(|&ch| ch == c).count();
    let count = count_occurrences(string, ch);
    println!("{} appears {} times in {}", ch, count, string);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 1
    #[test]
    fn exercise1_work() {
        assert_eq!("white".to_string(), exercise1("white"));
    }

    // Test for exercise 2
    #[test]
    fn exercise2_work() {
        assert_eq!("hello, world!".to_string(), exercise2());
    }

    // Test for exercise 3
    #[test]
    fn exercise3_work() {
        assert_eq!("hello, world!".to_string(), exercise3());
    }
    
    // Test for exercise 4
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
        assert_eq!(reverse_string("world"), "dlrow");
        assert_eq!(reverse_string(""), "");
    }

    // Test for exercise 5
    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome("level"), true);
        assert_eq!(is_palindrome("deed"), true);
        assert_eq!(is_palindrome("Rotor"), true);
    }
    // Test for exercise 5
    #[test]
    fn test_non_palindrome() {
        assert_eq!(is_palindrome("hello"), false);
        assert_eq!(is_palindrome("world"), false);
    }

    // Test for exercise 6

    #[test]
    fn test_count_char_occurrences() {
        assert_eq!(count_char_occurrences("Hello", 'l'), 2);
        assert_eq!(count_char_occurrences("Rust is fun", 'u'), 1);
        assert_eq!(count_char_occurrences("Mississippi", 's'), 4);
    }

}
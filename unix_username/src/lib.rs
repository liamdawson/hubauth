fn is_underscore(b: &u8) -> bool {
    *b == 95
}

fn is_lowercase_char(b: &u8) -> bool {
    *b > 96 && *b < 123
}

fn is_period(b: &u8) -> bool {
    *b == 46
}

fn is_digit_char(b: &u8) -> bool {
    *b > 47 && *b < 58
}

fn initial_username_byte(b: &u8) -> bool {
    is_underscore(b) || is_lowercase_char(b)
}

fn further_username_byte(b: &u8) -> bool {
    is_underscore(b) || is_lowercase_char(b) || is_period(b) || is_digit_char(b)
}

pub fn validate_username(name: &[u8]) -> bool {
    // length validation
    if name.len() > 0 && name.len() < 32 {
        // validate characters
        name.iter().enumerate().all(|(i, b)| if i == 0 {
            initial_username_byte(b)
        } else {
            further_username_byte(b)
        })
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::validate_username;

    #[test]
    fn it_successfully_parses_a_single_character_username() {
        assert!(validate_username("a".as_bytes()));
    }

    #[test]
    fn it_successfully_parses_a_two_character_username() {
        assert!(validate_username("aa".as_bytes()));
    }

    #[test]
    fn it_rejects_leading_extended_characters() {
        assert!(!validate_username(".username".as_bytes()));
        assert!(!validate_username("1username".as_bytes()));
    }

    #[test]
    fn it_rejects_invalid_length_usernames() {
        assert!(!validate_username("".as_bytes()));
        assert!(!validate_username("thisismorethanthirtytwocharacters".as_bytes()));
    }

    #[test]
    fn it_rejects_uppercase_letters() {
        assert!(!validate_username("userA".as_bytes()));
        assert!(!validate_username("Auser".as_bytes()));
    }

    #[test]
    fn accepts_some_use_cases() {
        assert!(validate_username("_1234567890".as_bytes()));
        assert!(validate_username("_.helloworld".as_bytes()));
    }
}

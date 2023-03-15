pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut digits = 0;
    cc_number.chars().rev().filter(|&digit| digit != ' ')
        .filter_map(|digit| digit.to_digit(10))
        .enumerate()
        .for_each(|(i, number)| {
            sum += if i % 2 == 1 {
                let double = number * 2;
                double / 10 + double % 10
            } else { number };
            digits += 1;
        });

    return if digits < 2 {
        false
    } else {
        sum % 10 == 0
    };
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

// PREFIX MATCHES EXERCISE

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefix_pieces = prefix.split('/');
    let requests_path = request_path.split('/')
        .map(|p| Some(p))
        .chain(std::iter::once(None));

    for (prefix, request_path) in prefix_pieces.zip(requests_path) {
        match request_path {
            Some(request_path) if (prefix != "*") && (request_path != prefix) => return false,
            None => return false,
            Some(_) => {}
        }
    }
    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books",
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books",
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1",
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor",
    ));
}

use crate::token::{Token, TokenKind};

/// Parse numbers (both integers and floats).
///
/// The regex for a Hjson number looks like:
/// `-?(0|[1-9]\d*)([.]\d+([eE][+-]\d+)?)?`.
///
/// In other words:
///
/// 1. They may have a leading `-`, but not a leading `+`.
/// 2. They may be zero, but not have leading zeroes (e.g. `0123`).
/// 3. They may have decimal components.
/// 4. The decimal component may have an exponent (starting with `e` or `E`).
/// 5. The exponent may specify `+` or `-`.
/// 6. The exponent _must_ specify some digits.
pub fn parse(mut input: &str) -> Option<Token> {
    let mut len = 0;
    let mut kind = TokenKind::Integer;

    // Take an optional leading minus.
    if input.starts_with('-') {
        len += 1;
        input = &input[1..];
    }

    // Take _either_ a zero or many digits.
    // Hjson does not allow a leading digit (i.e. 0123), so if you give 0 then
    // that has to be the full number.
    if input.starts_with('0') {
        len += 1;
        input = &input[1..];
    } else {
        match input
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(input.len())
        {
            0 => return None,
            x => {
                len += x;
                input = &input[x..];
            }
        }
    }

    // Check for and take any decimal part, converting to `Float` if found.
    if input.starts_with('.') {
        let non_digit = input[1..]
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(input[1..].len());

        if non_digit > 0 {
            len += 1 + non_digit;
            kind = TokenKind::Float;
            input = &input[1 + non_digit..];
        }
    }

    // Check for and accept an exponent part.
    if input.starts_with(['e', 'E']) {
        let mut exp_len = 1;

        if input[exp_len..].starts_with(['+', '-']) {
            exp_len += 1;
        }

        let non_digit = input[exp_len..]
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(input[exp_len..].len());

        if non_digit > 0 {
            len += exp_len + non_digit;
            kind = TokenKind::Float;
            input = &input[exp_len + non_digit..];
        } else {
            // The whole number is invalid if it's missing digits after `[eE]`.
            return None;
        }
    }

    // Numbers must be terminated by one of the characters that cannot
    // appear in an unquoted string (or a newline), otherwise it could be
    // an unquoted string that started with a digit.
    // We strip whitespace first (except for newlines).
    let term_symbols = [',', ':', '[', ']', '{', '}', '\n'];
    let input = input.trim_start_matches(|c: char| c.is_whitespace() && c != '\n');
    match input.is_empty() || input.starts_with(|c: char| term_symbols.contains(&c)) {
        true => Some(kind.with_len(len)),
        false => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn float() {
        let cases = [
            "123.456",
            "-123.456",
            "0.0",
            "0.123",
            "-0.123",
            "123.456e789",
            "123.456E789",
            "123.456e+789",
            "123.456E+789",
            "123.456e-789",
            "123.456E-789",
            "123e456",
            "123E456",
            "123e+456",
            "123E+456",
            "123e-456",
            "123E-456",
        ];

        for case in cases {
            assert_eq!(parse(case), Some(TokenKind::Float.with_len(case.len())));
        }
    }

    #[test]
    fn integer() {
        let cases = ["0", "-0", "123", "-123"];

        for case in cases {
            assert_eq!(parse(case), Some(TokenKind::Integer.with_len(case.len())));
        }
    }

    #[test]
    fn invalid() {
        let bad_cases = [
            // No digits.
            "-",
            // Leading `+`.
            "+123",
            // Leading `0`.
            "0123",
            "-0123",
            // decimal point missing either one of the sides.
            "123.",
            "0.",
            "-0.",
            ".123",
            "-.123",
            "123.e123",
            "123.E123",
            // Exponent without digits.
            "123e",
            "123E",
            "123.123e",
            "123.123E",
            "123.123e+",
            "123.123E+",
            "123.123e-",
            "123.123E-",
        ];

        for case in bad_cases {
            assert_eq!(parse(case), None);
        }
    }

    #[test]
    fn terminate() {
        assert!(parse("5 ").is_some());
        assert!(parse("5}").is_some());
        assert!(parse("5 }").is_some());
        assert!(parse("5  \t}").is_some());
    }
}

use super::{Parse, Token, TokenKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Number;

impl Parse for Number {
    fn parse(mut input: &str) -> Option<Token> {
        let mut len = 0;
        let mut kind = TokenKind::Integer;

        if input.starts_with('-') {
            len += 1;
            input = &input[1..];
        }

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
            }
        }

        // Numbers must be terminated by one of the characters that cannot
        // appear in an unquoted string (or a newline), otherwise it could be
        // an unquoted string that started with a digit.
        // We strip whitespace first (except for newlines).
        let term_symbols = [',', ':', '[', ']', '{', '}', '\n'];
        let input = input.trim_start_matches(|c: char| c.is_whitespace() && c != '\n');
        match input.is_empty() || input.starts_with(|c: char| term_symbols.contains(&c)) {
            true => Some(Token::new(kind, len)),
            false => None,
        }
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
            assert_eq!(
                Number::parse(case),
                Some(Token::new(TokenKind::Float, case.len()))
            );
        }

        let partial_cases = [
            "123.123e",
            "123.123E",
            "123.123+",
            "123.123-",
            "123.123e+",
            "123.123E+",
            "123.123e-",
            "123.123E-",
        ];

        for case in partial_cases {
            assert_eq!(Number::parse(case), None);
        }
    }

    #[test]
    fn integer() {
        let cases = ["0", "-0", "123", "-123"];

        for case in cases {
            assert_eq!(
                Number::parse(case),
                Some(Token::new(TokenKind::Integer, case.len()))
            );
        }

        let partial_cases = ["0123", "123e", "123E", "123.", "123.e", "123.E"];
        for case in partial_cases {
            assert_eq!(Number::parse(case), None);
        }
    }

    #[test]
    fn invalid() {
        let bad_cases = ["-", "+123", ".123", "-.123"];

        for case in bad_cases {
            assert_eq!(Number::parse(case), None);
        }
    }

    #[test]
    fn terminate() {
        assert!(Number::parse("5 ").is_some());
        assert!(Number::parse("5}").is_some());
        assert!(Number::parse("5 }").is_some());
        assert!(Number::parse("5  \t}").is_some());
    }
}

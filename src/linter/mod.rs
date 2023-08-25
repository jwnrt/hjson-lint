mod config;

use std::fmt::{self, Display};

use crate::lexer::{Cursor, Span, TokenKind};
use crate::parser::ast::{Array, ArrayMember, Map, MapMember, Node, Value};
use crate::parser::{ParseError, Parser};

pub use self::config::Config;
use self::config::{AllowDeny, AllowDenyRequire};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lint {
    kind: LintKind,
    span: LintSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LintSpan {
    start: Cursor,
    len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LintKind {
    ImplicitBraces,
    TrailingWhitespace,
}

impl Display for LintKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LintKind::ImplicitBraces => f.write_str("implicit braces"),
            LintKind::TrailingWhitespace => f.write_str("trailing whitespace"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Linter {
    config: Config,
    lints: Vec<Lint>,
}

impl Linter {
    pub fn lint(config: Config, input: &str) -> Result<Vec<Lint>, ParseError> {
        let mut linter = Linter {
            config,
            lints: Vec::new(),
        };

        let ast = Parser::parse(input)?;
        linter.lint_root(&ast);

        Ok(linter.lints)
    }

    fn lint_root(&mut self, map: &Map) {
        self.lint_root_braces(map);
        self.lint_map(map);
    }

    fn lint_map(&mut self, map: &Map) {
        self.lint_trailing_whitespace(&map.open_brace);
        self.lint_trailing_whitespace(&map.close_brace);

        for map_member in map.members.iter() {
            self.lint_map_member(map_member);
        }
    }

    fn lint_map_member(&mut self, map_member: &Node<MapMember>) {
        self.lint_trailing_whitespace(map_member);
        self.lint_trailing_whitespace(&map_member.inner.comma);
        self.lint_value(&map_member.inner.value);
    }

    fn lint_array(&mut self, array: &Array) {
        for array_member in array.members.iter() {
            self.lint_array_member(array_member);
        }
    }

    fn lint_array_member(&mut self, array_member: &Node<ArrayMember>) {
        self.lint_trailing_whitespace(array_member);
        self.lint_trailing_whitespace(&array_member.inner.comma);
        self.lint_value(&array_member.inner.value);
    }

    fn lint_value(&mut self, value: &Value) {
        let _value = match value {
            Value::Map(map) => return self.lint_map(map),
            Value::Array(array) => return self.lint_array(array),
            Value::Value(value) => value,
        };
    }

    fn lint_trailing_whitespace<T>(&mut self, node: &Node<T>) {
        if self.config.trailing_whitespace == AllowDeny::Allow {
            return;
        }

        let mut trailing_whitespace = |tokens: &Vec<Span>| {
            // Span of the current run of whitespace we're looking at.
            let mut whitespace = None;

            // Scan tokens for whitespace followed by a newline.
            for token in tokens {
                match token.kind {
                    // Whitespace starts or extends the span.
                    TokenKind::Whitespace => {
                        whitespace
                            .get_or_insert(LintSpan {
                                start: token.start,
                                len: 0,
                            })
                            .len += token.len;
                    }
                    // New lines and EOLs publish a lint and reset the span.
                    TokenKind::NewLine | TokenKind::Eof => {
                        if let Some(span) = whitespace {
                            self.lints.push(Lint {
                                kind: LintKind::TrailingWhitespace,
                                span,
                            });
                        }
                        whitespace = None
                    }
                    // Anything else (comments) resets the span.
                    _ => whitespace = None,
                }
            }
        };

        trailing_whitespace(&node.before);
        trailing_whitespace(&node.after);
    }

    fn lint_root_braces(&mut self, map: &Map) {
        match self.config.root_braces {
            AllowDenyRequire::Deny => {
                if let Some(ref brace) = map.open_brace.inner {
                    self.lints.push(Lint {
                        kind: LintKind::ImplicitBraces,
                        span: LintSpan {
                            start: brace.start,
                            len: brace.len,
                        },
                    });
                }
            }
            AllowDenyRequire::Require if map.open_brace.inner.is_none() => {
                let cursor = map
                    .open_brace
                    .before
                    .last()
                    .map_or(Cursor::default(), |span| {
                        let newline = span.kind == TokenKind::NewLine;
                        Cursor {
                            line: span.start.line + if newline { 1 } else { 0 },
                            column: if newline {
                                1
                            } else {
                                span.start.column + span.len
                            },
                            byte_offset: span.start.byte_offset + span.len,
                        }
                    });

                self.lints.push(Lint {
                    kind: LintKind::ImplicitBraces,
                    span: LintSpan {
                        start: cursor,
                        len: 0,
                    },
                });
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn allow_trailing_whitespace() {
        let conf = Config {
            trailing_whitespace: AllowDeny::Allow,
            ..Default::default()
        };

        assert!(Linter::lint(conf, "'foo': 3").unwrap().is_empty());
        assert!(Linter::lint(conf, "'foo': 3  \t").unwrap().is_empty());
    }

    #[test]
    fn deny_trailing_whitespace() {
        let conf = Config {
            trailing_whitespace: AllowDeny::Deny,
            ..Default::default()
        };

        // No trailing whitespace.
        assert_eq!(Linter::lint(conf, "'foo': 3").unwrap(), Vec::new());
        // New lines don't count as trailing whitespace
        assert_eq!(
            Linter::lint(conf, "'foo': 3\n'bar': 5").unwrap(),
            Vec::new()
        );
        // Trailing whitespace terminated by EOF.
        assert_eq!(
            Linter::lint(conf, "'foo': 3  \t").unwrap(),
            Vec::from([Lint {
                kind: LintKind::TrailingWhitespace,
                span: LintSpan {
                    start: Cursor {
                        line: 1,
                        column: 9,
                        byte_offset: 8
                    },
                    len: 3,
                }
            }])
        );
        // Trailing whitespace terminated by new line.
        assert_eq!(
            Linter::lint(conf, "'foo': 3  \t\n'bar': 5").unwrap(),
            Vec::from([Lint {
                kind: LintKind::TrailingWhitespace,
                span: LintSpan {
                    start: Cursor {
                        line: 1,
                        column: 9,
                        byte_offset: 8
                    },
                    len: 3,
                }
            }])
        );
        // Not trailing whitespace if it's closed by the map on the same line.
        assert_eq!(Linter::lint(conf, "{ 'foo': 3  \t}").unwrap(), Vec::new());
    }

    #[test]
    fn allow_root_braces() {
        let conf = Config {
            root_braces: AllowDenyRequire::Allow,
            ..Default::default()
        };

        assert!(Linter::lint(conf, "{ 'foo': 3 }").unwrap().is_empty());
        assert!(Linter::lint(conf, "'foo': 3").unwrap().is_empty());
    }

    #[test]
    fn deny_root_braces() {
        let conf = Config {
            root_braces: AllowDenyRequire::Deny,
            ..Default::default()
        };

        assert_eq!(Linter::lint(conf, "'foo': 3").unwrap(), Vec::new());
        assert_eq!(
            Linter::lint(conf, "{ 'foo': 3 }").unwrap(),
            Vec::from([Lint {
                kind: LintKind::ImplicitBraces,
                span: LintSpan {
                    start: Cursor {
                        line: 1,
                        column: 1,
                        byte_offset: 0
                    },
                    len: 1,
                }
            }])
        );
    }

    #[test]
    fn require_root_braces() {
        let conf = Config {
            root_braces: AllowDenyRequire::Require,
            ..Default::default()
        };

        assert_eq!(Linter::lint(conf, "{ 'foo': 3 }").unwrap(), Vec::new());
        assert_eq!(
            Linter::lint(conf, "'foo': 3").unwrap(),
            Vec::from([Lint {
                kind: LintKind::ImplicitBraces,
                span: LintSpan {
                    start: Cursor {
                        line: 1,
                        column: 1,
                        byte_offset: 0
                    },
                    len: 0,
                }
            }])
        );
    }
}

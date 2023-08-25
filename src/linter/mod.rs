mod config;

use std::fmt::{self, Display};

use crate::lexer::Cursor;
use crate::parser::ast::{Array, ArrayMember, Map, MapMember, Node, Value};
use crate::parser::{ParseError, Parser};

pub use self::config::Config;

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
pub enum LintKind {}

impl Display for LintKind {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
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
        self.lint_map(map);
    }

    fn lint_map(&mut self, map: &Map) {
        for map_member in map.members.iter() {
            self.lint_map_member(map_member);
        }
    }

    fn lint_map_member(&mut self, map_member: &Node<MapMember>) {
        self.lint_value(&map_member.inner.value);
    }

    fn lint_array(&mut self, array: &Array) {
        for array_member in array.members.iter() {
            self.lint_array_member(array_member);
        }
    }

    fn lint_array_member(&mut self, array_member: &Node<ArrayMember>) {
        self.lint_value(&array_member.inner.value);
    }

    fn lint_value(&mut self, value: &Value) {
        let _value = match value {
            Value::Map(map) => return self.lint_map(map),
            Value::Array(array) => return self.lint_array(array),
            Value::Value(value) => value,
        };
    }
}

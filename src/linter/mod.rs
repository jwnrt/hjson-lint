use crate::parser::ast::{Array, ArrayMember, Map, MapMember, Node, Value};
use crate::parser::{ParseError, Parser};

#[derive(Clone, Debug, Default)]
pub struct Linter;

impl Linter {
    pub fn lint(input: &str) -> Result<(), ParseError> {
        let mut linter = Linter::default();

        let ast = Parser::parse(input)?;
        linter.lint_root(&ast);

        Ok(())
    }

    fn lint_root(&mut self, map: &Map) {
        self.lint_map(map);
    }

    fn lint_map(&mut self, map: &Map) {
        for map_member in &map.members {
            self.lint_map_member(map_member);
        }
    }

    fn lint_map_member(&mut self, map_member: &Node<MapMember>) {
        self.lint_value(&map_member.inner.value);
    }

    fn lint_array(&mut self, array: &Array) {
        for array_member in &array.members {
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

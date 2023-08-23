use crate::lexer::Token;

#[derive(Clone, Debug)]
pub struct Node<T> {
    _before: Vec<Token>,
    _inner: T,
    _after: Vec<Token>,
}

impl<T> Node<T> {
    pub fn new(before: Vec<Token>, inner: T, after: Vec<Token>) -> Self {
        Self {
            _before: before,
            _inner: inner,
            _after: after,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    pub open_brace: Option<Node<Token>>,
    pub members: Vec<Node<MapMember>>,
    pub close_brace: Option<Node<Token>>,
}

#[derive(Clone, Debug)]
pub struct MapMember {
    pub key: Token,
    pub colon: Node<Token>,
    pub value: Value,
    pub comma: Option<Node<Token>>,
}

#[derive(Clone, Debug)]
pub struct Array {
    pub open_bracket: Node<Token>,
    pub members: Vec<Node<ArrayMember>>,
    pub close_bracket: Node<Token>,
}

#[derive(Clone, Debug)]
pub struct ArrayMember {
    pub value: Value,
    pub comma: Option<Node<Token>>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Map(Map),
    Array(Array),
    Value(Token),
}

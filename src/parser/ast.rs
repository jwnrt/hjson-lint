use crate::lexer::Span;

#[derive(Clone, Debug)]
pub struct Node<T> {
    _before: Vec<Span>,
    pub inner: T,
    _after: Vec<Span>,
}

impl<T> Node<T> {
    pub fn new(before: Vec<Span>, inner: T, after: Vec<Span>) -> Self {
        Self {
            _before: before,
            inner,
            _after: after,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    pub open_brace: Node<Option<Span>>,
    pub members: Vec<Node<MapMember>>,
    pub close_brace: Node<Option<Span>>,
}

#[derive(Clone, Debug)]
pub struct MapMember {
    pub key: Span,
    pub colon: Node<Span>,
    pub value: Value,
    pub comma: Node<Option<Span>>,
}

#[derive(Clone, Debug)]
pub struct Array {
    pub open_bracket: Node<Span>,
    pub members: Vec<Node<ArrayMember>>,
    pub close_bracket: Node<Span>,
}

#[derive(Clone, Debug)]
pub struct ArrayMember {
    pub value: Value,
    pub comma: Node<Option<Span>>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Map(Map),
    Array(Array),
    Value(Span),
}

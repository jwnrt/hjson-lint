use crate::parser::Event;
use crate::token::Token;

/// Kinds of tree that make up the parsed structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeKind {
    /// Trees which were unsuccessfully parsed.
    ErrorTree,
    /// The whole file, including decorations like comments.
    File,
    /// A map which may or may not contain surrounding braces.
    Map,
    /// A single mapping (`key: value`) in a map.
    Mapping,
    /// An array of values.
    Array,
}

/// A tree in the parsed structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tree {
    kind: TreeKind,
    children: Vec<Child>,
}

/// A child of a tree in the parsed structure, which may be a single token or
/// another tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Child {
    Token(Token),
    Tree(Tree),
}

impl Tree {
    /// Construct a tree from some stream of [`Event`]s. See [`parser::Parser`]
    /// for generating an event stream.
    pub(crate) fn build(mut events: Vec<Event>) -> Self {
        let mut stack = Vec::new();

        // We want a tree left over on the stack at the end of construction so
        // we can return it. Remove the last `Close` event to prevent it being
        // dropped (or added to some parent tree that doesn't exist).
        assert!(matches!(events.pop(), Some(Event::Close)));

        // Push, pop, and add to the stack of trees based on each event.
        for event in events {
            match event {
                Event::Open { kind } => stack.push(Tree {
                    kind,
                    children: Vec::new(),
                }),

                Event::Close => {
                    let tree = stack.pop().unwrap();
                    stack.last_mut().unwrap().children.push(Child::Tree(tree));
                }

                Event::Advance { token } => {
                    stack.last_mut().unwrap().children.push(Child::Token(token));
                }
            }
        }

        // The last thing on the stack should be the tree of the whole file.
        assert!(stack.len() == 1);
        stack.pop().unwrap()
    }
}

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

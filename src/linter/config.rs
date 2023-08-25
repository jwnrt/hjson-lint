/// Configuration for which linting rules to apply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    /// Whether to allow or deny trailing whitespace at the ends of lines.
    pub trailing_whitespace: AllowDeny,

    /// Whether to allow, deny, or require that braces `{ ... }` surround
    /// the whole document.
    pub root_braces: AllowDenyRequire,

    /// Whether to allow or deny omitting commas at the ends of lines.
    pub missing_commas: AllowDeny,

    /// Whether to allow, deny, or require adding trailing commas to the
    /// final member of a map or array.
    pub trailing_commas: AllowDenyRequire,

    /// Whether to allow, deny, or require (where permitted) that string
    /// values are unquoted.
    pub unquoted_values: AllowDenyRequire,

    /// Whether to allow, deny, or require (where permitted) that map keys
    /// are unquoted.
    pub unquoted_keys: AllowDenyRequire,
}

impl Default for Config {
    /// By default, we `Allow` all Hjson features and only deny obvious
    /// style issues (e.g. trailing whitespace).
    fn default() -> Self {
        Self {
            trailing_whitespace: AllowDeny::Deny,
            root_braces: AllowDenyRequire::Allow,
            missing_commas: AllowDeny::Allow,
            trailing_commas: AllowDenyRequire::Allow,
            unquoted_values: AllowDenyRequire::Allow,
            unquoted_keys: AllowDenyRequire::Allow,
        }
    }
}

impl Config {
    /// Strict configuration which reflects vanilla JSON syntax.
    pub fn strict() -> Self {
        Self {
            trailing_whitespace: AllowDeny::Deny,
            root_braces: AllowDenyRequire::Require,
            missing_commas: AllowDeny::Deny,
            trailing_commas: AllowDenyRequire::Deny,
            unquoted_values: AllowDenyRequire::Deny,
            unquoted_keys: AllowDenyRequire::Deny,
        }
    }
}

/// States for allowing or denying some rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllowDeny {
    Allow,
    Deny,
}

/// States for allowing, denying, or requiring some rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllowDenyRequire {
    Require,
    Allow,
    Deny,
}

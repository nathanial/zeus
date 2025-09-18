//! Semantic description for Zeus programs.

#[derive(Clone, Debug)]
pub enum InferenceMode {
    /// Types are inferred guided by annotations and runtime feedback.
    Directed,
    /// Types rely entirely on global constraint solving.
    Global,
}

#[derive(Clone, Debug)]
pub struct TypeDescriptor {
    pub name: String,
    pub kind: Kind,
}

#[derive(Clone, Debug)]
pub enum Kind {
    Star,
    Function(Box<Kind>, Box<Kind>),
}

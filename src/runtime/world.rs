//! Representation of the live Zeus world graph.

#[derive(Clone, Debug)]
pub struct WorldSnapshot {
    pub modules: Vec<String>,
    pub focus: Option<String>,
}

pub struct World {
    modules: Vec<String>,
    focus: Option<String>,
}

impl World {
    /// Constructs a seed world containing the standard library shell.
    pub fn bootstrap() -> Self {
        World {
            modules: vec!["Prelude".into(), "Temporal".into(), "Reflect".into()],
            focus: Some("Prelude".into()),
        }
    }

    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    pub fn snapshot(&self) -> WorldSnapshot {
        WorldSnapshot {
            modules: self.modules.clone(),
            focus: self.focus.clone(),
        }
    }
}

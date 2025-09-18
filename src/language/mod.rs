//! Language-level structures describing Zeus source code and semantics.

pub mod semantics;
pub mod syntax;

use semantics::InferenceMode;
use syntax::Module;

pub struct LanguageSpec {
    pub version: String,
    pub inference: InferenceMode,
    pub prelude: Module,
}

impl LanguageSpec {
    pub fn prototype() -> Self {
        LanguageSpec {
            version: "0.0.1-prototype".into(),
            inference: InferenceMode::Directed,
            prelude: Module::prelude(),
        }
    }
}

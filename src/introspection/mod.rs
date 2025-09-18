//! Introspection tools that mirror the Smalltalk-style live development experience.

use crate::runtime::ZeusRuntime;
use crate::runtime::world::WorldSnapshot;

#[derive(Debug)]
pub struct Workspace {
    snapshot: WorldSnapshot,
}

impl Workspace {
    pub fn from_runtime(runtime: &ZeusRuntime) -> Self {
        Workspace {
            snapshot: runtime.snapshot(),
        }
    }

    pub fn describe(&self) -> String {
        let focus = self.snapshot.focus.as_deref().unwrap_or("no active module");

        format!("{} in focus ({})", focus, self.snapshot.modules.join(", "))
    }
}

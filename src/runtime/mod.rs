//! Runtime system primitives for Zeus.

pub mod world;

use world::{World, WorldSnapshot};

/// Central coordinator for executing Zeus programs and managing their live state.
pub struct ZeusRuntime {
    world: World,
    tick_counter: usize,
}

impl ZeusRuntime {
    /// Creates a runtime with a seed world that can be evolved.
    pub fn bootstrap() -> Self {
        ZeusRuntime {
            world: World::bootstrap(),
            tick_counter: 0,
        }
    }

    /// Returns a human-friendly summary of the runtime state.
    pub fn status_line(&self) -> String {
        format!(
            "{} modules · tick {}",
            self.world.module_count(),
            self.tick_counter
        )
    }

    /// Produces a snapshot of the live world without mutating it.
    pub fn snapshot(&self) -> WorldSnapshot {
        self.world.snapshot()
    }

    /// Advances the runtime clock; placeholder for future scheduling logic.
    pub fn tick(&mut self) {
        self.tick_counter += 1;
    }
}

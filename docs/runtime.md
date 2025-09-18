# Zeus Runtime

The Zeus runtime hosts a living world of modules, values, and processes.
Instead of compiling to a static artifact, source files stream directly
into the runtime where they become part of an inspectable graph.

## Layers

1. **World graph** – tracks modules, bindings, and their dependencies.
   Every change results in a structural diff that can be replayed.
2. **Scheduler** – coordinates lightweight processes and reacts to
   external events. Laziness is preserved by default; explicit effect
   nodes opt into interaction with the host environment.
3. **Introspection fabric** – mirrors the world graph through lenses and
   workspaces. Tooling connects to the fabric to provide inspectors,
   time-travel debuggers, or collaborative sessions.

## Boot Sequence

1. Load the standard prelude module and effect definitions.
2. Restore user modules from the latest snapshot.
3. Expose an exploration workspace so developers can poke around the
   running image immediately.

## Runtime Values

All values carry **shape descriptors** recording how they were produced:
source expression, type derivation, instrumentation data, and temporal
identity. This makes it possible to see what computation led to any
piece of data and to replay or fork it interactively.

## Persistence

Snapshots are append-only journals. You can scrub through time, branch
from any checkpoint, and merge divergent timelines by replaying their
journals through the type checker.

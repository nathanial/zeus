use crate::ZeusResult;
use crate::introspection::Workspace;
use crate::runtime::ZeusRuntime;

/// Entry point for the Zeus command-line tooling.
pub fn run() -> ZeusResult<()> {
    let runtime = ZeusRuntime::bootstrap();
    let workspace = Workspace::from_runtime(&runtime);

    println!("🔥 Welcome to Zeus — a live functional playground.");
    println!("Runtime status: {}", runtime.status_line());
    println!("Active workspace: {}", workspace.describe());

    Ok(())
}

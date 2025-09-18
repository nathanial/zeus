fn main() {
    if let Err(err) = zeus::cli::run() {
        eprintln!("Zeus failed to start: {err}");
        std::process::exit(1);
    }
}

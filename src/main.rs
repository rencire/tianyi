fn main() {
    if let Err(e) = tianyi::run_args(std::env::args_os().skip(1)) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

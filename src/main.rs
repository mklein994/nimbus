extern crate nimbus;

fn main() {
    if let Err(e) = nimbus::run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

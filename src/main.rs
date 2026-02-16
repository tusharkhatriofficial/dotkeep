mod cli;
mod crypto;
mod tui;
mod vault;

fn main() {
    println!("dotkeep v{}", env!("CARGO_PKG_VERSION"));
}

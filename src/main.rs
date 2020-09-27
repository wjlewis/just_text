mod assets;
mod lib;

fn main() {
    if let Err(e) = lib::run() {
        eprintln!("{}", e);
    }
}

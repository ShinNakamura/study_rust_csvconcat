fn main() {
    if let Err(err) = csvconcat::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

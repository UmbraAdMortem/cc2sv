use ccsv::CCSV;

fn main() {
    CCSV::new(std::io::stdout(), std::io::stderr()).run();
}

use std::io::Write;

pub struct CCSV<W: Write> {
    out: W,
}

impl<W: Write> CCSV<W> {
    pub fn new(out: W) -> Self {
        CCSV { out }
    }

    pub fn run(&mut self) {
        let _ = writeln!(
            self.out,
            "Hello! \nThis is Conventional Commits 2 Sematic Verison!"
        );
    }
}

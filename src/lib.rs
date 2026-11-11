use std::io::Write;

pub struct CCSV<W: Write, E: Write> {
    out: W,
    err: E,
}

impl<W: Write, E: Write> CCSV<W, E> {
    pub fn new(out: W, err: E) -> Self {
        CCSV { out, err }
    }

    pub fn run(&mut self) {
        let _ = writeln!(
            self.out,
            "Hello! \nThis is Conventional Commits 2 Sematic Verison!"
        );

        let _ = writeln!(self.err, "There are no errors!");
    }
}

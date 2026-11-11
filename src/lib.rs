use std::io::Write;

pub struct CCSV<W: Write, E: Write> {
    out: W,
    err: E,
}

impl CCSV<std::io::Stdout, std::io::Stderr> {
    pub fn new() -> Self {
        return CCSV::new_internal(std::io::stdout(), std::io::stderr());
    }
}

impl<W: Write, E: Write> CCSV<W, E> {
    fn new_internal(out: W, err: E) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    // https://rust-cli.github.io/book/tutorial/testing.html
    #[test]
    fn test_stdout_not_empty() {
        let mut out: Vec<u8> = Vec::new();
        let mut err: Vec<u8> = Vec::new();

        CCSV::new_internal(&mut out, &mut err).run();

        assert!(!out.is_empty());
        println!("{}", String::from_utf8(out).unwrap());

        assert!(!err.is_empty());
        println!("{}", String::from_utf8(err).unwrap());
    }
}

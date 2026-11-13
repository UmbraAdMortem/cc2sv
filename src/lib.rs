use std::io::Write;

pub struct CCSV<W: Write, E: Write> {
    args: Vec<String>,

    out: W,
    err: E,
}

impl CCSV<std::io::Stdout, std::io::Stderr> {
    pub fn new() -> Self {
        return CCSV::new_internal(
            std::env::args().collect(),
            std::io::stdout(),
            std::io::stderr(),
        );
    }
}

impl<W: Write, E: Write> CCSV<W, E> {
    fn new_internal(args: Vec<String>, out: W, err: E) -> Self {
        CCSV { args, out, err }
    }

    pub fn run(&mut self) {
        let _ = writeln!(self.out, "Args:{:#?}", self.args);

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

        CCSV::new_internal(args(&[]), &mut out, &mut err).run();

        assert!(!out.is_empty());
        println!("{}", String::from_utf8(out).unwrap());

        assert!(!err.is_empty());
        println!("{}", String::from_utf8(err).unwrap());
    }

    #[test]
    fn test_help() {
        let mut out: Vec<u8> = Vec::new();
        let mut err: Vec<u8> = Vec::new();

        CCSV::new_internal(args(&["--help"]), &mut out, &mut err).run();

        assert!(!out.is_empty());
        println!("{}", String::from_utf8(out).unwrap());

        assert!(!err.is_empty());
        println!("{}", String::from_utf8(err).unwrap());
    }

    fn args(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }
}

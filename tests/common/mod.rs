use gag::BufferRedirect;
use std::io::Read;

pub fn args(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| s.to_string()).collect()
}

pub fn redirect_stdout() -> BufferRedirect {
    BufferRedirect::stdout().unwrap()
}

pub fn redirect_stderr() -> BufferRedirect {
    BufferRedirect::stderr().unwrap()
}

pub fn read_drop_buffer(stream: BufferRedirect) -> String {
    let mut out = String::new();
    let _ = stream.into_inner().read_to_string(&mut out);
    return out;
}

use std::io::{self, Write};

pub struct Output<W: Write> {
    inner: W,
}

impl<W: Write> Output<W> {
    pub fn new(inner: W) -> Self {
        Self { inner: inner }
    }

    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<'a, W: 'a + Write> Output<W> {
    pub fn from_ref(stdout: W) -> Output<W> {
        Output { inner: stdout }
    }
}

fn main() {
    let stdout = io::stdout();
    let output = Output::new(stdout);
    let _ = output.into_inner();

    let mut stdout = io::stdout();
    let output = Output::from_ref(&mut stdout);
    let _ = output.into_inner();
}